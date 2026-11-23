//! Every combination of the readings a page gives in the alternative, run.
//! A combination *runs* if every operation in it can be done, and *agrees*
//! if every figure the source states (`Gives`, `Answer`, `Rates`) is what the
//! board gives. `Count` is not a test: what each agreeing combination counts
//! is reported, rule by rule.

use crate::check::{self, Mark, Run, Severity};
use crate::count::{self, RULES};
use crate::page::{Kind, Stmt};

pub const LIMIT: usize = 1 << 20;

pub struct Outcome {
    pub choice: Vec<(usize, usize)>,
    pub run: Run,
}

impl Outcome {
    pub fn runs(&self) -> bool {
        self.run.stopped.is_none()
    }
    pub fn agrees(&self) -> bool {
        self.runs() && !self.run.findings.iter().any(|f| f.severity <= Severity::Wrong)
    }
    /// What each `Count` line comes to, rule by rule.
    pub fn counts(&self) -> Vec<(usize, u64, Vec<(&'static str, u64)>)> {
        let mut out = vec![];
        let mut since = 0;
        let mut ops_before = 0;
        for (i, l) in self.run.lines.iter().enumerate() {
            ops_before += l.steps.len();
            if let Kind::Stmt(v) = &l.line.kind {
                if let Stmt::Count(c) = v[l.choice] {
                    let ops = &self.run.ops[since..ops_before];
                    out.push((i, c, RULES.iter().map(|r| (r.name(), count::count(*r, ops))).collect()));
                    since = ops_before;
                }
            }
        }
        out
    }
}

pub fn all(text: &str) -> Result<Vec<Outcome>, String> {
    let base = check::run(text);
    let alts = base.alternatives();
    let total: usize = alts.iter().map(|a| a.1).try_fold(1usize, |acc, n| acc.checked_mul(n)).unwrap_or(usize::MAX);
    if total > LIMIT {
        return Err(format!("{} combinations: more than {} is not tried", total, LIMIT));
    }
    let mut out = vec![];
    let mut idx = vec![0usize; alts.len()];
    loop {
        let choice: Vec<(usize, usize)> = alts.iter().zip(&idx).map(|(a, &k)| (a.0, k)).collect();
        let run = check::run_with(text, &choice);
        out.push(Outcome { choice, run });
        // next combination
        let mut k = 0;
        loop {
            if k == idx.len() {
                return Ok(out);
            }
            idx[k] += 1;
            if idx[k] < alts[k].1 {
                break;
            }
            idx[k] = 0;
            k += 1;
        }
    }
}

fn describe(o: &Outcome) -> String {
    o.choice
        .iter()
        .map(|(line, k)| {
            let l = &o.run.lines[*line].line;
            let body = l.text.split('"').next().unwrap_or("").split(" %").next().unwrap_or("");
            let alt = body.split('|').nth(*k).unwrap_or("").trim().trim_end_matches('.');
            format!("  line {:>3}: {}", line + 1, alt)
        })
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn report(text: &str) -> i32 {
    let outs = match all(text) {
        Ok(o) => o,
        Err(e) => {
            eprintln!("jiuzhang: {}", e);
            return 2;
        }
    };
    let n = outs.len();
    let runs = outs.iter().filter(|o| o.runs()).count();
    let agree: Vec<&Outcome> = outs.iter().filter(|o| o.agrees()).collect();
    println!(
        "{} {}: {} can be done to the end, {} of them agree with every figure the source states.",
        n,
        if n == 1 { "reading" } else { "readings" },
        runs,
        agree.len()
    );
    // where the others stop
    let mut stops: Vec<(usize, String, usize)> = vec![];
    for o in outs.iter().filter(|o| !o.runs()) {
        let i = o.run.stopped.unwrap();
        let why = o.run.findings.iter().find(|f| f.line == i && f.severity == Severity::Fails).map(|f| f.message.clone()).unwrap_or_default();
        match stops.iter_mut().find(|s| s.0 == i) {
            Some(s) => s.2 += 1,
            None => stops.push((i, why, 1)),
        }
    }
    stops.sort();
    for (i, why, k) in &stops {
        println!("  {} stop at line {}: {}", k, i + 1, why);
    }
    let wrong = outs.iter().filter(|o| o.runs() && !o.agrees()).count();
    if wrong > 0 {
        println!("  {} run to the end but disagree with a figure the source states", wrong);
    }
    const SHOWN: usize = 12;
    for (k, o) in agree.iter().enumerate() {
        if k == SHOWN {
            println!("\n… and {} more", agree.len() - SHOWN);
            break;
        }
        println!("\nagrees:\n{}", describe(o));
        for (line, c, got) in o.counts() {
            let s = got.iter().map(|(r, v)| format!("{} {}{}", r, v, if *v == c { " ✓" } else { "" })).collect::<Vec<_>>().join(", ");
            println!("  Count {} (line {}): {}", c, line + 1, s);
        }
        for l in &o.run.lines {
            for m in &l.marks {
                if let Mark::Info(t) = m {
                    if matches!(&l.line.kind, Kind::Stmt(v) if matches!(v[l.choice], Stmt::Answer(_))) {
                        println!("  {}", t);
                    }
                }
            }
        }
    }
    0
}
