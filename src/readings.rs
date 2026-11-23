//! Every combination of the readings a page gives in the alternative, run.
//! A combination *runs* if every operation in it can be done, and *agrees*
//! if every figure the source states (`Gives`, `Answer`, `Rates`) is what the
//! board gives. `Count` is not a test: what each agreeing combination counts
//! is reported, rule by rule.
//!
//! The combinations are taken in order, the first line's reading changing
//! slowest. When a combination stops, or disagrees, at a line, every
//! combination that reads the lines up to it the same way does the same,
//! and they are counted together without being run.

use crate::check::{self, Mark, Run, Severity};
use crate::count::{self, RULES};
use crate::page::{Kind, Stmt};

/// Most combinations actually run.
pub const LIMIT: u64 = 4_000_000;

pub struct Outcome {
    pub choice: Vec<(usize, usize)>,
    pub run: Run,
}

impl Outcome {
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

pub struct Census {
    pub total: u128,
    pub run: u64,
    /// (line, why, how many combinations), for those that stop.
    pub stops: Vec<(usize, String, u128)>,
    /// (line, what, how many), for those that run past a figure the board does not give.
    pub disagree: Vec<(usize, String, u128)>,
    pub agree: Vec<Outcome>,
}

impl Census {
    pub fn stopped(&self) -> u128 {
        self.stops.iter().map(|s| s.2).sum()
    }
    pub fn disagreeing(&self) -> u128 {
        self.disagree.iter().map(|s| s.2).sum()
    }
}

fn tally(v: &mut Vec<(usize, String, u128)>, line: usize, why: String, n: u128) {
    match v.iter_mut().find(|s| s.0 == line && s.1 == why) {
        Some(s) => s.2 += n,
        None => v.push((line, why, n)),
    }
}

pub fn all(text: &str) -> Result<Census, String> {
    let parsed = crate::page::parse(text);
    let base = check::run_lines(parsed.clone(), &[]);
    let alts = base.alternatives();
    let sizes: Vec<u128> = alts.iter().map(|a| a.1 as u128).collect();
    let total: u128 = sizes.iter().product();
    let mut c = Census { total, run: 0, stops: vec![], disagree: vec![], agree: vec![] };
    let mut idx = vec![0usize; alts.len()];
    // combinations sharing positions ..=k: the product of the sizes after k
    let after = |k: usize| -> u128 { sizes[k + 1..].iter().product() };
    loop {
        if c.run >= LIMIT {
            return Err(format!("more than {} combinations would have to be run; give fewer readings", LIMIT));
        }
        let choice: Vec<(usize, usize)> = alts.iter().zip(&idx).map(|(a, &k)| (a.0, k)).collect();
        let run = check::run_lines(parsed.clone(), &choice);
        c.run += 1;
        // the first line at which this combination fails, if it does
        let fail = run
            .findings
            .iter()
            .filter(|f| f.severity <= Severity::Fails)
            .min_by_key(|f| f.line)
            .map(|f| (f.line, f.severity, f.message.clone()));
        let mut k = idx.len(); // the position to advance; len = none (the last)
        match fail {
            Some((line, sev, why)) => {
                let pos = alts.iter().rposition(|a| a.0 <= line);
                let n = match pos {
                    Some(p) => after(p),
                    None => total,
                };
                if sev == Severity::Fails {
                    tally(&mut c.stops, line, why, n);
                } else {
                    tally(&mut c.disagree, line, why, n);
                }
                match pos {
                    Some(p) => k = p,
                    None => return Ok(c),
                }
            }
            None => c.agree.push(Outcome { choice, run }),
        }
        if idx.is_empty() {
            return Ok(c);
        }
        if k == idx.len() {
            k = idx.len() - 1;
        }
        // advance the odometer at k
        for j in k + 1..idx.len() {
            idx[j] = 0;
        }
        loop {
            idx[k] += 1;
            if (idx[k] as u128) < sizes[k] {
                break;
            }
            idx[k] = 0;
            if k == 0 {
                return Ok(c);
            }
            k -= 1;
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
    let c = match all(text) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("jiuzhang: {}", e);
            return 2;
        }
    };
    println!(
        "{} {} ({} run, the rest counted with one that stopped or disagreed at the same line):",
        c.total,
        if c.total == 1 { "reading" } else { "readings" },
        c.run
    );
    println!(
        "  {} can be done to the end; {} of them agree with every figure the source states.",
        c.total - c.stopped(),
        c.agree.len()
    );
    let mut stops = c.stops.clone();
    stops.sort_by(|a, b| a.0.cmp(&b.0).then(b.2.cmp(&a.2)));
    let mut shown_lines = std::collections::BTreeMap::<usize, (u128, Vec<(String, u128)>)>::new();
    for (line, why, n) in &stops {
        let e = shown_lines.entry(*line).or_default();
        e.0 += n;
        e.1.push((why.clone(), *n));
    }
    for (line, (n, whys)) in &shown_lines {
        println!("  {} stop at line {}", n, line + 1);
        let mut whys = whys.clone();
        whys.sort_by(|a, b| b.1.cmp(&a.1));
        for (w, k) in whys.iter().take(3) {
            println!("      {}: {}", k, w);
        }
        if whys.len() > 3 {
            println!("      … and {} other reasons", whys.len() - 3);
        }
    }
    let mut dis = c.disagree.clone();
    dis.sort();
    let mut by_line = std::collections::BTreeMap::<usize, u128>::new();
    for (l, _, n) in &dis {
        *by_line.entry(*l).or_default() += n;
    }
    for (line, n) in by_line {
        println!("  {} run past line {} but disagree with the figure there", n, line + 1);
    }
    const SHOWN: usize = 12;
    for (k, o) in c.agree.iter().enumerate() {
        if k == SHOWN {
            println!("\n… and {} more", c.agree.len() - SHOWN);
            break;
        }
        println!("\nagrees:\n{}", describe(o));
        for (line, cnt, got) in o.counts() {
            let s = got
                .iter()
                .map(|(r, v)| format!("{} {}{}", r, v, if *v == cnt { " ✓" } else { "" }))
                .collect::<Vec<_>>()
                .join(", ");
            println!("  Count {} (line {}): {}", cnt, line + 1, s);
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
