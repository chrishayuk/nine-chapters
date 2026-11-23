//! The procedures, run on the board.
//!
//! `Text` is the 方程術 as the Nine Chapters states it for Problem 1:
//! multiply the whole of the next column by the head of the right-hand one
//! (徧乘) and take the right-hand column from it until its head is gone
//! (直除); then the middle column clears the left; the last figure standing
//! is the 法 (divisor) and the one under it the 實. Each thing above is found
//! by multiplying its column's 實 by the 法, taking away the 實s already
//! found, and dividing by the column's own count; at the end every 實 is
//! divided by the 法 (實皆如法).
//!
//! `Reducing` is the same, with each worked column divided by what its
//! figures have in common (約), which Liu Hui allows ("舉率以相減，不害餘數之課").
//!
//! `Cross` is Dai Zhen's reading in his collation notes (訂訛): the two
//! columns multiplied by each other's head (互乘) and taken one from the
//! other, and the things then found from the bottom up by value (轉減而上).
//!
//! `One` is Dai Zhen's shorter way for Problem 18: where a column's head is
//! one, work from that column, and where heads are equal, take directly.

use crate::board::{Board, Ev, Num, Op, Trouble};
use crate::rat::Rat;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Method {
    Text,
    Reducing,
    Cross,
    One,
}

pub const METHODS: &[Method] = &[Method::Text, Method::Reducing, Method::Cross, Method::One];

impl Method {
    pub fn name(self) -> &'static str {
        match self {
            Method::Text => "text",
            Method::Reducing => "reducing",
            Method::Cross => "cross",
            Method::One => "one",
        }
    }
    pub fn describe(self) -> &'static str {
        match self {
            Method::Text => "方程術 as stated: 徧乘 then 直除, right to left; the things found through the 法",
            Method::Reducing => "the same, each worked column divided by what its figures have in common (約)",
            Method::Cross => "Dai Zhen's 互乘: the two columns multiplied by each other's head, then taken; found by value from the bottom up",
            Method::One => "Dai Zhen's shorter way: work from a column whose head is one; equal heads taken directly",
        }
    }
    pub fn parse(s: &str) -> Option<Method> {
        METHODS.iter().copied().find(|m| m.name() == s)
    }
}

/// A step and the board after it.
#[derive(Clone, Debug)]
pub struct Step {
    pub op: Op,
    pub board: Board,
}

#[derive(Clone, Debug)]
pub struct Solution {
    pub method: Method,
    /// The column that cleared each position, top to bottom.
    pub order: Vec<usize>,
    /// The 法 (for the methods that find things through it).
    pub fa: Option<Num>,
    /// Each thing's 實 over the 法, before 實皆如法.
    pub shi: Vec<Rat>,
    /// What each thing comes to.
    pub values: Vec<Rat>,
    pub steps: Vec<Step>,
}

impl Solution {
    pub fn ops(&self) -> Vec<Op> {
        self.steps.iter().map(|s| s.op.clone()).collect()
    }
}

fn push(steps: &mut Vec<Step>, b: &Board, op: Op) {
    steps.push(Step { op, board: b.clone() });
}

/// Choose the column that clears position `k` from those still to be used.
fn pivot(b: &Board, method: Method, left: &[usize], k: usize) -> Option<usize> {
    let has = |i: &usize| b.col(*i).map(|c| c[k] != 0).unwrap_or(false);
    if method == Method::One {
        if let Some(i) = left.iter().rev().find(|i| has(i) && b.col(**i).unwrap()[k].abs() == 1) {
            return Some(*i);
        }
    }
    left.iter().copied().find(|i| has(i))
}

pub fn solve(board: &Board, method: Method) -> Result<Solution, (Trouble, Vec<Step>)> {
    let mut b = board.clone();
    let mut steps = vec![];
    let m = b.height() - 1; // things, without the 實
    let mut left: Vec<usize> = (0..b.n()).filter(|&i| b.cols[i].is_some()).collect();
    let mut order = vec![];
    let r: Result<(), Trouble> = (|| {
        for k in 0..m {
            let a = pivot(&b, method, &left, k).ok_or(Trouble::Empty {
                col: *left.first().unwrap_or(&0),
                pos: k,
            })?;
            left.retain(|&i| i != a);
            order.push(a);
            for &t in &left.clone() {
                let tk = b.col(t)?[k];
                if tk == 0 {
                    continue;
                }
                let ak = b.col(a)?[k];
                match method {
                    Method::Text | Method::Reducing => {
                        let op = b.multiply(t, ak)?;
                        push(&mut steps, &b, op);
                        let op = b.clear(a, t, k)?;
                        push(&mut steps, &b, op);
                        if method == Method::Reducing {
                            let op = b.reduce(t)?;
                            if !op.events.is_empty() {
                                push(&mut steps, &b, op);
                            }
                        }
                    }
                    Method::Cross => {
                        let op = b.cross(a, t, k)?;
                        push(&mut steps, &b, op);
                    }
                    Method::One => {
                        let op = if ak.abs() == tk.abs() || ak.abs() == 1 {
                            b.clear(a, t, k)?
                        } else {
                            b.cross(a, t, k)?
                        };
                        push(&mut steps, &b, op);
                    }
                }
            }
        }
        Ok(())
    })();
    if let Err(e) = r {
        return Err((e, steps));
    }
    // a column not used to clear any position must be empty, or the
    // conditions contradict one another
    for &i in &left {
        if b.col(i).map(|c| c.iter().any(|&x| x != 0)).unwrap_or(false) {
            return Err((Trouble::Inconsistent { col: i }, steps));
        }
    }
    let big = |steps: &Vec<Step>| (Trouble::TooLarge, steps.clone());
    let col = |i: usize| b.col(i).unwrap().clone();
    match method {
        Method::Text | Method::Reducing => {
            let last = col(order[m - 1]);
            let fa = last[m - 1];
            let mut shi = vec![Rat::ZERO; m];
            shi[m - 1] = Rat::int(last[m]);
            for k in (0..m - 1).rev() {
                let c = col(order[k]);
                let mut events = vec![];
                if c[m] != 0 {
                    events.push(Ev::Mul {
                        pos: m,
                        by: fa,
                        known: false,
                    });
                }
                let mut s = Rat::int(fa).try_mul(Rat::int(c[m])).ok_or_else(|| big(&steps))?;
                for j in k + 1..m {
                    if c[j] != 0 {
                        events.push(Ev::Mul {
                            pos: j,
                            by: c[j],
                            known: false,
                        });
                        events.push(Ev::Take {
                            pos: m,
                            times: 1,
                            empty: false,
                            known: false,
                        });
                        s = Rat::int(c[j])
                            .try_mul(shi[j])
                            .and_then(|p| s.try_sub(p))
                            .ok_or_else(|| big(&steps))?;
                    }
                }
                events.push(Ev::Div { pos: m, by: c[k] });
                shi[k] = s.try_div(Rat::int(c[k])).ok_or_else(|| big(&steps))?;
                let what = format!(
                    "find the {}: 法 times the 實 of {}, less the 實s found, by its {}",
                    b.thing(k),
                    b.name(order[k]),
                    c[k]
                );
                push(
                    &mut steps,
                    &b,
                    Op {
                        target: order[k],
                        actor: None,
                        what,
                        events,
                    },
                );
            }
            let events = (0..m).map(|_| Ev::Div { pos: m, by: fa }).collect();
            push(
                &mut steps,
                &b,
                Op {
                    target: order[m - 1],
                    actor: None,
                    what: format!("every 實 by the 法, {}", fa),
                    events,
                },
            );
            let values = shi
                .iter()
                .map(|&s| s.try_div(Rat::int(fa)))
                .collect::<Option<Vec<_>>>()
                .ok_or_else(|| big(&steps))?;
            Ok(Solution {
                method,
                order,
                fa: Some(fa),
                shi,
                values,
                steps,
            })
        }
        Method::Cross | Method::One => {
            let mut values = vec![Rat::ZERO; m];
            for k in (0..m).rev() {
                let c = col(order[k]);
                let mut events = vec![];
                let mut s = Rat::int(c[m]);
                for j in k + 1..m {
                    if c[j] != 0 {
                        events.push(Ev::Mul {
                            pos: j,
                            by: c[j],
                            known: false,
                        });
                        events.push(Ev::Take {
                            pos: m,
                            times: 1,
                            empty: false,
                            known: false,
                        });
                        s = Rat::int(c[j])
                            .try_mul(values[j])
                            .and_then(|p| s.try_sub(p))
                            .ok_or_else(|| big(&steps))?;
                    }
                }
                events.push(Ev::Div { pos: m, by: c[k] });
                values[k] = s.try_div(Rat::int(c[k])).ok_or_else(|| big(&steps))?;
                let what = format!("find the {} from {}", b.thing(k), b.name(order[k]));
                push(
                    &mut steps,
                    &b,
                    Op {
                        target: order[k],
                        actor: None,
                        what,
                        events,
                    },
                );
            }
            Ok(Solution {
                method,
                order,
                fa: None,
                shi: values.clone(),
                values,
                steps,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::count::{count, Rule};

    fn board(cols: &[&[Num]], things: &[&str]) -> Board {
        let mut b = Board::new(things.iter().map(|s| s.to_string()).collect(), cols.len());
        for (i, c) in cols.iter().enumerate() {
            b.set(i, c.to_vec()).unwrap();
        }
        b
    }

    #[test]
    fn problem_one_by_the_text() {
        let b = board(
            &[&[3, 2, 1, 39], &[2, 3, 1, 34], &[1, 2, 3, 26]],
            &["上", "中", "下", "實"],
        );
        let s = solve(&b, Method::Text).unwrap();
        assert_eq!(s.fa, Some(36));
        assert_eq!(s.shi, vec![Rat::int(333), Rat::int(153), Rat::int(99)]);
        assert_eq!(s.values[0], Rat::new(37, 4));
        let r = solve(&b, Method::Reducing).unwrap();
        assert_eq!(r.fa, Some(4));
    }

    const P18: &[&[Num]] = &[
        &[9, 7, 3, 2, 5, 140],
        &[7, 6, 4, 5, 3, 128],
        &[3, 5, 7, 6, 4, 116],
        &[2, 5, 3, 9, 4, 112],
        &[1, 3, 2, 8, 5, 95],
    ];
    const THINGS18: &[&str] = &["麻", "麥", "菽", "荅", "黍", "實"];

    #[test]
    fn dai_zhen_counts() {
        let b = board(P18, THINGS18);
        let s = solve(&b, Method::Cross).unwrap();
        assert_eq!(s.values, [7, 4, 3, 5, 6].map(Rat::int).to_vec());
        assert_eq!(count(Rule::Plain, &s.ops()), 145);
        let s = solve(&b, Method::One).unwrap();
        assert_eq!(s.values, [7, 4, 3, 5, 6].map(Rat::int).to_vec());
        // Dai Zhen says 99. The worked rule gives 98: the one left is a
        // multiplication by -1 in finding the 麥, which this rule does not
        // count. Recorded, not tuned away.
        assert_eq!(count(Rule::Worked, &s.ops()), 98);
        assert_eq!(count(Rule::Plain, &s.ops()), 101);
    }
}
