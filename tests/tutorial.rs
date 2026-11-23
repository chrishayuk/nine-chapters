//! Every page and every command in the tutorial is run, and what it prints is
//! compared with what the tutorial says it prints. `JIUZHANG_BLESS=1 cargo test`
//! writes what it prints into the tutorial instead.
//!
//! A page is a fence opened with ```jz (or ```jz --plain), followed by a
//! fence holding its output. A command is a line `$ jiuzhang …` in a
//! ```console fence; its output is the lines after it, up to the next `$`
//! line or the end of the fence.

use std::path::{Path, PathBuf};
use std::process::Command;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn jiuzhang(args: &[String]) -> String {
    let out = Command::new(env!("CARGO_BIN_EXE_jiuzhang"))
        .args(args)
        .current_dir(root())
        .output()
        .expect("jiuzhang runs");
    let mut s = String::from_utf8(out.stdout).unwrap();
    s.push_str(&String::from_utf8(out.stderr).unwrap());
    s.trim_end().to_string()
}

/// Split a command line on spaces: the tutorial uses no quoting.
fn words(line: &str) -> Vec<String> {
    line.split_whitespace().skip(1).map(String::from).collect()
}

/// The chapter with every output worked out afresh. `name` keeps the
/// chapters' page files apart.
fn worked(name: &str, text: &str) -> String {
    let lines: Vec<&str> = text.lines().collect();
    let mut out: Vec<String> = Vec::new();
    let mut pages = 0;
    let mut i = 0;
    while i < lines.len() {
        let line = lines[i];
        if line == "```jz" || line.starts_with("```jz ") {
            // the page, then its output in the next fence
            let flags: Vec<String> = line["```jz".len()..].split_whitespace().map(String::from).collect();
            let mut page = String::new();
            out.push(line.to_string());
            i += 1;
            while lines[i] != "```" {
                page.push_str(lines[i]);
                page.push('\n');
                out.push(lines[i].to_string());
                i += 1;
            }
            out.push("```".into());
            i += 1;
            while lines[i].trim().is_empty() {
                out.push(lines[i].to_string());
                i += 1;
            }
            assert!(
                lines[i].starts_with("```"),
                "{}: a page must be followed by its output",
                name
            );
            out.push(lines[i].to_string());
            i += 1;
            while lines[i] != "```" {
                i += 1;
            }
            pages += 1;
            let file = Path::new(env!("CARGO_TARGET_TMPDIR")).join(format!("{}-{}.jz", name, pages));
            std::fs::write(&file, &page).unwrap();
            let mut args = flags;
            args.push(file.to_string_lossy().into_owned());
            out.push(jiuzhang(&args));
            out.push("```".into());
            i += 1;
            continue;
        }
        if line == "```console" {
            out.push(line.to_string());
            i += 1;
            while lines[i] != "```" {
                if let Some(cmd) = lines[i].strip_prefix("$ ") {
                    assert!(cmd.starts_with("jiuzhang"), "{}: only jiuzhang is run: {}", name, cmd);
                    out.push(lines[i].to_string());
                    let got = jiuzhang(&words(cmd));
                    if !got.is_empty() {
                        out.push(got);
                    }
                    i += 1;
                    while lines[i] != "```" && !lines[i].starts_with("$ ") {
                        i += 1;
                    }
                } else {
                    out.push(lines[i].to_string());
                    i += 1;
                }
            }
            out.push("```".into());
            i += 1;
            continue;
        }
        out.push(line.to_string());
        i += 1;
    }
    let mut s = out.join("\n");
    s.push('\n');
    s
}

#[test]
fn every_page_and_command_in_the_tutorial() {
    let dir = root().join("tutorial");
    let mut chapters: Vec<_> = std::fs::read_dir(&dir)
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| p.extension().map(|e| e == "md").unwrap_or(false))
        .collect();
    chapters.sort();
    assert!(chapters.len() >= 10);
    // `counts` and `readings` take a while; the chapters run side by side.
    let runs: Vec<_> = chapters
        .into_iter()
        .map(|path| {
            std::thread::spawn(move || {
                let text = std::fs::read_to_string(&path).unwrap();
                let name = path.file_stem().unwrap().to_string_lossy().into_owned();
                let fresh = worked(&name, &text);
                (path, text, fresh)
            })
        })
        .collect();
    let mut failures = vec![];
    for run in runs {
        let (path, text, fresh) = run.join().unwrap();
        if std::env::var("JIUZHANG_BLESS").is_ok() {
            if fresh != text {
                std::fs::write(&path, &fresh).unwrap();
            }
            continue;
        }
        if fresh != text {
            let at = text.lines().zip(fresh.lines()).position(|(a, b)| a != b).unwrap_or(0);
            failures.push(format!(
                "{} line {}:\n  says: {}\n  gets: {}",
                path.display(),
                at + 1,
                text.lines().nth(at).unwrap_or(""),
                fresh.lines().nth(at).unwrap_or("")
            ));
        }
    }
    assert!(
        failures.is_empty(),
        "the tutorial and jiuzhang disagree (JIUZHANG_BLESS=1 to accept):\n{}",
        failures.join("\n")
    );
}
