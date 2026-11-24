//! What is one 算? Liu Hui says his working of Problem 18 "uses 77 算" and
//! his new method 124; Dai Zhen, collating the text in the 1770s, says the
//! plain method uses 145 and a shorter way 99. None of them says what is
//! being counted. These are candidate rules, each a reading of the word, all
//! applied to the same record of what was done to each position.

use crate::board::{Ev, Op};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rule {
    /// Every multiplication, taking and division of one figure, leaving out
    /// only the heads known to cancel after cross-multiplying (Dai Zhen:
    /// 凡上一位互乘其數必同可省乘). Taking a figure several times over is one
    /// multiplication and one taking. This is the rule that gives Dai Zhen's
    /// 145 for the plain method.
    Plain,
    /// Every multiplication, taking and division of one figure, leaving out
    /// those whose result is known before it is worked: the heads after
    /// cross-multiplying, and multiplying or dividing by one (Dai Zhen's
    /// economies: 上一位遇一則省其與對行徧乘). This gives his 99 for the
    /// shorter way.
    Worked,
    /// As `Plain`, but a division by one (or minus one) is read off, not
    /// worked. Proposed by the independent review, after the counts were
    /// known, as a uniform rule that gives both of Dai Zhen's figures: so it
    /// is a candidate fitted after the fact, and is reported as one.
    Readoff,
    /// Every multiplication, taking and division of one figure, by one or
    /// not, known or not.
    Every,
    /// As `Worked`, but taking a figure several times over is that many
    /// takings, done one at a time as 直除 says (no multiplication).
    Repeated,
    /// One for each operation on a whole column, whatever it touches.
    Columns,
    /// Guo Shuchun's (九章筭術譯注, 2020, ch. 8, n. 40): 一算即一次運算，如布算，
    /// 以某數乘或除某一整行，行與行的一度減，實除以法. One multiplication or
    /// division of a whole column is one; each taking of one column from
    /// another is one (so a taking three times over is three); finding a
    /// thing is counted figure by figure, a division by one read off.
    /// Placing rods (布算) is in his count but not in this one: the board's
    /// placings are not modelled, and are reported apart.
    Guo,
}

pub const RULES: &[Rule] = &[
    Rule::Guo,
    Rule::Plain,
    Rule::Worked,
    Rule::Readoff,
    Rule::Every,
    Rule::Repeated,
    Rule::Columns,
];

impl Rule {
    pub fn name(self) -> &'static str {
        match self {
            Rule::Plain => "plain",
            Rule::Worked => "worked",
            Rule::Readoff => "readoff",
            Rule::Every => "every",
            Rule::Repeated => "repeated",
            Rule::Columns => "columns",
            Rule::Guo => "guo",
        }
    }
    pub fn describe(self) -> &'static str {
        match self {
            Rule::Plain => "each multiplication, taking or division of one figure, except a head known to cancel; a taking several times over is a multiplication and a taking",
            Rule::Worked => "each multiplication, taking or division of one figure that has to be worked (not by one, not a head known to cancel); a taking several times over is a multiplication and a taking",
            Rule::Readoff => "as plain, but a division by one is read off, not worked (proposed by the review, after the counts were known)",
            Rule::Every => "each multiplication, taking or division of one figure, by one or not",
            Rule::Repeated => "as worked, but a taking several times over is that many takings, one at a time",
            Rule::Columns => "each operation on a whole column",
            Rule::Guo => "Guo Shuchun's (2020, n. 40): each multiplication or division of a whole column, each taking of one column from another, each step of finding a thing; placing rods not counted here",
        }
    }
    pub fn parse(s: &str) -> Option<Rule> {
        RULES.iter().copied().find(|r| r.name() == s)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Tally {
    pub mul: u64,
    pub take: u64,
    pub div: u64,
}

impl Tally {
    pub fn total(&self) -> u64 {
        self.mul + self.take + self.div
    }
}

pub fn tally(rule: Rule, ops: &[Op]) -> Tally {
    let mut t = Tally::default();
    for op in ops {
        if rule == Rule::Guo && !op.scalar {
            let takes: Vec<i128> = op
                .events
                .iter()
                .filter_map(|e| {
                    if let Ev::Take { times, .. } = e {
                        Some(*times)
                    } else {
                        None
                    }
                })
                .collect();
            let muls: Vec<i128> = op
                .events
                .iter()
                .filter_map(|e| if let Ev::Mul { by, .. } = e { Some(*by) } else { None })
                .collect();
            let divs = op.events.iter().any(|e| matches!(e, Ev::Div { .. }));
            if !takes.is_empty() && !muls.is_empty() {
                // cross-multiplying: each column multiplied (unless by one), then one
                // taking. The other column's factor is the multiplication just before a
                // taking; the column's own, any other.
                let mut own = None;
                let mut other = None;
                for w in op.events.windows(2) {
                    if let (Ev::Mul { by, .. }, Ev::Take { .. }) = (w[0], w[1]) {
                        other.get_or_insert(by);
                    }
                }
                for (k, e) in op.events.iter().enumerate() {
                    if let Ev::Mul { by, .. } = e {
                        if !matches!(op.events.get(k + 1), Some(Ev::Take { .. })) {
                            own.get_or_insert(*by);
                        }
                    }
                }
                t.mul += [own, other].iter().flatten().filter(|b| b.abs() != 1).count() as u64;
                t.take += 1;
            } else if let Some(q) = takes.first() {
                t.take += q.unsigned_abs() as u64;
            } else if let Some(b) = muls.first() {
                if b.abs() != 1 {
                    t.mul += 1;
                }
            } else if divs {
                t.div += 1;
            }
            continue;
        }
        if rule == Rule::Columns {
            if !op.events.is_empty() {
                t.take += 1;
            }
            continue;
        }
        for ev in &op.events {
            match (*ev, rule) {
                (Ev::Mul { .. }, Rule::Every) => t.mul += 1,
                (Ev::Mul { known, .. }, Rule::Plain | Rule::Readoff | Rule::Guo) => {
                    if !known {
                        t.mul += 1
                    }
                }
                (Ev::Mul { by, known, .. }, _) => {
                    if !known && by.abs() != 1 {
                        t.mul += 1
                    }
                }
                (Ev::Take { .. }, Rule::Every) => t.take += 1,
                (Ev::Take { known: true, .. }, _) => {}
                (Ev::Take { times, .. }, Rule::Repeated) => t.take += times.unsigned_abs() as u64,
                (Ev::Take { times, .. }, _) => {
                    t.take += 1;
                    if times.abs() != 1 {
                        t.mul += 1
                    }
                }
                (Ev::Div { .. }, Rule::Every | Rule::Plain) => t.div += 1,
                (Ev::Div { by, .. }, _) => {
                    if by.abs() != 1 {
                        t.div += 1
                    }
                }
            }
        }
    }
    t
}

pub fn count(rule: Rule, ops: &[Op]) -> u64 {
    tally(rule, ops).total()
}
