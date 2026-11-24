//! The whole transcription, run: every page is in the notation, and
//! `verify`, `answers` and `counts` print what they printed when last
//! accepted. `JIUZHANG_BLESS=1 cargo test` accepts a change, which then shows
//! in the diff.

use std::process::Command;

fn jiuzhang(args: &[&str]) -> String {
    let out = Command::new(env!("CARGO_BIN_EXE_jiuzhang"))
        .args(args)
        .output()
        .expect("jiuzhang runs");
    String::from_utf8(out.stdout).unwrap()
}

fn expect(name: &str, got: &str) {
    let path = format!("{}/tests/expected/{}.txt", env!("CARGO_MANIFEST_DIR"), name);
    if std::env::var("JIUZHANG_BLESS").is_ok() {
        std::fs::write(&path, got).unwrap();
        return;
    }
    let want = std::fs::read_to_string(&path).unwrap_or_default();
    if want != got {
        let first = want.lines().zip(got.lines()).position(|(a, b)| a != b).unwrap_or(0);
        panic!(
            "`jiuzhang {}` has changed (JIUZHANG_BLESS=1 to accept). First difference at line {}:\n  was: {}\n  now: {}",
            name,
            first + 1,
            want.lines().nth(first).unwrap_or(""),
            got.lines().nth(first).unwrap_or("")
        );
    }
}

#[test]
fn every_page_is_in_the_notation() {
    for (id, text) in jiuzhang::corpus::CORPUS {
        let run = jiuzhang::check::run(text);
        let bad: Vec<_> = run
            .findings
            .iter()
            .filter(|f| f.severity == jiuzhang::check::Severity::Notation)
            .collect();
        assert!(bad.is_empty(), "{}: {:?}", id, bad);
    }
}

#[test]
fn verify() {
    expect("verify", &jiuzhang(&["verify"]));
}

#[test]
fn answers() {
    expect("answers", &jiuzhang(&["answers"]));
}

#[test]
fn counts() {
    expect("counts", &jiuzhang(&["counts"]));
}

#[test]
fn the_findings() {
    let answers = jiuzhang(&["answers"]);
    assert!(answers.contains("Of 11 answers with a fraction: board writes 7 as the text does, 約 11, lowest terms 9."));
    let counts = jiuzhang(&["counts"]);
    assert!(counts.contains("Count 145  plain     145   ✓"));
    assert!(counts.contains("liu-hui-18-old: Liu Hui says 77 算. Of 100000000 readings"));
    assert!(counts.contains("and 0 agree"));
    // the two published readings of Liu Hui's 77
    assert!(
        counts.contains("on his emended text: runs to the prices; under his rule 67 operations, leaving 10 of the 77")
    );
    assert!(counts.contains("every count of rods he reports agrees; at the end of each step 77 rods at most"));
    assert!(counts.contains("a position at a time top down 78, bottom up 77, in the best order 77"));
}

#[test]
fn hart_rods_and_guo_steps() {
    let hart = jiuzhang::corpus::find("hart-18-old").unwrap().1;
    let r = jiuzhang::check::run(hart);
    assert_eq!(r.checked, (16, 0), "fifteen of Hart's counts of rods and the 黍");
    let rods = jiuzhang::report::rods_at(hart);
    assert_eq!(
        (rods.ends, rods.whole, rods.top_down, rods.bottom_up, rods.best),
        (77, 86, 78, 77, 77)
    );
    let guo = jiuzhang::corpus::find("guo-18-old").unwrap().1;
    let r = jiuzhang::check::run(guo);
    assert!(r.stopped.is_none());
    assert_eq!(jiuzhang::count::count(jiuzhang::count::Rule::Guo, &r.ops), 67);
}
