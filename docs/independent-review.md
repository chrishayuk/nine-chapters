# Independent review

## Who reviewed, and what was checked

The repository at 8532977 was reviewed by OpenAI Codex CLI (codex-cli
0.166.0) on 23 November 2026. It is an independent second model, not a
person. It changed no file in the repository. Its report, as written, is
[`independent-review-report.md`](independent-review-report.md).

It built the program and ran all 21 tests, which passed. It checked against
the scans every `Answer` and `Rates` line of the eighteen problems, the signs
of Problem 6's procedure, every figure on the commentary pages, the whole of
the 77 passage, the four stated counts (77, 124, 145, 99), and three `Set`
lines of its own choosing. It re-derived the answers table, the counts and
the workspace figures with a Python program of its own, re-ran both censuses
and inspected the pruning, tried a wider search for a working reading of the
77 passage, and wrote seven probe pages to make the checker fail.

Each finding below was checked again here, by running the probe or the
reviewer's arithmetic, before anything was changed.

## What the scans showed

No mistranscription was found. Problem 6 does read 益實六斗正 and 益實一斗正;
the 77 passage does read 及減第三行 and 得空; the four counts are as
transcribed. The reviewer notes that 空 for 六 is a conjecture from the answer,
which the page already says.

## The findings, and what was done

### 1. A contradictory board got a ✓: fixed

Confirmed: a board setting x = 1 and x = 2 was solved and its answer ticked;
the column left over after elimination was never looked at. `Solve` now
requires every column not used to clear a position to be empty, and says the
conditions contradict one another if one is not. Test:
`tests/review.rs`, `a_contradictory_board_is_not_solved`.

### 2. An answer could be checked against an earlier board: fixed

Confirmed. A solution now belongs to the board it was found on: any
operation that changes the board discards it, and an `Answer` or `Rates`
with nothing solved since is an error ("`Solve` first"), which stops the
page. Test: `an_answer_is_of_the_board_it_was_found_on`.

### 3. An answer with the wrong number of figures got no mark: fixed

Confirmed: the error was made and thrown away. It is now ✗. Test:
`an_answer_with_the_wrong_number_of_figures_is_wrong`.

### 4. ✓ did not certify how a fraction is written: fixed

Confirmed: the notation says `Answer` checks the written form, but a form no
reading gives was only noted. It is now ✗ beside the value's ✓. No answer in
the corpus changes: every one is written as one of the readings writes it.
Test: `a_form_no_reading_writes_is_wrong`.

### 5. Fractions could overflow without saying so: fixed

Confirmed: the board's arithmetic was checked, the fractions of the last
step were not, and a release build wrapped. The last step now uses checked
fractions and says a figure is too large. Test: `a_figure_too_large_is_said_to_be`.

### 6. The workspace figures are of the recorded boards: reworded

Confirmed. The rods and places are measured on the board as it stands after
each step; the working of the last step (a product of 16 places in finding
the 荅) and the products of cross-multiplying are not on it. The README and
tutorial now say "the recorded boards", give the 16 places, and no longer say
the text's procedure "needs" 11 places. No allocation model for the
reckoner's working is attempted.

### 7. "180,403 takings" mislabelled: reworded

Confirmed: under the `repeated` rule it is 180,403 operations, 180,330 of
them takings of single figures, or 89,773 takings of a whole column. Both
are now given, as the reviewer computed them.

### 8. Dai Zhen's words do not define the rules: accepted; a rule added

Accepted. His economies are about multiplying; that dividing by one is free
is `worked`'s extension, not his; leaving out the heads' takings is this
project's convention. The reviewer proposed a uniform rule, `plain` with a
division by one read off rather than worked, which gives both 145 and 99. It
is added as `readoff`, labelled everywhere as proposed after the counts were
known, and not preferred. The tutorial's statement that any rule giving 99
"would be written to fit" is withdrawn: it was stronger than the evidence.
The tutorial also now quotes 依術得荅價 for the order of the last two
things; the reviewer found that the other order gives the same counts.

### 9. The census: the reading space is narrower than its size; a reporting bug: fixed and reworded

The pruning was found sound for the absence of agreement. But a combination
pruned after a *disagreement* was counted as able to run to the end without
being run. The census now counts as executable only what it ran, and reports
the rest as not known. The corpus figures do not change (the 77 passage's
only assertion is its last line). Test:
`the_census_does_not_count_what_it_did_not_run`.

The README now says the 100,000,000 are combinations of a small menu of
operations, not interpretations, lists what the menu leaves out (heads as
anything but the first figure of the column worked on; the columns set one at
a time with copies kept; other repairs of 及減第三行; mutual subtraction
ending in either column), and reports that the reviewer's own wider search
found no working reading either. The README's claim that the self-subtraction
was among the readings was wrong (it is not), and is gone.

### 10. Universal claims: reworded

Confirmed. The README and tutorial said every figure is checked; figures the
notation cannot hold (Dai Zhen's half and double depths, the sparrows'
figures and rates, the fractions of the new method's last part) are quoted,
not checked. They now say so. The 39 checked statements are now called
statements, not figures.

### The answers table

The reviewer reproduced 7, 11 and 9 of 11 and found the readings fairly
defined, and pointed out that the 約 reading gives the same denominators as
reducing each answer and writing them over their least common denominator,
so the table cannot say where a reduction was made. The README and tutorial
now say this.

### Not in the report: `right` and `1st`

One probe page, not among the numbered findings, set the right-hand column
under both names; the board was sized as three columns with an empty middle.
Fixed: they are one column. Test: `right_and_1st_are_one_column`.
