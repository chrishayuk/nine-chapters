//! Running a page: each operation done on the board, and each figure the
//! source states (`Gives`, `Answer`, `Rates`, `Count`) compared with what
//! the board holds.

use crate::board::{self, gcd_all, Board, Num, Op};
use crate::count::{self, RULES};
use crate::page::{self, ColRef, Factor, How, Kind, Line, Mixed, Pos, Stmt};
use crate::rat::Rat;
use crate::solve::{self, Solution, Step};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    /// Not in the notation.
    Notation,
    /// A figure the source states that the board does not give.
    Wrong,
    /// An operation that cannot be done as read.
    Fails,
    /// Information: how a figure is written, what a count comes to.
    Form,
}

#[derive(Clone, Debug)]
pub struct Finding {
    pub line: usize,
    pub span: (usize, usize),
    pub severity: Severity,
    pub message: String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Mark {
    Right,
    Wrong(String),
    Info(String),
}

#[derive(Clone, Debug)]
pub struct LineRun {
    pub line: Line,
    /// The reading used, when the line gives several.
    pub choice: usize,
    pub marks: Vec<Mark>,
    /// What was done, and the board after each.
    pub steps: Vec<Step>,
    pub solution: Option<Solution>,
}

#[derive(Clone, Debug)]
pub struct Run {
    pub lines: Vec<LineRun>,
    pub findings: Vec<Finding>,
    pub board: Option<Board>,
    /// Figures the source states, compared: (right, wrong).
    pub checked: (usize, usize),
    /// Every operation, in order.
    pub ops: Vec<Op>,
    /// The operation could not be done; nothing after it was run.
    pub stopped: Option<usize>,
}

impl Run {
    pub fn title(&self) -> Option<String> {
        self.lines.iter().find_map(|l| match &l.line.kind {
            Kind::Heading(h) => Some(h.clone()),
            _ => None,
        })
    }
    pub fn solution(&self) -> Option<&Solution> {
        self.lines.iter().rev().find_map(|l| l.solution.as_ref())
    }
    /// Lines with readings in the alternative, and how many each has.
    pub fn alternatives(&self) -> Vec<(usize, usize)> {
        self.lines
            .iter()
            .enumerate()
            .filter_map(|(i, l)| match &l.line.kind {
                Kind::Stmt(v) if v.len() > 1 => Some((i, v.len())),
                _ => None,
            })
            .collect()
    }
}

fn resolve(c: &ColRef, n: usize) -> Result<usize, String> {
    match c {
        ColRef::Right => Ok(0),
        ColRef::Left => Ok(n.saturating_sub(1)),
        ColRef::Middle if n == 3 => Ok(1),
        ColRef::Middle => Err(format!("`middle` is for a board of three columns; this one has {}", n)),
        ColRef::Nth(k) if *k <= n => Ok(k - 1),
        ColRef::Nth(k) => Err(format!("there is no {} column on a board of {}", board::ordinal(*k), n)),
    }
}

fn position(b: &Board, t: usize, p: &Pos) -> Result<usize, String> {
    match p {
        Pos::Head => b
            .head(t)
            .map_err(|e| board::trouble_text(b, &e))?
            .ok_or(format!("{} has nothing left above its 實", b.name(t))),
        Pos::Thing(name) => b
            .things
            .iter()
            .position(|x| x == name)
            .ok_or(format!("`{}` is not a position of this board: {}", name, b.things.join(" "))),
    }
}

pub fn show_mixed(v: Rat) -> String {
    let neg = v < Rat::ZERO;
    let a = if neg { -v } else { v };
    let w = a.floor();
    let f = a.fract();
    let s = match (w, f.is_zero()) {
        (_, true) => w.to_string(),
        (0, false) => format!("{}/{}", f.numer(), f.denom()),
        _ => format!("{} {}/{}", w, f.numer(), f.denom()),
    };
    if neg {
        format!("-{}", s)
    } else {
        s
    }
}

/// How an answer would be written under each reading of the last step.
pub struct Forms {
    /// The 法 as the board leaves it, every 實 over it.
    pub board: Option<Num>,
    /// The 法 and every 實 divided by what they have in common (約).
    pub common: Option<Num>,
}

pub fn forms(s: &Solution) -> Forms {
    let fa = match s.fa {
        Some(f) => f,
        None => return Forms { board: None, common: None },
    };
    let ints: Option<Vec<Num>> = s.shi.iter().map(|r| if r.is_int() { Some(r.numer()) } else { None }).collect();
    let common = ints.map(|v| {
        let mut all = v.clone();
        all.push(fa);
        fa.abs() / gcd_all(&all).max(1)
    });
    Forms { board: Some(fa.abs()), common }
}

/// How `v` is written over `den`, as a mixed number: `None` if it cannot be.
pub fn over(v: Rat, den: Num) -> Option<String> {
    let x = v * Rat::int(den);
    if !x.is_int() {
        return None;
    }
    let n = x.numer().abs();
    let (w, r) = (n / den, n % den);
    let s = if r == 0 {
        w.to_string()
    } else if w == 0 {
        format!("{}/{}", r, den)
    } else {
        format!("{} {}/{}", w, r, den)
    };
    Some(if x.numer() < 0 { format!("-{}", s) } else { s })
}

fn written_den(a: &[Mixed]) -> Option<Num> {
    let dens: Vec<Num> = a.iter().filter(|m| m.has_fraction()).map(|m| m.den).collect();
    if dens.is_empty() || dens.iter().any(|&d| d != dens[0]) {
        None
    } else {
        Some(dens[0])
    }
}

fn check_answer(sol: &Solution, a: &[Mixed], marks: &mut Vec<Mark>) -> Result<(), String> {
    let m = sol.values.len();
    if a.len() != m {
        return Err(format!("the answer has {} figures; the board finds {}", a.len(), m));
    }
    let wrong: Vec<String> = a
        .iter()
        .zip(&sol.values)
        .filter(|(x, v)| x.value() != **v)
        .map(|(x, v)| format!("{} for {}", show_mixed(*v), show_mixed(x.value())))
        .collect();
    if !wrong.is_empty() {
        let msg = format!("the board gives {}", wrong.join(", "));
        marks.push(Mark::Wrong(msg.clone()));
        return Err(msg);
    }
    marks.push(Mark::Right);
    // how it is written
    let fracs: Vec<&Mixed> = a.iter().filter(|x| x.has_fraction()).collect();
    if fracs.is_empty() {
        return Ok(());
    }
    let f = forms(sol);
    let lowest = a.iter().all(|x| !x.has_fraction() || x.value().fract().denom() == x.den);
    let mut fits = vec![];
    if let Some(d) = written_den(a) {
        if f.board == Some(d) {
            fits.push(format!("the 法 as the board leaves it, {}", d));
        }
        if f.common == Some(d) && f.board != Some(d) {
            fits.push(format!("the 法 and 實s divided by what they share (約), {}", d));
        }
    }
    if lowest {
        fits.push("each in its lowest terms".into());
    }
    let written = match written_den(a) {
        Some(d) => format!("over {}", d),
        None => "over several".into(),
    };
    let mut note = if fits.is_empty() {
        format!("written {}: no reading here writes it so", written)
    } else {
        format!("written {}: {}", written, fits.join("; or "))
    };
    let mut nots = vec![];
    if let Some(b) = f.board {
        if Some(b) != written_den(a) {
            if let Some(s) = over(a[0].value(), b) {
                nots.push(format!("as the board leaves it, {}", s));
            }
        }
    }
    if !lowest {
        let first = fracs[0];
        nots.push(format!("in lowest terms, {}", show_mixed(first.value())));
    }
    if !nots.is_empty() {
        note.push_str(&format!(" (not {})", nots.join(", nor ")));
    }
    marks.push(Mark::Info(note));
    Ok(())
}

/// Run a page with the first reading of every line.
pub fn run(text: &str) -> Run {
    run_with(text, &[])
}

/// Run a page with chosen readings: `(line, reading)`.
pub fn run_with(text: &str, choices: &[(usize, usize)]) -> Run {
    run_lines(page::parse(text), choices)
}

/// Run lines already read.
pub fn run_lines(lines: Vec<Line>, choices: &[(usize, usize)]) -> Run {
    // the board's size: its positions from `Things` (or the first `Set`),
    // its columns from the `Set`s.
    let mut things: Option<Vec<String>> = None;
    let mut setcols: Vec<ColRef> = vec![];
    let mut first_len = None;
    for l in &lines {
        if let Kind::Stmt(v) = &l.kind {
            for s in v {
                match s {
                    Stmt::Things(t) if things.is_none() => things = Some(t.clone()),
                    Stmt::Set(c, f) => {
                        first_len.get_or_insert(f.len());
                        if !setcols.contains(c) {
                            setcols.push(c.clone())
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    let n = setcols
        .iter()
        .map(|c| match c {
            ColRef::Nth(k) => *k,
            _ => 0,
        })
        .max()
        .unwrap_or(0)
        .max(setcols.len());
    let things = things.unwrap_or_else(|| {
        let m = first_len.unwrap_or(1);
        (1..m).map(|i| format!("#{}", i)).chain(std::iter::once("實".to_string())).collect()
    });
    let mut b = Board::new(things, n);
    let mut run = Run { lines: vec![], findings: vec![], board: None, checked: (0, 0), ops: vec![], stopped: None };
    let mut since_count = 0usize;
    let mut sol: Option<Solution> = None;
    for (i, line) in lines.into_iter().enumerate() {
        let width = line.text.chars().count();
        let mut lr = LineRun { line, choice: 0, marks: vec![], steps: vec![], solution: None };
        let span = (lr.line.text.chars().take_while(|c| c.is_whitespace()).count(), lr.line.note.unwrap_or(width));
        let find = |sev: Severity, msg: String, run: &mut Run| {
            run.findings.push(Finding { line: i, span, severity: sev, message: msg });
        };
        match &lr.line.kind {
            Kind::Bad(e) => find(Severity::Notation, e.clone(), &mut run),
            Kind::Stmt(alts) => {
                let choice = choices.iter().find(|c| c.0 == i).map(|c| c.1).unwrap_or(0).min(alts.len() - 1);
                lr.choice = choice;
                let s = alts[choice].clone();
                if run.stopped.is_some() && (s.works() || matches!(s, Stmt::Solve(_))) {
                    run.lines.push(lr);
                    continue;
                }
                let r: Result<(), String> = (|| {
                    let col = |c: &ColRef| resolve(c, n);
                    let tr = |b: &Board, e: board::Trouble| board::trouble_text(b, &e);
                    let did = |b: &Board, op: Result<Op, board::Trouble>, lr: &mut LineRun| -> Result<(), String> {
                        let op = op.map_err(|e| tr(b, e))?;
                        lr.steps.push(Step { op, board: b.clone() });
                        Ok(())
                    };
                    match &s {
                        Stmt::Things(_) | Stmt::Nothing => {}
                        Stmt::Set(c, f) => {
                            let t = col(c)?;
                            let op = b.set(t, f.clone());
                            did(&b, op, &mut lr)?;
                        }
                        Stmt::Multiply(c, f) => {
                            let t = col(c)?;
                            let by = match f {
                                Factor::N(k) => *k,
                                Factor::Of(p, a) => {
                                    let a = col(a)?;
                                    let pos = position(&b, a, p)?;
                                    b.col(a).map_err(|e| tr(&b, e))?[pos]
                                }
                            };
                            let op = b.multiply(t, by);
                            did(&b, op, &mut lr)?;
                        }
                        Stmt::Take { a, t, how } => {
                            let (a, t) = (col(a)?, col(t)?);
                            let op = match how {
                                How::Once => b.take(a, t, 1, None),
                                How::Until(p) => {
                                    let pos = position(&b, t, p)?;
                                    b.clear(a, t, pos)
                                }
                                How::AsFar(p) => {
                                    let pos = position(&b, t, p)?;
                                    b.as_far(a, t, pos)
                                }
                                How::Crosswise(p) => {
                                    let pos = position(&b, t, p)?;
                                    b.cross(a, t, pos)
                                }
                                How::InTurn(p) => {
                                    let pos = position(&b, t, p)?;
                                    let ops = b.mutual(a, t, pos).map_err(|e| tr(&b, e))?;
                                    if ops.is_empty() {
                                        return Err(format!("the {} of {} is already empty", b.thing(pos), b.name(t)));
                                    }
                                    for (op, board) in ops {
                                        lr.steps.push(Step { op, board });
                                    }
                                    return Ok(());
                                }
                            };
                            did(&b, op, &mut lr)?;
                        }
                        Stmt::Add { a, t } => {
                            let (a, t) = (col(a)?, col(t)?);
                            let op = b.take(a, t, -1, None);
                            did(&b, op, &mut lr)?;
                        }
                        Stmt::Halve(c) => {
                            let t = col(c)?;
                            let op = b.halve(t);
                            did(&b, op, &mut lr)?;
                        }
                        Stmt::Reduce(c) => {
                            let t = col(c)?;
                            let op = b.reduce(t);
                            did(&b, op, &mut lr)?;
                        }
                        Stmt::Divide(c, k) => {
                            let t = col(c)?;
                            let op = b.divide(t, *k);
                            did(&b, op, &mut lr)?;
                        }
                        Stmt::Drop(c) => {
                            let t = col(c)?;
                            let op = b.drop_col(t);
                            did(&b, op, &mut lr)?;
                        }
                        Stmt::Solve(m) => match solve::solve(&b, *m) {
                            Ok(s) => {
                                lr.steps = s.steps.clone();
                                let mut info = match s.fa {
                                    Some(fa) => format!(
                                        "法 {}; 實 {}",
                                        fa,
                                        s.shi.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" ")
                                    ),
                                    None => format!(
                                        "{}",
                                        s.values.iter().map(|x| show_mixed(*x)).collect::<Vec<_>>().join(", ")
                                    ),
                                };
                                info.push_str(&format!("; {} steps", s.steps.len()));
                                lr.marks.push(Mark::Info(info));
                                lr.solution = Some(s.clone());
                                sol = Some(s);
                            }
                            Err((e, steps)) => {
                                let at = steps.last().map(|s| s.board.clone()).unwrap_or(b.clone());
                                lr.steps = steps;
                                return Err(board::trouble_text(&at, &e));
                            }
                        },
                        Stmt::Gives(c, f) => {
                            let t = col(c)?;
                            let have = b.col(t).map_err(|e| tr(&b, e))?;
                            if have == f {
                                lr.marks.push(Mark::Right);
                            } else {
                                let show = |v: &[Num]| v.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" ");
                                lr.marks.push(Mark::Wrong(format!("the board has {}", show(have))));
                            }
                        }
                        Stmt::Answer(a) => match &sol {
                            Some(s) => {
                                let _ = check_answer(s, a, &mut lr.marks);
                            }
                            None => lr.marks.push(Mark::Info("nothing solved yet to compare it with".into())),
                        },
                        Stmt::Rates(r) => match sol.as_ref().and_then(|s| s.fa.map(|fa| (fa, s))) {
                            Some((fa, s)) => {
                                let ints: Option<Vec<Num>> =
                                    s.shi.iter().map(|x| if x.is_int() { Some(x.numer()) } else { None }).collect();
                                let mut all = vec![fa];
                                all.extend(ints.unwrap_or_default());
                                let g = gcd_all(&all).max(1) * fa.signum();
                                let have: Vec<Num> = all.iter().map(|x| x / g).collect();
                                if &have == r {
                                    lr.marks.push(Mark::Right);
                                    if g.abs() != 1 {
                                        lr.marks.push(Mark::Info(format!("the board's 法 {} and 實s divided by {}", fa, g.abs())));
                                    } else {
                                        lr.marks.push(Mark::Info(format!("the 法 {} and the 實s as the board leaves them", fa)));
                                    }
                                } else {
                                    lr.marks.push(Mark::Wrong(format!(
                                        "the board's rates are {}",
                                        have.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" ")
                                    )));
                                }
                            }
                            None => lr.marks.push(Mark::Info("nothing solved through a 法 to compare it with".into())),
                        },
                        Stmt::Count(c) => {
                            let ops = &run.ops[since_count..];
                            let got: Vec<(&str, u64)> = RULES.iter().map(|r| (r.name(), count::count(*r, ops))).collect();
                            let hit: Vec<&str> = got.iter().filter(|g| g.1 == *c).map(|g| g.0).collect();
                            let all = got.iter().map(|(n, v)| format!("{} {}", n, v)).collect::<Vec<_>>().join(", ");
                            if hit.is_empty() {
                                lr.marks.push(Mark::Info(format!("no rule counts {}: {}", c, all)));
                            } else {
                                lr.marks.push(Mark::Right);
                                lr.marks.push(Mark::Info(format!("{}: {}", hit.join(", "), all)));
                            }
                            since_count = run.ops.len();
                        }
                    }
                    Ok(())
                })();
                for s in &lr.steps {
                    run.ops.push(s.op.clone());
                }
                if let Some(last) = lr.steps.last() {
                    b = last.board.clone();
                }
                if let Err(e) = r {
                    find(Severity::Fails, e.clone(), &mut run);
                    lr.marks.push(Mark::Wrong(e));
                    run.stopped = Some(i);
                }
                for m in &lr.marks {
                    match m {
                        Mark::Right => run.checked.0 += 1,
                        Mark::Wrong(w) if !matches!(s, Stmt::Solve(_)) && run.stopped != Some(i) => {
                            run.checked.1 += 1;
                            find(Severity::Wrong, w.clone(), &mut run);
                        }
                        Mark::Info(t) if matches!(s, Stmt::Count(_) | Stmt::Answer(_)) => {
                            find(Severity::Form, t.clone(), &mut run)
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
        run.lines.push(lr);
    }
    run.board = Some(b);
    run
}

/// The page as text, each line with its marks, and the board after each
/// operation (after a run of `Set`s, once).
pub fn render(run: &Run, boards: bool) -> String {
    let mut out = String::new();
    let w = run
        .lines
        .iter()
        .map(|l| board::width(l.line.text.trim_end()))
        .filter(|&w| w <= 72)
        .max()
        .unwrap_or(40)
        + 2;
    for (k, l) in run.lines.iter().enumerate() {
        let text = l.line.text.trim_end();
        let marks: Vec<String> = l
            .marks
            .iter()
            .map(|m| match m {
                Mark::Right => "✓".to_string(),
                Mark::Wrong(s) => format!("✗ {}", s),
                Mark::Info(s) => s.clone(),
            })
            .collect();
        if marks.is_empty() {
            out.push_str(text);
        } else {
            let pad = if board::width(text) < w { w - board::width(text) } else { 2 };
            out.push_str(&format!("{}{}{}", text, " ".repeat(pad), marks.join("  ")));
        }
        out.push('\n');
        if !boards || l.steps.is_empty() {
            continue;
        }
        let next_is_set = run.lines[k + 1..]
            .iter()
            .find(|n| !matches!(n.line.kind, Kind::Blank | Kind::Note))
            .map(|n| matches!(&n.line.kind, Kind::Stmt(v) if matches!(v[n.choice], Stmt::Set(..))))
            .unwrap_or(false);
        let is_set = matches!(&l.line.kind, Kind::Stmt(v) if matches!(v[l.choice], Stmt::Set(..)));
        if is_set && next_is_set {
            continue;
        }
        let is_solve = matches!(&l.line.kind, Kind::Stmt(v) if matches!(v[l.choice], Stmt::Solve(_)));
        let shown: Vec<&Step> = if is_solve {
            l.steps.iter().filter(|s| s.op.actor.is_some() || s.op.what.starts_with("reduce")).collect()
        } else {
            l.steps.iter().collect()
        };
        for s in shown {
            if is_solve {
                out.push_str(&format!("    {}\n", s.op.what));
            }
            for row in s.board.to_string().lines() {
                out.push_str(&format!("    {}\n", row));
            }
        }
    }
    let (r, wr) = run.checked;
    if r + wr > 0 {
        out.push_str(&format!(
            "\n{} checked, {} right, {} wrong.\n",
            plural(r + wr, "figure"),
            r,
            wr
        ));
    }
    if let Some(i) = run.stopped {
        out.push_str(&format!("stopped at line {}: nothing after it was run.\n", i + 1));
    }
    out
}

pub fn plural(n: usize, w: &str) -> String {
    if n == 1 {
        format!("1 {}", w)
    } else {
        format!("{} {}s", n, w)
    }
}

/// Problems a line at a time, for `--check`.
pub fn problems(run: &Run) -> String {
    let mut out = String::new();
    for f in &run.findings {
        let sev = match f.severity {
            Severity::Notation => "notation",
            Severity::Wrong => "wrong",
            Severity::Fails => "fails",
            Severity::Form => "note",
        };
        out.push_str(&format!("line {}: {}: {}\n", f.line + 1, sev, f.message));
    }
    out
}
