//! Reading a `.jz` page: a board set out and the operations done on it,
//! one to a line, in the order the text gives them. `docs/notation.md` has
//! all of it.
//!
//! ```text
//! No. 8.1
//! "今有上禾三秉，中禾二秉，下禾一秉，實三十九斗……"
//! Things 上禾 中禾 下禾 實
//! Set right   3 2 1 39
//! Set middle  2 3 1 34
//! Set left    1 2 3 26
//! Multiply middle by the head of right.      % 以右行上禾徧乘中行
//! Take right from middle until the head is gone.   % 而以直除
//! Solve.
//! Answer 9 1/4, 4 1/4, 2 3/4
//! ```
//!
//! A line may give readings in the alternative, `A | B`: the page runs with
//! the first, and `jiuzhang readings` runs every combination.

use crate::board::Num;
use crate::solve::Method;

#[derive(Clone, Debug, PartialEq)]
pub enum ColRef {
    Right,
    Middle,
    Left,
    /// Counted from the right, from 1: `2nd` is 第二行.
    Nth(usize),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Pos {
    /// The first position of the column worked on that is not empty (頭位).
    Head,
    Thing(String),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Factor {
    N(Num),
    Of(Pos, ColRef),
}

#[derive(Clone, Debug, PartialEq)]
pub enum How {
    /// 減: once.
    Once,
    /// 直除 / 去: as many times as clears the position.
    Until(Pos),
    /// 不足減乃止: as many times as goes.
    AsFar(Pos),
    /// 互乘: cross-multiplied, then taken.
    Crosswise(Pos),
    /// 相減: each taken from the other in turn until the position is gone.
    InTurn(Pos),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Mixed {
    pub neg: bool,
    pub whole: Num,
    pub num: Num,
    pub den: Num,
}

impl Mixed {
    pub fn value(&self) -> crate::rat::Rat {
        use crate::rat::Rat;
        let v = Rat::int(self.whole) + if self.den == 0 { Rat::ZERO } else { Rat::new(self.num, self.den) };
        if self.neg {
            -v
        } else {
            v
        }
    }
    /// As written: `1 18/52`.
    pub fn written(&self) -> String {
        let s = if !self.has_fraction() {
            self.whole.to_string()
        } else if self.whole == 0 {
            format!("{}/{}", self.num, self.den)
        } else {
            format!("{} {}/{}", self.whole, self.num, self.den)
        };
        if self.neg {
            format!("-{}", s)
        } else {
            s
        }
    }
    pub fn has_fraction(&self) -> bool {
        self.den != 0 && self.num != 0
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Stmt {
    Things(Vec<String>),
    Set(ColRef, Vec<Num>),
    Multiply(ColRef, Factor),
    Take { a: ColRef, t: ColRef, how: How },
    Add { a: ColRef, t: ColRef },
    Halve(ColRef),
    Reduce(ColRef),
    Divide(ColRef, Num),
    Drop(ColRef),
    Gives(ColRef, Vec<Num>),
    Solve(Method),
    Answer(Vec<Mixed>),
    Rates(Vec<Num>),
    Count(u64),
    /// The reading that does nothing: a clause taken as corrupt or idle.
    Nothing,
}

impl Stmt {
    pub fn keyword(&self) -> &'static str {
        match self {
            Stmt::Things(_) => "Things",
            Stmt::Set(..) => "Set",
            Stmt::Multiply(..) => "Multiply",
            Stmt::Take { .. } => "Take",
            Stmt::Add { .. } => "Add",
            Stmt::Halve(_) => "Halve",
            Stmt::Reduce(_) => "Reduce",
            Stmt::Divide(..) => "Divide",
            Stmt::Drop(_) => "Drop",
            Stmt::Gives(..) => "Gives",
            Stmt::Solve(_) => "Solve",
            Stmt::Answer(_) => "Answer",
            Stmt::Rates(_) => "Rates",
            Stmt::Count(_) => "Count",
            Stmt::Nothing => "Nothing",
        }
    }
    /// Does it change the board?
    pub fn works(&self) -> bool {
        matches!(
            self,
            Stmt::Set(..)
                | Stmt::Multiply(..)
                | Stmt::Take { .. }
                | Stmt::Add { .. }
                | Stmt::Halve(_)
                | Stmt::Reduce(_)
                | Stmt::Divide(..)
                | Stmt::Drop(_)
        )
    }
}

pub const KEYWORDS: &[&str] = &[
    "Things", "Set", "Multiply", "Take", "Add", "Halve", "Reduce", "Divide", "Drop", "Gives", "Solve", "Answer", "Rates",
    "Count", "Nothing",
];

/// The kind of a token, for colouring.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tok {
    Keyword,
    Word,
    Column,
    Number,
    Thing,
    Quote,
    Note,
    Heading,
    Bar,
}

#[derive(Clone, Debug)]
pub enum Kind {
    Blank,
    Heading(String),
    /// A line of the text, quoted, not computed.
    Words(String),
    /// A line that is only a note.
    Note,
    /// One or more readings of an operation.
    Stmt(Vec<Stmt>),
    Bad(String),
}

#[derive(Clone, Debug)]
pub struct Line {
    pub text: String,
    pub kind: Kind,
    /// Tokens, by character offsets: (start, length, kind).
    pub toks: Vec<(usize, usize, Tok)>,
    /// The source words quoted on the same line as an operation.
    pub quote: Option<String>,
    /// Start of a trailing `%` note, in characters.
    pub note: Option<usize>,
}

/// Words with their character offsets.
fn words(s: &str, base: usize) -> Vec<(usize, String)> {
    let mut out = vec![];
    let mut cur = String::new();
    let mut start = 0;
    for (i, c) in s.chars().enumerate() {
        if c.is_whitespace() || c == ',' || c == '，' {
            if !cur.is_empty() {
                out.push((base + start, std::mem::take(&mut cur)));
            }
        } else {
            if cur.is_empty() {
                start = i;
            }
            cur.push(c);
        }
    }
    if !cur.is_empty() {
        out.push((base + start, cur));
    }
    // a full stop ends a statement; it is not part of the last word
    if let Some(last) = out.last_mut() {
        if last.1.len() > 1 && (last.1.ends_with('.') || last.1.ends_with('。')) {
            last.1.pop();
        } else if last.1 == "." || last.1 == "。" {
            out.pop();
        }
    }
    out
}

pub fn parse_col(w: &str) -> Option<ColRef> {
    Some(match w {
        "right" | "右行" => ColRef::Right,
        "middle" | "中行" => ColRef::Middle,
        "left" | "左行" => ColRef::Left,
        _ => {
            let digits: String = w.chars().take_while(|c| c.is_ascii_digit()).collect();
            let rest = &w[digits.len()..];
            if !digits.is_empty() && matches!(rest, "st" | "nd" | "rd" | "th") {
                let k: usize = digits.parse().ok()?;
                if k == 0 {
                    return None;
                }
                ColRef::Nth(k)
            } else {
                let zh = ["一", "二", "三", "四", "五", "六", "七", "八", "九"];
                let inner = w.strip_prefix('第')?.strip_suffix('行')?;
                ColRef::Nth(zh.iter().position(|d| *d == inner)? + 1)
            }
        }
    })
}

pub fn parse_num(w: &str) -> Option<Num> {
    if w == "·" {
        return Some(0);
    }
    let w = w.replace('−', "-");
    let (neg, body) = match w.strip_prefix('-') {
        Some(b) => (true, b.to_string()),
        None => (false, w.strip_prefix('+').unwrap_or(&w).to_string()),
    };
    if body.is_empty() || !body.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }
    let v: Num = body.parse().ok()?;
    Some(if neg { -v } else { v })
}

struct P<'a> {
    w: &'a [(usize, String)],
    i: usize,
    toks: Vec<(usize, usize, Tok)>,
}

impl<'a> P<'a> {
    fn peek(&self) -> Option<&str> {
        self.w.get(self.i).map(|(_, s)| s.as_str())
    }
    fn next(&mut self, tok: Tok) -> Option<String> {
        let (at, s) = self.w.get(self.i)?;
        self.toks.push((*at, s.chars().count(), tok));
        self.i += 1;
        Some(s.clone())
    }
    fn word(&mut self, want: &str) -> Result<(), String> {
        match self.peek() {
            Some(w) if w == want => {
                self.next(Tok::Word);
                Ok(())
            }
            Some(w) => Err(format!("expected `{}`, found `{}`", want, w)),
            None => Err(format!("expected `{}`", want)),
        }
    }
    fn eat(&mut self, want: &str) -> bool {
        if self.peek() == Some(want) {
            self.next(Tok::Word);
            true
        } else {
            false
        }
    }
    fn col(&mut self) -> Result<ColRef, String> {
        let w = self.peek().ok_or("expected a column: right, middle, left, 2nd, 3rd …")?.to_string();
        let c = parse_col(&w).ok_or(format!("`{}` is not a column: right, middle, left, 2nd, 3rd …", w))?;
        self.next(Tok::Column);
        Ok(c)
    }
    fn num(&mut self) -> Result<Num, String> {
        let w = self.peek().ok_or("expected a number")?.to_string();
        let n = parse_num(&w).ok_or(format!("`{}` is not a whole number", w))?;
        self.next(Tok::Number);
        Ok(n)
    }
    fn nums(&mut self) -> Result<Vec<Num>, String> {
        let mut v = vec![];
        while self.peek().is_some() {
            v.push(self.num()?);
        }
        if v.is_empty() {
            return Err("expected figures".into());
        }
        Ok(v)
    }
    /// `the head` or `the THING` / `THING`.
    fn pos(&mut self) -> Result<Pos, String> {
        self.eat("the");
        match self.peek() {
            Some("head") => {
                self.next(Tok::Word);
                Ok(Pos::Head)
            }
            Some(_) => Ok(Pos::Thing(self.next(Tok::Thing).unwrap())),
            None => Err("expected `the head` or the name of a thing".into()),
        }
    }
    fn done(&self) -> Result<(), String> {
        match self.peek() {
            None => Ok(()),
            Some(w) => Err(format!("`{}`: more than the statement takes", w)),
        }
    }
}

fn mixed(s: &str) -> Option<Mixed> {
    // `9 1/4`, `1/4`, `9`, `-3`, `37 1/2`
    let s = s.trim().replace('−', "-");
    let (neg, s) = match s.strip_prefix('-') {
        Some(r) => (true, r.trim().to_string()),
        None => (false, s),
    };
    let parts: Vec<&str> = s.split_whitespace().collect();
    let frac = |f: &str| -> Option<(Num, Num)> {
        let (a, b) = f.split_once('/')?;
        let (a, b): (Num, Num) = (a.parse().ok()?, b.parse().ok()?);
        if b == 0 {
            None
        } else {
            Some((a, b))
        }
    };
    let m = match parts.as_slice() {
        [w] if w.contains('/') => {
            let (n, d) = frac(w)?;
            Mixed { neg, whole: 0, num: n, den: d }
        }
        [w] => Mixed { neg, whole: w.parse().ok()?, num: 0, den: 0 },
        [w, f] => {
            let (n, d) = frac(f)?;
            Mixed { neg, whole: w.parse().ok()?, num: n, den: d }
        }
        _ => return None,
    };
    Some(m)
}

fn stmt(text: &str, base: usize) -> (Result<Stmt, String>, Vec<(usize, usize, Tok)>) {
    let w = words(text, base);
    let mut p = P { w: &w, i: 0, toks: vec![] };
    let r = (|| {
        let kw = p.peek().ok_or("empty")?.to_string();
        if !KEYWORDS.contains(&kw.as_str()) {
            return Err(format!("`{}` is not an operation: {}", kw, KEYWORDS.join(", ")));
        }
        p.next(Tok::Keyword);
        let s = match kw.as_str() {
            "Things" => {
                let mut v = vec![];
                while p.peek().is_some() {
                    v.push(p.next(Tok::Thing).unwrap());
                }
                if v.len() < 2 {
                    return Err("`Things` names each position, top to bottom, and last the 實".into());
                }
                Stmt::Things(v)
            }
            "Set" => {
                let c = p.col()?;
                Stmt::Set(c, p.nums()?)
            }
            "Gives" => {
                let c = p.col()?;
                Stmt::Gives(c, p.nums()?)
            }
            "Multiply" => {
                let c = p.col()?;
                p.word("by")?;
                let f = if p.peek() == Some("the") {
                    let pos = p.pos()?;
                    p.word("of")?;
                    Factor::Of(pos, p.col()?)
                } else {
                    Factor::N(p.num()?)
                };
                Stmt::Multiply(c, f)
            }
            "Take" => {
                let a = p.col()?;
                p.word("from")?;
                let t = p.col()?;
                let how = match p.peek() {
                    None => How::Once,
                    Some("until") => {
                        p.next(Tok::Word);
                        let pos = p.pos()?;
                        p.word("is")?;
                        p.word("gone")?;
                        How::Until(pos)
                    }
                    Some("as") => {
                        p.next(Tok::Word);
                        p.word("far")?;
                        p.word("as")?;
                        if p.eat("it") {
                            p.word("goes")?;
                            How::AsFar(Pos::Head)
                        } else {
                            let pos = p.pos()?;
                            p.word("goes")?;
                            How::AsFar(pos)
                        }
                    }
                    Some("in") => {
                        p.next(Tok::Word);
                        p.word("turn")?;
                        let pos = if p.eat("until") {
                            let pos = p.pos()?;
                            p.word("is")?;
                            p.word("gone")?;
                            pos
                        } else {
                            Pos::Head
                        };
                        How::InTurn(pos)
                    }
                    Some("crosswise") => {
                        p.next(Tok::Word);
                        let pos = if p.eat("at") { p.pos()? } else { Pos::Head };
                        How::Crosswise(pos)
                    }
                    Some(w) => return Err(format!("`{}`: after `Take A from B`, `until the head is gone`, `as far as it goes`, `in turn` or `crosswise`", w)),
                };
                Stmt::Take { a, t, how }
            }
            "Add" => {
                let a = p.col()?;
                p.word("to")?;
                Stmt::Add { a, t: p.col()? }
            }
            "Halve" => Stmt::Halve(p.col()?),
            "Reduce" => Stmt::Reduce(p.col()?),
            "Divide" => {
                let c = p.col()?;
                p.word("by")?;
                Stmt::Divide(c, p.num()?)
            }
            "Drop" => Stmt::Drop(p.col()?),
            "Solve" => {
                let m = if p.eat("by") {
                    let w = p.next(Tok::Word).ok_or("expected a method: text, reducing, cross, one")?;
                    Method::parse(&w).ok_or(format!("`{}` is not a method: text, reducing, cross, one", w))?
                } else {
                    Method::Text
                };
                Stmt::Solve(m)
            }
            "Rates" => Stmt::Rates(p.nums()?),
            "Nothing" => Stmt::Nothing,
            "Count" => {
                let n = p.num()?;
                if n < 0 {
                    return Err("a count is not negative".into());
                }
                Stmt::Count(n as u64)
            }
            "Answer" => {
                // the rest of the line, split on commas
                let start = p.w[p.i..].first().map(|(a, _)| *a - base).unwrap_or(text.chars().count());
                let rest: String = text.chars().skip(start).collect();
                let rest = rest.trim_end().trim_end_matches('.');
                let mut v = vec![];
                let mut at = base + start;
                for part in rest.split([',', '，']) {
                    let n = part.chars().count();
                    let m = mixed(part).ok_or(format!("`{}` is not a number: write 9, 9 1/4 or 1/4", part.trim()))?;
                    let lead = part.chars().take_while(|c| c.is_whitespace()).count();
                    p.toks.push((at + lead, part.trim().chars().count(), Tok::Number));
                    at += n + 1;
                    v.push(m);
                }
                p.i = p.w.len();
                Stmt::Answer(v)
            }
            _ => unreachable!(),
        };
        p.done()?;
        Ok(s)
    })();
    (r, p.toks)
}

/// Split off a trailing quotation (the source's words for this operation).
fn split_quote(s: &str) -> (&str, Option<(usize, String)>) {
    let t = s.trim_end();
    for (close, open) in [('"', '"'), ('”', '“'), ('」', '「')] {
        if t.ends_with(close) {
            let body = &t[..t.len() - close.len_utf8()];
            if let Some(i) = body.rfind(open) {
                let quote = body[i + open.len_utf8()..].to_string();
                return (&s[..i], Some((s[..i].chars().count(), quote)));
            }
        }
    }
    (s, None)
}

pub fn parse_line(text: &str) -> Line {
    let mut toks = vec![];
    let trimmed = text.trim();
    let lead = text.chars().take_while(|c| c.is_whitespace()).count();
    let len = text.chars().count();
    if trimmed.is_empty() {
        return Line { text: text.into(), kind: Kind::Blank, toks, quote: None, note: None };
    }
    if trimmed.starts_with('%') {
        toks.push((lead, len - lead, Tok::Note));
        return Line { text: text.into(), kind: Kind::Note, toks, quote: None, note: Some(lead) };
    }
    if let Some(h) = trimmed.strip_prefix("No.") {
        toks.push((lead, trimmed.chars().count(), Tok::Heading));
        return Line { text: text.into(), kind: Kind::Heading(h.trim().into()), toks, quote: None, note: None };
    }
    // a trailing note
    let (body, note) = match text.find(" %") {
        Some(i) => (&text[..i], Some(text[..i].chars().count() + 1)),
        None => (text, None),
    };
    if let Some(n) = note {
        toks.push((n, len - n, Tok::Note));
    }
    let bt = body.trim();
    if bt.starts_with('"') || bt.starts_with('“') || bt.starts_with('「') {
        toks.push((lead, bt.chars().count(), Tok::Quote));
        let inner = bt.trim_matches(|c| c == '"' || c == '“' || c == '”' || c == '「' || c == '」');
        return Line { text: text.into(), kind: Kind::Words(inner.into()), toks, quote: None, note };
    }
    let (body, quote) = split_quote(body);
    if let Some((at, q)) = &quote {
        toks.push((*at, q.chars().count() + 2, Tok::Quote));
    }
    // readings in the alternative
    let mut alts = vec![];
    let mut at = 0usize;
    let mut err = None;
    for (k, part) in body.split('|').enumerate() {
        if k > 0 {
            toks.push((at - 1, 1, Tok::Bar));
        }
        let (r, t) = stmt(part, at);
        toks.extend(t);
        match r {
            Ok(s) => alts.push(s),
            Err(e) => {
                err.get_or_insert(e);
            }
        }
        at += part.chars().count() + 1;
    }
    toks.sort_by_key(|t| t.0);
    let kind = match err {
        Some(e) => Kind::Bad(e),
        None => Kind::Stmt(alts),
    };
    Line { text: text.into(), kind, toks, quote: quote.map(|q| q.1), note }
}

pub fn parse(text: &str) -> Vec<Line> {
    text.lines().map(parse_line).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    fn one(s: &str) -> Stmt {
        match parse_line(s).kind {
            Kind::Stmt(mut v) => v.remove(0),
            k => panic!("{:?}", k),
        }
    }
    #[test]
    fn statements() {
        assert_eq!(one("Set right 3 2 1 39"), Stmt::Set(ColRef::Right, vec![3, 2, 1, 39]));
        assert_eq!(
            one("Multiply middle by the head of right."),
            Stmt::Multiply(ColRef::Middle, Factor::Of(Pos::Head, ColRef::Right))
        );
        assert_eq!(
            one("Take right from middle until the head is gone."),
            Stmt::Take { a: ColRef::Right, t: ColRef::Middle, how: How::Until(Pos::Head) }
        );
        assert_eq!(
            one("Take 3rd from 4th as far as it goes"),
            Stmt::Take { a: ColRef::Nth(3), t: ColRef::Nth(4), how: How::AsFar(Pos::Head) }
        );
        assert_eq!(one("Take 第三行 from 左行"), Stmt::Take { a: ColRef::Nth(3), t: ColRef::Left, how: How::Once });
        assert_eq!(one("Solve by cross."), Stmt::Solve(Method::Cross));
        assert_eq!(one("Gives right 0 33 -45 3000"), Stmt::Gives(ColRef::Right, vec![0, 33, -45, 3000]));
    }
    #[test]
    fn answers() {
        match one("Answer 9 1/4, 4 1/4, 2 3/4") {
            Stmt::Answer(v) => {
                assert_eq!(v.len(), 3);
                assert_eq!(v[0], Mixed { neg: false, whole: 9, num: 1, den: 4 });
            }
            s => panic!("{:?}", s),
        }
    }
    #[test]
    fn alternatives_quotes_and_notes() {
        let l = parse_line(r#"Take 3rd from 4th | Take 4th from 3rd.  "以減第四行" % corrupt?"#);
        match &l.kind {
            Kind::Stmt(v) => assert_eq!(v.len(), 2),
            k => panic!("{:?}", k),
        }
        assert_eq!(l.quote.as_deref(), Some("以減第四行"));
        assert!(l.note.is_some());
    }
    #[test]
    fn bad() {
        assert!(matches!(parse_line("Take right from").kind, Kind::Bad(_)));
        assert!(matches!(parse_line("Frobnicate right").kind, Kind::Bad(_)));
    }
}
