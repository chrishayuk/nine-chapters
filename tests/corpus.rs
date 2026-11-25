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
    // each take done singly: the whole column 86, the best interleaving still 77
    assert_eq!((rods.singly_whole, rods.singly_best), (86, 77));
    let guo = jiuzhang::corpus::find("guo-18-old").unwrap().1;
    let r = jiuzhang::check::run(guo);
    assert!(r.stopped.is_none());
    assert_eq!(jiuzhang::count::count(jiuzhang::count::Rule::Guo, &r.ops), 67);
}

#[test]
fn guo_124_runs_and_peaks_at_124_rods() {
    let page = jiuzhang::corpus::find("guo-18-new").unwrap().1;
    let r = jiuzhang::check::run(page);
    assert!(r.stopped.is_none());
    assert_eq!(r.checked, (7, 0), "every figure the text and Guo's notes state");
    // the last board, as docs/guo-77.md reconstructs state (21), the 2nd with the sign as added
    let b = r.board.unwrap();
    let col = |i: usize| {
        b.cols[i]
            .clone()
            .map(|c| c.iter().map(|&x| x as i64).collect::<Vec<_>>())
    };
    assert_eq!(col(0), Some(vec![0, 0, -2, 0, 1, 0]));
    assert_eq!(col(1), Some(vec![-4, 7, 0, 0, 0, 0]));
    assert_eq!(col(2), None);
    assert_eq!(col(3), Some(vec![-1, 1, 1, 0, 0, 0]));
    assert_eq!(col(4), Some(vec![0, 1, -3, 1, 0, 0]));
    let rods = jiuzhang::report::rods_at(page);
    assert_eq!(
        (rods.ends, rods.whole, rods.top_down, rods.bottom_up, rods.best),
        (124, 124, 130, 124, 124)
    );
    // each take done singly: 133 on the whole column, and no interleaving
    // of the single takes keeps to 124; the 3-times step needs 125
    assert_eq!((rods.singly_whole, rods.singly_best), (133, 125));
    let over: Vec<_> = rods
        .singly_over
        .iter()
        .filter(|o| o.2 > 124)
        .map(|o| (o.1.as_str(), o.2))
        .collect();
    assert_eq!(over.len(), 1);
    assert!(over[0].0.starts_with("Take 3rd from left 3 times"));
    assert_eq!(over[0].1, 125);
    assert_eq!(jiuzhang::count::count(jiuzhang::count::Rule::Guo, &r.ops), 228);
}

#[test]
fn least_peak_searches_every_interleaving() {
    // two positions that each dip before they rise: worked one after the
    // other the board reaches 7; taking both down first, it never passes 5
    assert_eq!(
        jiuzhang::report::least_peak(0, &[vec![2, 0, 5, 0], vec![2, 0, 5, 0]]),
        5
    );
    // the 3-times step of guo-18-new, the left column's figures taken singly
    let fig = |n: i64| jiuzhang::board::Board::rods(n as jiuzhang::board::Num);
    let left = [-22i64, 3, -90, 77, 5, 3];
    let third = [23i64, -3, 94, -80, -5, 1];
    let chains: Vec<Vec<u32>> = (0..6)
        .map(|p| (0..4).map(|i| fig(left[p] - i * third[p])).collect())
        .collect();
    assert_eq!(jiuzhang::report::least_peak(98, &chains), 125);
    // one figure (麻) taken three times at once is enough for 124
    let mut one = chains.clone();
    one[0] = vec![fig(-22), fig(-22 - 3 * 23)];
    assert_eq!(jiuzhang::report::least_peak(98, &one), 124);
}
