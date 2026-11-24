//! A language server for `.jz` pages, over standard input and output
//! (`jiuzhang lsp`). Problems as the text changes: figures the source states
//! that the board does not give, operations that cannot be done, lines not
//! in the notation. Hover on an operation shows what it did to each position,
//! by the rule of signs, and the board after it. Colours come as semantic
//! tokens, so the same server colours the editor in VS Code and on the web.

use crate::board::{col_name, col_name_zh, Board, Ev};
use crate::check::{self, Mark, Run, Severity};
use crate::count::{self, RULES};
use crate::json::{self, obj, Json};
use crate::page::{Kind, Stmt, Tok};
use std::collections::HashMap;
use std::io::{BufRead, Write};

pub const TOKEN_TYPES: &[&str] = &[
    "comment",
    "string",
    "keyword",
    "number",
    "operator",
    "namespace",
    "variable",
    "macro",
];

fn tok_type(t: Tok) -> usize {
    match t {
        Tok::Note => 0,
        Tok::Quote => 1,
        Tok::Keyword => 2,
        Tok::Number => 3,
        Tok::Bar | Tok::Word => 4,
        Tok::Column => 5,
        Tok::Thing => 6,
        Tok::Heading => 7,
    }
}

fn read_message(input: &mut impl BufRead) -> Option<Json> {
    let mut length = None;
    loop {
        let mut line = String::new();
        if input.read_line(&mut line).ok()? == 0 {
            return None;
        }
        let line = line.trim_end();
        if line.is_empty() {
            break;
        }
        if let Some((name, value)) = line.split_once(':') {
            if name.eq_ignore_ascii_case("content-length") {
                length = value.trim().parse::<usize>().ok();
            }
        }
    }
    let mut body = vec![0; length?];
    input.read_exact(&mut body).ok()?;
    json::parse(&String::from_utf8_lossy(&body)).ok()
}

fn send(output: &mut impl Write, message: Json) {
    let mut body = String::new();
    message.write(&mut body);
    let _ = write!(output, "Content-Length: {}\r\n\r\n{}", body.len(), body);
    let _ = output.flush();
}

fn u16_col(text: &str, c: usize) -> usize {
    text.chars().take(c).map(|ch| ch.len_utf16()).sum()
}

fn position(line: usize, character: usize) -> Json {
    obj([("line", line.into()), ("character", character.into())])
}

fn range(line: usize, start: usize, end: usize) -> Json {
    obj([("start", position(line, start)), ("end", position(line, end))])
}

fn diagnostic_list(run: &Run) -> Vec<Json> {
    run.findings
        .iter()
        .map(|f| {
            let text = &run.lines[f.line].line.text;
            let (s, e) = (u16_col(text, f.span.0), u16_col(text, f.span.1.max(f.span.0 + 1)));
            obj([
                ("range", range(f.line, s, e)),
                (
                    "severity",
                    match f.severity {
                        Severity::Notation => 1,
                        Severity::Wrong | Severity::Fails => 2,
                        Severity::Form => 4,
                    }
                    .into(),
                ),
                ("source", "jiuzhang".into()),
                ("message", f.message.clone().into()),
            ])
        })
        .collect()
}

fn diagnostics(uri: &str, text: &str) -> Json {
    let run = check::run(text);
    obj([
        ("jsonrpc", "2.0".into()),
        ("method", "textDocument/publishDiagnostics".into()),
        (
            "params",
            obj([("uri", uri.into()), ("diagnostics", diagnostic_list(&run).into())]),
        ),
    ])
}

fn capabilities() -> Json {
    obj([
        (
            "capabilities",
            obj([
                ("textDocumentSync", 1.into()),
                ("hoverProvider", true.into()),
                ("documentSymbolProvider", true.into()),
                (
                    "semanticTokensProvider",
                    obj([
                        (
                            "legend",
                            obj([
                                (
                                    "tokenTypes",
                                    TOKEN_TYPES.iter().map(|t| Json::from(*t)).collect::<Vec<_>>().into(),
                                ),
                                ("tokenModifiers", Vec::new().into()),
                            ]),
                        ),
                        ("full", true.into()),
                    ]),
                ),
                ("completionProvider", obj([("resolveProvider", false.into())])),
            ]),
        ),
        (
            "serverInfo",
            obj([
                ("name", "jiuzhang".into()),
                ("version", env!("CARGO_PKG_VERSION").into()),
            ]),
        ),
    ])
}

/// (line, start, length, type) in UTF-16 columns, in order.
pub fn semantic(run: &Run) -> Vec<(usize, usize, usize, usize)> {
    let mut out = vec![];
    for (i, l) in run.lines.iter().enumerate() {
        let mut toks = l.line.toks.clone();
        toks.sort_by_key(|t| t.0);
        for (s, len, k) in toks {
            let a = u16_col(&l.line.text, s);
            let b = u16_col(&l.line.text, s + len);
            if b > a {
                out.push((i, a, b - a, tok_type(k)));
            }
        }
    }
    out
}

fn semantic_tokens(text: &str) -> Json {
    let run = check::run(text);
    let mut data = Vec::new();
    let (mut last_line, mut last_start) = (0, 0);
    for (line, start, length, kind) in semantic(&run) {
        let delta = if line == last_line { start - last_start } else { start };
        data.extend([line - last_line, delta, length, kind, 0].map(Json::from));
        (last_line, last_start) = (line, start);
    }
    obj([("data", data.into())])
}

/// What one taking did at one position, in the words of the 正負術.
fn sign_rule(before: i128, taken: i128) -> &'static str {
    // `taken` is what was taken away (negative: added)
    if before == 0 {
        if taken > 0 {
            "無入: nothing there; the figure taken is set down with the other sign (正無入負之 / 負無入正之)"
        } else {
            "無入: nothing there; the figure added is set down as it is"
        }
    } else if (before > 0) == (taken > 0) {
        "同名相除: same sign, taken away"
    } else {
        "異名相益: opposite signs, added together"
    }
}

pub const KEYWORD_HELP: &[(&str, &str)] = &[
    ("Things", "`Things A B C 實` names the positions of a column, top to bottom; the last is the 實."),
    ("Set", "`Set right 3 2 1 39` sets a column (置). Columns are counted from the right: right, middle, left; or right, 2nd, 3rd, …, left."),
    ("Multiply", "`Multiply middle by the head of right` multiplies the whole of a column (徧乘)."),
    ("Take", "`Take right from middle` takes one column from another once (減); `… until the head is gone` as many times as empties the head (直除); `… as far as it goes` while it goes (不足減乃止); `… crosswise` after cross-multiplying (互乘); `… 3 times` a stated number of times."),
    ("Add", "`Add right to middle` adds one column to another (益, 并)."),
    ("Halve", "`Halve middle` halves a column (半)."),
    ("Reduce", "`Reduce middle` divides a column by what its figures share (約)."),
    ("Divide", "`Divide middle by 3` divides every figure of a column."),
    ("Drop", "`Drop 3rd` takes a column off the board (廢去)."),
    ("Gives", "`Gives middle 0 5 1 24`: the source says the column now stands so. Checked."),
    ("Solve", "`Solve` does the rest of the procedure (方程術); `Solve by reducing|cross|one` by another reading."),
    ("Answer", "`Answer 9 1/4, 4 1/4, 2 3/4`: the answer as the text writes it. Checked, and how it is written compared with each reading of the last step."),
    ("Rates", "`Rates 721 265 …`: an answer given as rates (率): the 法, then the 實s, reduced together. Checked."),
    ("Rods", "`Rods 77`: the source says the board now holds 77 rods (a digit to five is that many rods, six to nine one for five and the rest). Checked."),
    ("Count", "`Count 77`: the source says the operations since the last count take so many 算. Compared under every rule."),
];

pub fn hover(text: &str, line: usize, col16: usize) -> Option<String> {
    let run = check::run(text);
    let l = run.lines.get(line)?;
    // char column
    let mut col = 0;
    let mut acc = 0;
    for c in l.line.text.chars() {
        if acc >= col16 {
            break;
        }
        acc += c.len_utf16();
        col += 1;
    }
    let mut out = String::new();
    if let Kind::Stmt(v) = &l.line.kind {
        let s = &v[l.choice];
        // on the keyword: what it means
        if let Some((start, len, Tok::Keyword)) = l
            .line
            .toks
            .iter()
            .find(|t| t.2 == Tok::Keyword && col >= t.0 && col < t.0 + t.1)
            .copied()
        {
            let _ = (start, len);
            if let Some((_, h)) = KEYWORD_HELP.iter().find(|k| k.0 == s.keyword()) {
                out.push_str(h);
                out.push_str("\n\n");
            }
        }
        if v.len() > 1 {
            out.push_str(&format!(
                "Read in {} ways; this run takes the first. `jiuzhang readings` runs them all.\n\n",
                v.len()
            ));
        }
        let mut prev: Option<Board> = run.lines[..line]
            .iter()
            .rev()
            .find_map(|x| x.steps.last().map(|s| s.board.clone()));
        for st in &l.steps {
            let n = st.board.n();
            out.push_str(&format!("**{}**", st.op.what));
            if let Some(a) = st.op.actor {
                out.push_str(&format!(" ({} → {})", col_name_zh(a, n), col_name_zh(st.op.target, n)));
            }
            out.push('\n');
            if let Some(p) = &prev {
                for ev in &st.op.events {
                    if let (Ev::Take { pos, times, .. }, Some(a)) = (ev, st.op.actor) {
                        let before = p.col(st.op.target).map(|c| c[*pos]).unwrap_or(0);
                        let ac = p.col(a).map(|c| c[*pos]).unwrap_or(0);
                        let after = st.board.col(st.op.target).map(|c| c[*pos]).unwrap_or(0);
                        out.push_str(&format!(
                            "- {}: {} → {} — {}\n",
                            st.board.thing(*pos),
                            before,
                            after,
                            sign_rule(before, ac * times)
                        ));
                    }
                }
            }
            let tally: Vec<String> = RULES
                .iter()
                .map(|r| format!("{} {}", r.name(), count::count(*r, std::slice::from_ref(&st.op))))
                .collect();
            out.push_str(&format!("\n算: {}\n\n```\n{}```\n", tally.join(", "), st.board));
            prev = Some(st.board.clone());
        }
        if matches!(s, Stmt::Solve(_)) {
            if let Some(sol) = &l.solution {
                let ops = sol.ops();
                let tally: Vec<String> = RULES
                    .iter()
                    .map(|r| format!("{} {}", r.name(), count::count(*r, &ops)))
                    .collect();
                out = format!(
                    "**{}**: {}\n\n{} steps; 算: {}\n\n",
                    sol.method.name(),
                    sol.method.describe(),
                    sol.steps.len(),
                    tally.join(", ")
                ) + &out;
            }
        }
        for m in &l.marks {
            match m {
                Mark::Right => out.push_str("\n✓ as the source says\n"),
                Mark::Wrong(w) => out.push_str(&format!("\n✗ {}\n", w)),
                Mark::Info(i) => out.push_str(&format!("\n{}\n", i)),
            }
        }
        if let Some(q) = &l.line.quote {
            out.push_str(&format!("\n> {}\n", q));
        }
    }
    let _ = col_name;
    if out.is_empty() {
        None
    } else {
        Some(out)
    }
}

fn completion() -> Json {
    let mut items: Vec<Json> = KEYWORD_HELP
        .iter()
        .map(|(k, d)| obj([("label", (*k).into()), ("kind", 14.into()), ("detail", (*d).into())]))
        .collect();
    for c in [
        "right", "middle", "left", "2nd", "3rd", "4th", "5th", "右行", "中行", "左行",
    ] {
        items.push(obj([
            ("label", c.into()),
            ("kind", 6.into()),
            ("detail", "a column".into()),
        ]));
    }
    for w in [
        "until the head is gone",
        "as far as it goes",
        "crosswise",
        "by the head of",
    ] {
        items.push(obj([("label", w.into()), ("kind", 15.into())]));
    }
    items.into()
}

fn symbols(text: &str) -> Json {
    let run = check::run(text);
    let heads: Vec<(usize, String)> = run
        .lines
        .iter()
        .enumerate()
        .filter_map(|(i, l)| match &l.line.kind {
            Kind::Heading(h) => Some((i, h.clone())),
            _ => None,
        })
        .collect();
    heads
        .iter()
        .enumerate()
        .map(|(k, (i, h))| {
            let last = heads
                .get(k + 1)
                .map(|x| x.0.saturating_sub(1))
                .unwrap_or(run.lines.len().saturating_sub(1));
            let w = |n: usize| run.lines[n].line.text.encode_utf16().count();
            obj([
                ("name", format!("No. {}", h).into()),
                ("kind", 12.into()),
                (
                    "range",
                    obj([("start", position(*i, 0)), ("end", position(last, w(last)))]),
                ),
                ("selectionRange", range(*i, 0, w(*i))),
            ])
        })
        .collect::<Vec<_>>()
        .into()
}

fn answer(method: &str, params: &Json, docs: &HashMap<String, String>) -> Option<Json> {
    let uri = params.get("textDocument").get("uri").str().unwrap_or("");
    let text = docs.get(uri).map(String::as_str).unwrap_or("");
    let line = params.get("position").get("line").num().unwrap_or(0);
    let col = params.get("position").get("character").num().unwrap_or(0);
    Some(match method {
        "initialize" => capabilities(),
        "shutdown" => Json::Null,
        "textDocument/hover" => match hover(text, line, col) {
            Some(h) => obj([("contents", obj([("kind", "markdown".into()), ("value", h.into())]))]),
            None => Json::Null,
        },
        "textDocument/semanticTokens/full" => semantic_tokens(text),
        "textDocument/documentSymbol" => symbols(text),
        "textDocument/completion" => completion(),
        _ => return None,
    })
}

pub fn serve(input: &mut impl BufRead, output: &mut impl Write) -> Result<(), String> {
    let mut docs: HashMap<String, String> = HashMap::new();
    let mut shutting_down = false;
    while let Some(message) = read_message(input) {
        let method = message.get("method").str().unwrap_or("").to_string();
        let params = message.get("params");
        let id = message.get("id").clone();
        let uri = params.get("textDocument").get("uri").str().unwrap_or("").to_string();
        match method.as_str() {
            "exit" => {
                return if shutting_down {
                    Ok(())
                } else {
                    Err("exit without shutdown".into())
                }
            }
            "textDocument/didOpen" => {
                let text = params.get("textDocument").get("text").str().unwrap_or("").to_string();
                send(output, diagnostics(&uri, &text));
                docs.insert(uri, text);
            }
            "textDocument/didChange" => {
                if let Some(text) = params
                    .get("contentChanges")
                    .items()
                    .last()
                    .and_then(|c| c.get("text").str())
                {
                    send(output, diagnostics(&uri, text));
                    docs.insert(uri, text.to_string());
                }
            }
            "textDocument/didClose" => {
                docs.remove(&uri);
                send(output, diagnostics(&uri, ""));
            }
            _ if id == Json::Null => {}
            _ => {
                shutting_down |= method == "shutdown";
                let reply = match answer(&method, params, &docs) {
                    Some(result) => obj([("jsonrpc", "2.0".into()), ("id", id), ("result", result)]),
                    None => obj([
                        ("jsonrpc", "2.0".into()),
                        ("id", id),
                        (
                            "error",
                            obj([
                                ("code", Json::Num(-32601.0)),
                                ("message", format!("method not found: {}", method).into()),
                            ]),
                        ),
                    ]),
                };
                send(output, reply);
            }
        }
    }
    Ok(())
}

fn board_json(b: &Board) -> Json {
    let n = b.n();
    obj([
        (
            "things",
            b.things
                .iter()
                .map(|t| Json::from(t.as_str()))
                .collect::<Vec<_>>()
                .into(),
        ),
        (
            "cols",
            b.cols
                .iter()
                .map(|c| match c {
                    Some(v) => v.iter().map(|x| Json::Num(*x as f64)).collect::<Vec<_>>().into(),
                    None => Json::Null,
                })
                .collect::<Vec<_>>()
                .into(),
        ),
        (
            "names",
            (0..n).map(|i| Json::from(col_name(i, n))).collect::<Vec<_>>().into(),
        ),
        ("rods", (b.rod_count() as usize).into()),
    ])
}

/// A run as JSON, for the playground: every line with its marks, and every
/// step with the board after it, so the board can be drawn and played.
pub fn run_json(text: &str) -> String {
    let run = check::run(text);
    let lines: Vec<Json> = run
        .lines
        .iter()
        .map(|l| {
            let kind = match &l.line.kind {
                Kind::Blank => "blank",
                Kind::Heading(_) => "heading",
                Kind::Words(_) => "words",
                Kind::Note => "note",
                Kind::Stmt(_) => "op",
                Kind::Bad(_) => "bad",
            };
            let marks: Vec<Json> = l
                .marks
                .iter()
                .map(|m| match m {
                    Mark::Right => obj([("kind", "right".into()), ("text", "✓".into())]),
                    Mark::Wrong(w) => obj([("kind", "wrong".into()), ("text", w.clone().into())]),
                    Mark::Info(i) => obj([("kind", "info".into()), ("text", i.clone().into())]),
                })
                .collect();
            let steps: Vec<Json> = l
                .steps
                .iter()
                .map(|s| {
                    obj([
                        ("what", s.op.what.clone().into()),
                        ("target", s.op.target.into()),
                        ("actor", s.op.actor.map(Json::from).unwrap_or(Json::Null)),
                        (
                            "count",
                            obj([
                                (
                                    "plain",
                                    (count::count(count::Rule::Plain, std::slice::from_ref(&s.op)) as usize).into(),
                                ),
                                (
                                    "worked",
                                    (count::count(count::Rule::Worked, std::slice::from_ref(&s.op)) as usize).into(),
                                ),
                            ]),
                        ),
                        ("board", board_json(&s.board)),
                    ])
                })
                .collect();
            obj([
                ("text", l.line.text.clone().into()),
                ("kind", kind.into()),
                ("quote", l.line.quote.clone().map(Json::from).unwrap_or(Json::Null)),
                ("marks", marks.into()),
                ("steps", steps.into()),
            ])
        })
        .collect();
    let findings: Vec<Json> = run
        .findings
        .iter()
        .map(|f| {
            obj([
                ("line", f.line.into()),
                (
                    "severity",
                    match f.severity {
                        Severity::Notation => "notation",
                        Severity::Wrong => "wrong",
                        Severity::Fails => "fails",
                        Severity::Form => "note",
                    }
                    .into(),
                ),
                ("message", f.message.clone().into()),
            ])
        })
        .collect();
    let peak = run
        .lines
        .iter()
        .flat_map(|l| l.steps.iter())
        .map(|s| s.board.rod_count())
        .max()
        .unwrap_or(0);
    let widest = run
        .lines
        .iter()
        .flat_map(|l| l.steps.iter())
        .map(|s| s.board.widest())
        .max()
        .unwrap_or(0);
    let mut out = String::new();
    obj([
        ("title", run.title().unwrap_or_default().into()),
        ("lines", lines.into()),
        ("findings", findings.into()),
        ("right", run.checked.0.into()),
        ("wrong", run.checked.1.into()),
        ("stopped", run.stopped.map(Json::from).unwrap_or(Json::Null)),
        ("peakRods", (peak as usize).into()),
        ("widest", widest.into()),
        ("text", check::render(&run, false).into()),
    ])
    .write(&mut out);
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    const P: &str = "No. 8.1\nThings 上 中 下 實\nSet right 3 2 1 39\nSet middle 2 3 1 34\nSet left 1 2 3 26\nMultiply middle by the head of right.\nTake right from middle until the head is gone.\nGives middle 0 5 1 24\n";
    #[test]
    fn hover_and_tokens() {
        let h = hover(P, 6, 1).unwrap();
        assert!(h.contains("同名相除"), "{}", h);
        assert!(h.contains("直除"), "{}", h);
        let toks = semantic(&check::run(P));
        assert!(toks.iter().any(|t| t.3 == 7));
        assert!(toks.iter().any(|t| t.3 == 5));
    }
    #[test]
    fn json() {
        let j = run_json(P);
        assert!(j.contains("\"peakRods\""));
        assert!(json::parse(&j).is_ok());
    }
}
