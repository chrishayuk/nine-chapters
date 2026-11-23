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
    /// Every multiplication, taking and division of one figure, by one or
    /// not, known or not.
    Every,
    /// As `Worked`, but taking a figure several times over is that many
    /// takings, done one at a time as 直除 says (no multiplication).
    Repeated,
    /// One for each operation on a whole column, whatever it touches.
    Columns,
}

pub const RULES: &[Rule] = &[Rule::Plain, Rule::Worked, Rule::Every, Rule::Repeated, Rule::Columns];

impl Rule {
    pub fn name(self) -> &'static str {
        match self {
            Rule::Plain => "plain",
            Rule::Worked => "worked",
            Rule::Every => "every",
            Rule::Repeated => "repeated",
            Rule::Columns => "columns",
        }
    }
    pub fn describe(self) -> &'static str {
        match self {
            Rule::Plain => "each multiplication, taking or division of one figure, except a head known to cancel; a taking several times over is a multiplication and a taking",
            Rule::Worked => "each multiplication, taking or division of one figure that has to be worked (not by one, not a head known to cancel); a taking several times over is a multiplication and a taking",
            Rule::Every => "each multiplication, taking or division of one figure, by one or not",
            Rule::Repeated => "as worked, but a taking several times over is that many takings, one at a time",
            Rule::Columns => "each operation on a whole column",
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
        if rule == Rule::Columns {
            if !op.events.is_empty() {
                t.take += 1;
            }
            continue;
        }
        for ev in &op.events {
            match (*ev, rule) {
                (Ev::Mul { .. }, Rule::Every) => t.mul += 1,
                (Ev::Mul { known, .. }, Rule::Plain) => {
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
