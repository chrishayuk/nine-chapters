//! The reports over the whole transcription: `verify`, `answers`, `counts`.

use crate::check::{self, forms, over, show_mixed, Mark, Severity};
use crate::corpus::CORPUS;
use crate::page::{Kind, Stmt};
use crate::readings;
use crate::solve::{solve, Method, METHODS};

pub const DECISIONS: &str = "\
The choices this reconstruction makes that the text does not fix.

1. The board. A column is a list of signed whole numbers, top to bottom, the
   實 last; columns are counted from the right, as the text sets them. Red
   and black rods are the signs; an empty position is zero. Nothing about
   the board's size or the rods' layout is assumed by the checks.

2. The 正負術 is signed arithmetic. The text's rule of signs (同名相除，異名
   相益，正無入負之，負無入正之 …) is read as what taking and adding signed
   numbers does, including the cases of an empty position (無入). `jiuzhang`
   does not test the rule; it uses it.

3. 徧乘 and 直除. \"以右行上禾徧乘中行而以直除\" is read as: multiply every
   figure of the middle column by the head of the right, then take the right
   column from it as many times as empties its head. The scan reads 直除;
   Wikisource's unproofread transcription of it drops 直 throughout.

4. The last step. The text finds each thing through the 法: the 法 times a
   column's 實, less the 實s already found, divided by the column's own
   count; then \"實皆如法\". Whether, and how, the 法 and 實s are reduced
   before the answer is written is not said. `jiuzhang answers` tries three
   readings: as the board leaves them; the 法 and all the 實s divided by what
   they share (約); each answer in its lowest terms.

5. Head (頭位) is the first position of the column worked on that is not
   empty, above the 實.

6. Counting 算. What one 算 is, nobody says. `jiuzhang rules` lists the
   candidates; every count is reported under every rule, and none is chosen.
   A rule is not tuned to make a count come out: where Dai Zhen's 99 comes
   out 98, it is reported as 98.

7. Readings in the alternative. Where a clause of the commentary can be read
   more than one way, the page gives the readings, and `jiuzhang readings`
   runs every combination. The readings offered are the transcriber's; a
   reading not offered is not tried, and that none of those offered works
   does not show that none does.

8. Units. Each page works in one unit, said in a note (升 where the text
   mixes 斗 and 升, 寸 for Problem 13, 兩 for Problem 9, 錢 for coins).

9. The text. Figures are read from the scans of the Sibu congkan edition (the
   Weiboxie printing, with Liu Hui's commentary, Li Chunfeng's notes and Dai
   Zhen's collation notes), checked against the Wikisource transcription of
   those pages, which is marked unproofread. Where they differ, the scan is
   followed and the line has a note.
";

pub fn verify() -> i32 {
    let mut total = (0, 0);
    let mut bad = 0;
    for (id, text) in CORPUS {
        let run = check::run(text);
        let (r, w) = run.checked;
        total.0 += r;
        total.1 += w;
        let notation = run.findings.iter().filter(|f| f.severity == Severity::Notation).count();
        let mut status = format!("{} right, {} wrong", r, w);
        if let Some(i) = run.stopped {
            status.push_str(&format!(", stops at line {}", i + 1));
        }
        if notation > 0 {
            status.push_str(&format!(", {} not in the notation", notation));
            bad += 1;
        }
        let alts = run.alternatives();
        if !alts.is_empty() {
            status.push_str(&format!(", {} lines read in the alternative", alts.len()));
        }
        println!("{:<18} {:<44} {}", id, run.title().unwrap_or_default(), status);
        for f in run.findings.iter().filter(|f| f.severity <= Severity::Fails) {
            println!("    line {}: {}", f.line + 1, f.message);
        }
    }
    println!(
        "\n{} pages; {} figures checked, {} right, {} wrong.",
        CORPUS.len(),
        total.0 + total.1,
        total.0,
        total.1
    );
    if bad > 0 {
        1
    } else {
        0
    }
}

/// Each problem whose answer has a fraction: how it is written, and how each
/// reading of the last step would write it.
pub fn answers() -> i32 {
    println!("How each answer is written, and which reading of the last step writes it so.");
    println!("board: every 實 over the 法 as the board leaves it.  約: the 法 and all the 實s divided by what");
    println!("they share.  lowest: each answer in its lowest terms.  ✓: writes it as the text does.\n");
    println!(
        "{:<8} {:<22} {:>8} {:>8} {:>8}   法 by the text, after 約",
        "problem", "written", "board", "約", "lowest"
    );
    let mut tally = [0usize; 3];
    let mut n = 0;
    for (id, text) in CORPUS {
        if !id.starts_with("no-") {
            continue;
        }
        let run = check::run(text);
        let Some(sol) = run.solution() else { continue };
        let answer = run.lines.iter().find_map(|l| match &l.line.kind {
            Kind::Stmt(v) => match &v[l.choice] {
                Stmt::Answer(a) => Some(a.clone()),
                _ => None,
            },
            _ => None,
        });
        let Some(a) = answer else { continue };
        if !a.iter().any(|m| m.has_fraction()) {
            continue;
        }
        n += 1;
        let f = forms(sol);
        let written: Vec<String> = a
            .iter()
            .filter(|m| m.has_fraction())
            .map(|m| format!("{}/{}", m.num, m.den))
            .collect();
        let first = a.iter().find(|m| m.has_fraction()).unwrap();
        let same_as_text = |den: Option<i128>| -> bool {
            match den {
                Some(d) => a
                    .iter()
                    .all(|m| over(m.value(), d).map(|s| s == written_of(m)).unwrap_or(false)),
                None => false,
            }
        };
        let lowest_ok = a
            .iter()
            .all(|m| !m.has_fraction() || m.value().fract().denom() == m.den);
        let b = same_as_text(f.board);
        let c = same_as_text(f.common);
        for (k, ok) in [b, c, lowest_ok].iter().enumerate() {
            if *ok {
                tally[k] += 1;
            }
        }
        let cell = |ok: bool, s: String| format!("{}{}", s, if ok { " ✓" } else { "  " });
        println!(
            "{:<8} {:<22} {:>8} {:>8} {:>8}   {}",
            id.trim_start_matches("no-").trim_start_matches('0'),
            written.join(" ").chars().take(22).collect::<String>(),
            cell(b, f.board.map(|d| d.to_string()).unwrap_or("-".into())),
            cell(c, f.common.map(|d| d.to_string()).unwrap_or("-".into())),
            cell(
                lowest_ok,
                show_mixed(first.value())
                    .split(' ')
                    .next_back()
                    .unwrap_or("")
                    .to_string()
            ),
            f.common.map(|d| d.to_string()).unwrap_or("-".into()),
        );
    }
    println!(
        "\nOf {} answers with a fraction: board writes {} as the text does, 約 {}, lowest terms {}.",
        n, tally[0], tally[1], tally[2]
    );
    0
}

fn written_of(m: &crate::page::Mixed) -> String {
    let s = if !m.has_fraction() {
        m.whole.to_string()
    } else if m.whole == 0 {
        format!("{}/{}", m.num, m.den)
    } else {
        format!("{} {}/{}", m.whole, m.num, m.den)
    };
    if m.neg {
        format!("-{}", s)
    } else {
        s
    }
}

/// The counts the commentators give, each under every rule and reading.
pub fn counts() -> i32 {
    for (id, text) in CORPUS {
        let run = check::run(text);
        let has_count = run
            .lines
            .iter()
            .any(|l| matches!(&l.line.kind, Kind::Stmt(v) if v.iter().any(|s| matches!(s, Stmt::Count(_)))));
        if !has_count {
            continue;
        }
        println!("{}  {}", id, run.title().unwrap_or_default());
        let census = match readings::all(text) {
            Ok(c) => c,
            Err(e) => {
                println!("  {}", e);
                continue;
            }
        };
        let agree: Vec<_> = census.agree.iter().collect();
        if census.total > 1 {
            println!(
                "  {} readings: {} can be done to the end, {} agree with every figure the source states",
                census.total,
                census.executable,
                agree.len()
            );
        }
        let mut lines: Vec<(usize, u64)> = vec![];
        for o in &agree {
            for (line, c, _) in o.counts() {
                if !lines.contains(&(line, c)) {
                    lines.push((line, c));
                }
            }
        }
        if agree.is_empty() {
            for l in &run.lines {
                if let Kind::Stmt(v) = &l.line.kind {
                    if let Stmt::Count(c) = v[l.choice] {
                        println!("  Count {}: no reading gets that far", c);
                    }
                }
            }
        }
        for (line, c) in lines {
            for rule in crate::count::RULES {
                let mut vals: Vec<u64> = agree
                    .iter()
                    .flat_map(|o| {
                        o.counts()
                            .into_iter()
                            .filter(|x| x.0 == line)
                            .flat_map(|x| x.2.into_iter().filter(|r| r.0 == rule.name()).map(|r| r.1))
                    })
                    .collect();
                vals.sort();
                vals.dedup();
                let hit = vals.contains(&c);
                let shown = if vals.len() > 8 {
                    format!("{} to {} ({} values)", vals[0], vals[vals.len() - 1], vals.len())
                } else {
                    vals.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(", ")
                };
                println!(
                    "  Count {:<4} {:<9} {}{}",
                    c,
                    rule.name(),
                    shown,
                    if hit { "   ✓" } else { "" }
                );
            }
        }
        println!();
    }
    // the methods on Problem 18, for comparison
    if let Some((_, text)) = crate::corpus::find("18") {
        let board = board_before_solve(text);
        if let Some(board) = board {
            println!("Problem 18 by each method, counted under each rule, and the board it needs:");
            print!("  {:<9}", "");
            for r in crate::count::RULES {
                print!("{:>9}", r.name());
            }
            println!("{:>8}{:>8}", "rods", "places");
            for m in METHODS {
                if let Ok(s) = solve(&board, *m) {
                    print!("  {:<9}", m.name());
                    for r in crate::count::RULES {
                        print!("{:>9}", crate::count::count(*r, &s.ops()));
                    }
                    let rods = s
                        .steps
                        .iter()
                        .map(|st| st.board.rod_count())
                        .max()
                        .unwrap_or(0)
                        .max(board.rod_count());
                    let places = s
                        .steps
                        .iter()
                        .map(|st| st.board.widest())
                        .max()
                        .unwrap_or(0)
                        .max(board.widest());
                    println!("{:>8}{:>8}", rods, places);
                }
            }
            println!("  rods: the most rods on the board at once; places: the most digits in any figure.");
        }
    }
    println!();
    for (id, stated) in LIU_HUI {
        let Some((_, text)) = crate::corpus::find(id) else {
            continue;
        };
        match readings::all(text) {
            Ok(c) => println!(
                "{}: Liu Hui says {} 算. Of {} readings of his working, {} can be done to the end and {} agree with the figures he states, so there is nothing to count.",
                id,
                stated,
                c.total,
                c.executable,
                c.agree.len()
            ),
            Err(e) => println!("{}: {}", id, e),
        }
    }
    // the two published readings of the 77
    let guo = crate::corpus::find("guo-18-old");
    let hart = crate::corpus::find("hart-18-old");
    if let (Some((_, g)), Some((_, h))) = (guo, hart) {
        let ops = |text: &str| -> (u64, usize, bool) {
            let run = check::run(text);
            let placings = run
                .lines
                .iter()
                .filter(|l| matches!(&l.line.kind, Kind::Stmt(v) if matches!(v[l.choice], Stmt::Set(..))))
                .count();
            let ok = run.stopped.is_none() && !run.findings.iter().any(|f| f.severity <= Severity::Fails);
            (crate::count::count(crate::count::Rule::Guo, &run.ops), placings, ok)
        };
        let (go, gp, gok) = ops(g);
        let (ho, hp, hok) = ops(h);
        let (gr, hr) = (rods_at(g), rods_at(h));
        println!("\nTwo published readings of Liu Hui's 77 (see docs/guo-77.md):");
        println!(
            "  Guo Shuchun (2020), 算 as operations, on his emended text: {}; under his rule {} operations, \
leaving {} of the 77 for placing rods, which are not modelled ({} columns are set on this page); the board holds {} rods at most",
            if gok { "runs to the prices" } else { "does NOT run to the prices" },
            go,
            77i64 - go as i64,
            gp,
            gr.ends
        );
        println!(
            "  Hart (2012), 算 as rods, on his path: {}; at the end of each step {} rods at most; worked a column at a time {}, \
a position at a time top down {}, bottom up {}, in the best order {}; under Guo's rule {} operations and {} placings",
            if hok { "runs to the price of 黍, and every count of rods he reports agrees" } else { "does NOT agree with his counts" },
            hr.ends,
            hr.whole,
            hr.top_down,
            hr.bottom_up,
            hr.best,
            ho,
            hp
        );
    }
    let _ = Method::Text;
    let _ = Mark::Right;
    0
}

/// The commentary pages whose count of 算 is stated, and the count.
pub const LIU_HUI: &[(&str, u64)] = &[("liu-hui-18-old", 77), ("liu-hui-18-new", 124)];

/// The board as it stands before the first `Solve` of a page.
pub fn board_before_solve(text: &str) -> Option<crate::board::Board> {
    let run = check::run(text);
    let mut b = None;
    for l in &run.lines {
        if let Kind::Stmt(v) = &l.line.kind {
            if matches!(v[l.choice], Stmt::Solve(_)) {
                return b;
            }
        }
        if let Some(s) = l.steps.last() {
            b = Some(s.board.clone());
        }
    }
    b
}

/// How many rods a page's board needs, under ways of working a step.
///
/// The board after each step is what the page records. Within a step, the
/// column being worked passes through other figures: multiplied first and
/// then taken from, it stands for a moment as the product; set position by
/// position straight to its new figures, it is part old and part new. The
/// most rods it needs depends on which. For each way, the most rods on the
/// board at any moment.
pub struct RodsAt {
    pub ends: u32,
    pub whole: u32,
    pub top_down: u32,
    pub bottom_up: u32,
    pub best: u32,
    /// (line, what, best) for the steps whose best order needs more than the ends do.
    pub over: Vec<(usize, String, u32)>,
}

fn positions_peak(before: &crate::board::Board, after: &crate::board::Board, t: usize, order: &[usize]) -> u32 {
    let mut b = before.clone();
    let mut peak = b.rod_count();
    let new = after.cols[t].clone().unwrap_or_default();
    for &i in order {
        if let Some(Some(c)) = b.cols.get_mut(t) {
            if i < c.len() {
                c[i] = new[i];
            }
        }
        peak = peak.max(b.rod_count());
    }
    peak.max(after.rod_count())
}

fn permutations(n: usize) -> Vec<Vec<usize>> {
    if n == 0 {
        return vec![vec![]];
    }
    let mut out = vec![];
    for p in permutations(n - 1) {
        for k in 0..=p.len() {
            let mut q = p.clone();
            q.insert(k, n - 1);
            out.push(q);
        }
    }
    out
}

/// A step of the source: a run of operation lines on one column, ended by
/// anything else (an assertion, a note, a blank line) or by another column.
struct Group {
    target: usize,
    line: usize,
    what: String,
    before: crate::board::Board,
    after: crate::board::Board,
}

pub fn rods_at(text: &str) -> RodsAt {
    let run = check::run(text);
    let mut r = RodsAt {
        ends: 0,
        whole: 0,
        top_down: 0,
        bottom_up: 0,
        best: 0,
        over: vec![],
    };
    let mut groups: Vec<Group> = vec![];
    let mut open: Option<Group> = None;
    let mut board: Option<crate::board::Board> = None;
    for (li, l) in run.lines.iter().enumerate() {
        let op = match &l.line.kind {
            Kind::Stmt(v) => v[l.choice].works() && !matches!(v[l.choice], Stmt::Set(..) | Stmt::Drop(_)),
            _ => false,
        };
        // every board the page records, worked a column at a time
        for st in &l.steps {
            r.whole = r.whole.max(st.board.rod_count());
        }
        let Some(last) = l.steps.last() else {
            if !op {
                groups.extend(open.take());
            }
            continue;
        };
        if !op {
            // placing a column, or solving: a step of its own
            groups.extend(open.take());
            r.ends = r.ends.max(last.board.rod_count());
            r.top_down = r.top_down.max(last.board.rod_count());
            r.bottom_up = r.bottom_up.max(last.board.rod_count());
            r.best = r.best.max(last.board.rod_count());
            board = Some(last.board.clone());
            continue;
        }
        let t = last.op.target;
        let before = board.clone().unwrap_or_else(|| last.board.clone());
        match &mut open {
            Some(g) if g.target == t => g.after = last.board.clone(),
            _ => {
                groups.extend(open.take());
                let what = l.line.text.split('"').next().unwrap_or("").trim().to_string();
                open = Some(Group {
                    target: t,
                    line: li,
                    what,
                    before,
                    after: last.board.clone(),
                });
            }
        }
        board = Some(last.board.clone());
    }
    groups.extend(open.take());
    for g in &groups {
        let h = g.after.height();
        let down: Vec<usize> = (0..h).collect();
        let up: Vec<usize> = (0..h).rev().collect();
        let ends = g.after.rod_count().max(g.before.rod_count());
        r.ends = r.ends.max(g.after.rod_count());
        r.top_down = r.top_down.max(positions_peak(&g.before, &g.after, g.target, &down));
        r.bottom_up = r.bottom_up.max(positions_peak(&g.before, &g.after, g.target, &up));
        let best = permutations(h)
            .iter()
            .map(|o| positions_peak(&g.before, &g.after, g.target, o))
            .min()
            .unwrap_or(0);
        r.best = r.best.max(best);
        if best > ends {
            r.over.push((g.line, g.what.clone(), best));
        }
    }
    r
}

pub fn rods(text: &str) -> i32 {
    let r = rods_at(text);
    println!("The most rods on the board at any moment, by how a step is worked:");
    println!(
        "  {:>5}  at the end of each step (a run of lines on one column, up to an assertion or a note)",
        r.ends
    );
    println!(
        "  {:>5}  each line worked on the whole column at once (a product set down, then taken from)",
        r.whole
    );
    println!(
        "  {:>5}  each step worked a position at a time, straight to its new figure, top down",
        r.top_down
    );
    println!("  {:>5}  the same, bottom up", r.bottom_up);
    println!(
        "  {:>5}  each step worked a position at a time, in the best order for it",
        r.best
    );
    for (line, what, n) in &r.over {
        println!("    line {}: {} needs {} at best", line + 1, what, n);
    }
    0
}
