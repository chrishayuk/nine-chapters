//! The pages the independent review wrote to make the checker give a wrong
//! mark, each with what it must do now. See docs/independent-review.md.

use jiuzhang::check::{run, Mark, Severity};
use jiuzhang::readings;

fn page(name: &str) -> String {
    std::fs::read_to_string(format!("{}/tests/review/{}.jz", env!("CARGO_MANIFEST_DIR"), name)).unwrap()
}

fn rights(name: &str) -> usize {
    run(&page(name)).checked.0
}

#[test]
fn a_contradictory_board_is_not_solved() {
    let r = run(&page("inconsistent"));
    assert!(r.stopped.is_some());
    assert!(r.findings.iter().any(|f| f.message.contains("contradict")));
    assert_eq!(rights("inconsistent"), 0);
}

#[test]
fn an_answer_is_of_the_board_it_was_found_on() {
    let r = run(&page("stale"));
    assert!(r.findings.iter().any(|f| f.message.contains("`Solve` first")));
    assert_eq!(rights("stale"), 0);
}

#[test]
fn an_answer_with_the_wrong_number_of_figures_is_wrong() {
    let r = run(&page("arity"));
    assert!(r
        .findings
        .iter()
        .any(|f| f.severity == Severity::Wrong && f.message.contains("figures")));
}

#[test]
fn a_form_no_reading_writes_is_wrong() {
    let r = run(&page("form"));
    let line = r
        .lines
        .iter()
        .find(|l| l.line.text.starts_with("Answer 1 36/104"))
        .unwrap();
    assert!(line
        .marks
        .iter()
        .any(|m| matches!(m, Mark::Wrong(w) if w.contains("written over 104"))));
}

#[test]
fn a_figure_too_large_is_said_to_be() {
    let r = run(&page("overflow"));
    assert!(r.findings.iter().any(|f| f.message.contains("grows past")));
    assert_eq!(r.checked.1, 0, "no false ✗");
}

#[test]
fn the_census_does_not_count_what_it_did_not_run() {
    let c = readings::all(&page("census-count")).unwrap();
    assert_eq!(c.executable, 0);
}

#[test]
fn right_and_1st_are_one_column() {
    let r = run(&page("alias"));
    assert_eq!(r.board.as_ref().unwrap().n(), 2);
}
