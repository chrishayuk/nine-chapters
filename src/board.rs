//! The counting board. A *fangcheng* array is set out in columns (行), the
//! first on the right, each a list of positions (位) from the top: one for
//! each kind of thing, then the 實 (shi), the "stuff" the column totals. A
//! figure is a signed whole number: red rods for 正 (positive), black for
//! 負 (negative), and an empty position is zero.
//!
//! Every operation on the board is recorded as what it did to each
//! position, so that the counts a commentator gives ("this uses 77 算") can
//! be compared afterwards under any rule of what one 算 is.

use std::fmt;

pub type Num = i128;

/// What an operation did to one position.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ev {
    /// The figure at `pos` multiplied by `by`. `known` when the result is
    /// fixed in advance (the heads after cross-multiplying are equal, so a
    /// reckoner need not work them).
    Mul { pos: usize, by: Num, known: bool },
    /// `times` of another column's figure taken from the one at `pos`
    /// (negative `times` is an addition). `empty` when the position taken
    /// from was empty (無入). `known` as for `Mul`.
    Take { pos: usize, times: Num, empty: bool, known: bool },
    /// The figure at `pos` divided by `by`.
    Div { pos: usize, by: Num },
}

/// One operation on one column.
#[derive(Clone, Debug)]
pub struct Op {
    pub target: usize,
    pub actor: Option<usize>,
    pub what: String,
    pub events: Vec<Ev>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Board {
    /// The names of the positions, top to bottom; the last is the 實.
    pub things: Vec<String>,
    /// Column 0 is the right-hand column. `None` is a column not set, or dropped.
    pub cols: Vec<Option<Vec<Num>>>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Trouble {
    NoColumn(usize),
    Empty { col: usize, pos: usize },
    Inexact { col: usize, pos: usize, figure: Num, by: Num },
    Odd { col: usize },
    TooLarge,
    Length { col: usize, want: usize, got: usize },
}

fn gcd(mut a: Num, mut b: Num) -> Num {
    a = a.abs();
    b = b.abs();
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

pub fn gcd_all(v: &[Num]) -> Num {
    v.iter().fold(0, |g, &x| gcd(g, x))
}

/// The name of column `i` of `n`, as the text names it.
pub fn col_name(i: usize, n: usize) -> String {
    if i == 0 {
        "right".into()
    } else if i + 1 == n {
        "left".into()
    } else if n == 3 {
        "middle".into()
    } else {
        ordinal(i + 1)
    }
}

pub fn col_name_zh(i: usize, n: usize) -> String {
    if i == 0 {
        "右行".into()
    } else if i + 1 == n {
        "左行".into()
    } else if n == 3 {
        "中行".into()
    } else {
        format!("第{}行", zh_digit(i + 1))
    }
}

fn zh_digit(n: usize) -> &'static str {
    ["〇", "一", "二", "三", "四", "五", "六", "七", "八", "九"].get(n).copied().unwrap_or("?")
}

pub fn ordinal(n: usize) -> String {
    let suffix = match (n % 10, n % 100) {
        (1, x) if x != 11 => "st",
        (2, x) if x != 12 => "nd",
        (3, x) if x != 13 => "rd",
        _ => "th",
    };
    format!("{}{}", n, suffix)
}

impl Board {
    pub fn new(things: Vec<String>, n: usize) -> Board {
        Board { things, cols: vec![None; n] }
    }
    pub fn n(&self) -> usize {
        self.cols.len()
    }
    pub fn height(&self) -> usize {
        self.things.len()
    }
    pub fn col(&self, i: usize) -> Result<&Vec<Num>, Trouble> {
        self.cols.get(i).and_then(|c| c.as_ref()).ok_or(Trouble::NoColumn(i))
    }
    pub fn name(&self, i: usize) -> String {
        col_name(i, self.n())
    }
    /// The first position of a column, above the 實, that is not empty.
    pub fn head(&self, i: usize) -> Result<Option<usize>, Trouble> {
        let c = self.col(i)?;
        Ok(c[..c.len().saturating_sub(1)].iter().position(|&x| x != 0))
    }
    pub fn thing(&self, pos: usize) -> &str {
        self.things.get(pos).map(|s| s.as_str()).unwrap_or("?")
    }

    pub fn set(&mut self, i: usize, figures: Vec<Num>) -> Result<Op, Trouble> {
        if figures.len() != self.height() {
            return Err(Trouble::Length { col: i, want: self.height(), got: figures.len() });
        }
        if i >= self.n() {
            self.cols.resize(i + 1, None);
        }
        self.cols[i] = Some(figures);
        Ok(Op { target: i, actor: None, what: format!("set {}", self.name(i)), events: vec![] })
    }

    pub fn drop_col(&mut self, i: usize) -> Result<Op, Trouble> {
        self.col(i)?;
        self.cols[i] = None;
        Ok(Op { target: i, actor: None, what: format!("drop {}", self.name(i)), events: vec![] })
    }

    /// 徧乘: multiply the whole of a column.
    pub fn multiply(&mut self, t: usize, by: Num) -> Result<Op, Trouble> {
        let mut c = self.col(t)?.clone();
        let mut events = vec![];
        for (pos, x) in c.iter_mut().enumerate() {
            if *x != 0 {
                *x = x.checked_mul(by).ok_or(Trouble::TooLarge)?;
                events.push(Ev::Mul { pos, by, known: false });
            }
        }
        self.cols[t] = Some(c);
        Ok(Op { target: t, actor: None, what: format!("multiply {} by {}", self.name(t), by), events })
    }

    /// Take `times` of column `a` from column `t` (減, or 直除 when repeated).
    /// `known` names a position whose result is fixed in advance.
    pub fn take(&mut self, a: usize, t: usize, times: Num, known: Option<usize>) -> Result<Op, Trouble> {
        let ac = self.col(a)?.clone();
        let mut c = self.col(t)?.clone();
        let mut events = vec![];
        for pos in 0..c.len() {
            if ac[pos] == 0 || times == 0 {
                continue;
            }
            let empty = c[pos] == 0;
            let d = ac[pos].checked_mul(times).ok_or(Trouble::TooLarge)?;
            c[pos] = c[pos].checked_sub(d).ok_or(Trouble::TooLarge)?;
            events.push(Ev::Take { pos, times, empty, known: known == Some(pos) });
        }
        self.cols[t] = Some(c);
        let what = match times {
            1 => format!("take {} from {}", self.name(a), self.name(t)),
            -1 => format!("add {} to {}", self.name(a), self.name(t)),
            k if k < 0 => format!("add {} to {}, {} times", self.name(a), self.name(t), -k),
            k => format!("take {} from {}, {} times", self.name(a), self.name(t), k),
        };
        Ok(Op { target: t, actor: Some(a), what, events })
    }

    /// 直除: take column `a` from `t` as many times as clears position `pos`.
    pub fn clear(&mut self, a: usize, t: usize, pos: usize) -> Result<Op, Trouble> {
        let ap = self.col(a)?[pos];
        let tp = self.col(t)?[pos];
        if ap == 0 {
            return Err(Trouble::Empty { col: a, pos });
        }
        if tp % ap != 0 {
            return Err(Trouble::Inexact { col: t, pos, figure: tp, by: ap });
        }
        self.take(a, t, tp / ap, Some(pos))
    }

    /// 不足減乃止: take `a` from `t` while it goes, at position `pos`: as
    /// many times as leaves a figure smaller than `a`'s there.
    pub fn as_far(&mut self, a: usize, t: usize, pos: usize) -> Result<Op, Trouble> {
        let ap = self.col(a)?[pos];
        let tp = self.col(t)?[pos];
        if ap == 0 {
            return Err(Trouble::Empty { col: a, pos });
        }
        let q = tp / ap; // towards zero
        let known = if tp % ap == 0 { Some(pos) } else { None };
        self.take(a, t, q, known)
    }

    /// 互乘: multiply `t` by `a`'s figure at `pos` and `a` by `t`'s, and
    /// take one from the other, so that `pos` is cleared in `t`. `a` is left
    /// as it was: its products are worked and taken, not set down.
    pub fn cross(&mut self, a: usize, t: usize, pos: usize) -> Result<Op, Trouble> {
        let ac = self.col(a)?.clone();
        let mut c = self.col(t)?.clone();
        let (p, q) = (ac[pos], c[pos]);
        if p == 0 {
            return Err(Trouble::Empty { col: a, pos });
        }
        let mut events = vec![];
        for i in 0..c.len() {
            let known = i == pos;
            if c[i] != 0 {
                events.push(Ev::Mul { pos: i, by: p, known });
            }
            if ac[i] != 0 && q != 0 {
                events.push(Ev::Mul { pos: i, by: q, known });
                events.push(Ev::Take { pos: i, times: 1, empty: c[i] == 0, known });
            }
            c[i] = c[i]
                .checked_mul(p)
                .and_then(|x| x.checked_sub(ac[i].checked_mul(q)?))
                .ok_or(Trouble::TooLarge)?;
        }
        self.cols[t] = Some(c);
        Ok(Op {
            target: t,
            actor: Some(a),
            what: format!("cross-multiply {} and {}, and take", self.name(a), self.name(t)),
            events,
        })
    }

    pub fn divide(&mut self, t: usize, by: Num) -> Result<Op, Trouble> {
        let mut c = self.col(t)?.clone();
        let mut events = vec![];
        for (pos, x) in c.iter_mut().enumerate() {
            if *x != 0 {
                if *x % by != 0 {
                    return Err(Trouble::Inexact { col: t, pos, figure: *x, by });
                }
                *x /= by;
                events.push(Ev::Div { pos, by });
            }
        }
        self.cols[t] = Some(c);
        Ok(Op { target: t, actor: None, what: format!("divide {} by {}", self.name(t), by), events })
    }

    /// 半: halve a column.
    pub fn halve(&mut self, t: usize) -> Result<Op, Trouble> {
        if self.col(t)?.iter().any(|x| x % 2 != 0) {
            return Err(Trouble::Odd { col: t });
        }
        let mut op = self.divide(t, 2)?;
        op.what = format!("halve {}", self.name(t));
        Ok(op)
    }

    /// 約: divide a column by what its figures have in common.
    pub fn reduce(&mut self, t: usize) -> Result<Op, Trouble> {
        let g = gcd_all(self.col(t)?);
        if g <= 1 {
            return Ok(Op { target: t, actor: None, what: format!("reduce {}: nothing in common", self.name(t)), events: vec![] });
        }
        let mut op = self.divide(t, g)?;
        op.what = format!("reduce {} by {}", self.name(t), g);
        Ok(op)
    }

    /// The rods it takes to set a figure: a digit up to five is that many
    /// rods; six to nine is a crossing rod for five and the rest.
    pub fn rods(x: Num) -> u32 {
        let mut x = x.unsigned_abs();
        let mut r = 0;
        while x > 0 {
            let d = (x % 10) as u32;
            r += if d > 5 { d - 4 } else { d };
            x /= 10;
        }
        r
    }
    pub fn rod_count(&self) -> u32 {
        self.cols.iter().flatten().flatten().map(|&x| Board::rods(x)).sum()
    }
    /// The most digits of any figure on the board: how many places a row needs.
    pub fn widest(&self) -> usize {
        self.cols.iter().flatten().flatten().map(|x| x.unsigned_abs().to_string().len()).max().unwrap_or(1)
    }
}

pub fn trouble_text(b: &Board, t: &Trouble) -> String {
    let n = b.n();
    match t {
        Trouble::NoColumn(i) => format!("there is no {} on the board", col_name(*i, n)),
        Trouble::Empty { col, pos } => format!("the {} of {} is empty: nothing to take with", b.thing(*pos), col_name(*col, n)),
        Trouble::Inexact { col, pos, figure, by } => format!(
            "{} at the {} of {} does not go by {} exactly",
            figure,
            b.thing(*pos),
            col_name(*col, n),
            by
        ),
        Trouble::Odd { col } => format!("{} cannot be halved: a figure in it is odd", col_name(*col, n)),
        Trouble::TooLarge => "a figure grows past what this program holds (about 10^38)".into(),
        Trouble::Length { col, want, got } => format!(
            "{} has {} figures; the board has {} positions ({})",
            col_name(*col, n),
            got,
            want,
            b.things.join(" ")
        ),
    }
}

/// Display width in a terminal: CJK takes two cells.
pub fn width(s: &str) -> usize {
    s.chars()
        .map(|c| {
            let u = c as u32;
            if (0x1100..=0x115F).contains(&u)
                || (0x2E80..=0xA4CF).contains(&u)
                || (0xAC00..=0xD7A3).contains(&u)
                || (0xF900..=0xFAFF).contains(&u)
                || (0xFE30..=0xFE4F).contains(&u)
                || (0xFF00..=0xFF60).contains(&u)
                || (0x20000..=0x3FFFD).contains(&u)
            {
                2
            } else {
                1
            }
        })
        .sum()
}

pub fn pad_left(s: &str, w: usize) -> String {
    format!("{}{}", " ".repeat(w.saturating_sub(width(s))), s)
}
pub fn pad_right(s: &str, w: usize) -> String {
    format!("{}{}", s, " ".repeat(w.saturating_sub(width(s))))
}

impl fmt::Display for Board {
    /// The board as it lies: the right-hand column on the right.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let n = self.n();
        let label = self.things.iter().map(|t| width(t)).max().unwrap_or(1);
        let shown: Vec<usize> = (0..n).rev().filter(|&i| self.cols[i].is_some()).collect();
        let w: Vec<usize> = shown
            .iter()
            .map(|&i| {
                let c = self.cols[i].as_ref().unwrap();
                c.iter().map(|x| x.to_string().len()).max().unwrap_or(1).max(width(&col_name(i, n)))
            })
            .collect();
        let mut head = " ".repeat(label);
        for (k, &i) in shown.iter().enumerate() {
            head.push_str("  ");
            head.push_str(&pad_left(&col_name(i, n), w[k]));
        }
        writeln!(f, "{}", head.trim_end())?;
        for pos in 0..self.height() {
            let mut row = pad_right(self.thing(pos), label);
            for (k, &i) in shown.iter().enumerate() {
                let x = self.cols[i].as_ref().unwrap()[pos];
                row.push_str("  ");
                row.push_str(&pad_left(&if x == 0 { "·".into() } else { x.to_string() }, w[k]));
            }
            writeln!(f, "{}", row)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn board() -> Board {
        let mut b = Board::new(vec!["上禾".into(), "中禾".into(), "下禾".into(), "實".into()], 3);
        b.set(0, vec![3, 2, 1, 39]).unwrap();
        b.set(1, vec![2, 3, 1, 34]).unwrap();
        b.set(2, vec![1, 2, 3, 26]).unwrap();
        b
    }
    #[test]
    fn the_first_step_of_problem_one() {
        let mut b = board();
        b.multiply(1, 3).unwrap();
        assert_eq!(b.col(1).unwrap(), &vec![6, 9, 3, 102]);
        let op = b.clear(0, 1, 0).unwrap();
        assert_eq!(b.col(1).unwrap(), &vec![0, 5, 1, 24]);
        assert_eq!(op.events.len(), 4);
    }
    #[test]
    fn cross_multiplying() {
        let mut b = board();
        b.cross(0, 1, 0).unwrap();
        assert_eq!(b.col(1).unwrap(), &vec![0, 5, 1, 24]);
    }
    #[test]
    fn as_far_as_it_goes() {
        let mut b = board();
        b.as_far(2, 0, 0).unwrap(); // 3 / 1 = 3
        assert_eq!(b.col(0).unwrap(), &vec![0, -4, -8, -39]);
    }
    #[test]
    fn rods() {
        assert_eq!(Board::rods(8), 4);
        assert_eq!(Board::rods(39), 3 + 5);
        assert_eq!(Board::rods(-721), 3 + 2 + 1);
    }
    #[test]
    fn inexact() {
        let mut b = board();
        assert!(matches!(b.clear(0, 1, 0), Err(Trouble::Inexact { .. })));
    }
}
