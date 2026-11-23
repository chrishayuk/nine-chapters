<!-- The reviewer's report, as written, on 8532977. Its working files (probes, programs, scan renders) were in the reviewer's scratch directory; the probe pages are kept in tests/review/. What was done about each finding is in independent-review.md. -->

# Independent review of jiuzhang

Reviewer: OpenAI Codex, GPT-6 family (the session identifies the family, not an exact model build). Date: 23 November 2026. Subject: `/Users/christopherhay/chris-source/esoteric/nine-chapters`, commit `853297727e467cc5ba88af8737f321fb200280f9`.

The repository was read only; final `git status --short` was empty. All programs, probes, image copies, logs, and build output are in this review directory. No subagents were used. This is an independent computational and scan review, not a review of the secondary historical literature.

**Result:** the requested numerical transcriptions agree with the scans. The answer-form table is reproducible. The principal defects are in what a checkmark guarantees, the scope of the counting/space claims, and the interpretation of the failed reading searches.

## What I read and ran

Read README first, then `docs/notation.md`, `sources/TRANSCRIBING.md`, all three `sources/notes/*.md`, `jiuzhang decisions`, every module in `src/`, the corpus, and the tutorial. Visually inspected the requested passages in the local JPEG scans, including page/column continuations; the ledger below records those checks. The unproofread paged text was used for navigation, not as independent confirmation of a figure.

Ran the requested command, from this directory:

```sh
CARGO_TARGET_DIR=$PWD/target cargo test --release --manifest-path /Users/christopherhay/chris-source/esoteric/nine-chapters/Cargo.toml
```

**Passed:** 15 library tests, 5 corpus tests, 1 tutorial test; 0 failures. Binary and doc-test suites contain 0 tests. Compilation took 1.79 s; corpus and tutorial suites reported 17.58 s and 17.59 s. Passing snapshots establish consistency with accepted output, not historical correctness or the absence of the probe failures below.

Also ran `decisions`, `answers`, `counts`, `verify`, both Liu Hui `readings` censuses, JSON output, and the probe pages. Saved outputs include `answers.txt`, `decisions.txt`, `verify.txt`, `census77.txt`, `census124.txt`, and `probe-*.txt`. `independent.py` uses Python arbitrary-size integers and `fractions.Fraction`; it imports no project code. `inspect.rs` and `audit-census.rs` separately inspect the project's recorded operations/census through its compiled library. Exploratory searches are `search77.py` and `search77-permute.py`.

## Findings, most serious first

### 1. High — a contradictory board receives a correct-answer checkmark

`probe-inconsistent.jz` sets `x = 1` and `x = 2`, calls `Solve`, and asserts `Answer 1`. It prints **✓**, exits 0, and reports one figure right. There is no solution.

`src/solve.rs:106` selects one pivot per unknown, but never validates unused columns after the last pivot. Here elimination leaves an unused column `[0, 1]`; the nonzero residual is discarded. Reject inconsistent residuals, and either support or explicitly reject excess conditions. The README's “not a solver” qualification does not make a false ✓ on an accepted page correct.

### 2. High — subsequent assertions can use a previous board's solution

`probe-stale.jz` solves `[1, 1]`, replaces the column with `[1, 2]`, then receives **✓** for both `Answer 1` and `Rates 1 1`, and **✗** for the now-correct `Answer 2`.

`src/check.rs:411` caches `sol`; board-changing statements never invalidate it, while `Answer`/`Rates` consult it (`:429` onward). Assertions can therefore certify the wrong board. Requiring another Solve would be reasonable, but the checker must report the missing solve instead of silently retaining a valid-looking old answer.

### 3. Medium — a malformed answer is silently accepted by the page and census

In `probe-arity.jz`, a two-unknown board is solved correctly, followed by `Answer 999`. The line gets no mark, no finding, and exit 0. `readings::all` counts this page as agreeing.

`check_answer` returns an error for the wrong number of entries (`src/check.rs:178`), but its caller discards the Result (`:431`). Unlike a value mismatch, the arity branch does not first emit a Wrong mark. This is a fail-open assertion, not merely a display omission.

### 4. Medium — ✓ does not certify the promised written fraction form

`probe-form.jz` solves Problem 2's board and asserts `Answer 1 36/104, 82/104`. It receives **✓**, exits 0, and is an agreeing census reading, while the same line says “no reading here writes it so.”

`src/check.rs:192` pushes Right on numerical equality; the later form analysis is only information. The documentation says Answer checks both value **and how each fraction is written** (`docs/notation.md`, “What the source states”; `tutorial/04-the-last-step.md:136`). The source writes `1 18/52, 41/52`; the probe form is supported by none of the three proposed rules. Distinguish a value check from a form check instead of giving a single unqualified ✓. The separate `answers` table does compare forms and is not invalidated by this defect.

### 5. Medium — release arithmetic can wrap and falsely reject an exact answer

`probe-overflow.jz` sets `[1, 0, 100000000000000000000]` and `[0, 100000000000000000000, 0]`. The exact answer is `(100000000000000000000, 0)`, but release execution prints **✗** and a spurious fractional value for x.

The board's checked integer operations do not protect `Rat` arithmetic. Back substitution forms `10^20 × 10^20` (`src/solve.rs:158`); `src/rat.rs` uses unchecked i128 products/additions. Release wrapping corrupts the result rather than returning TooLarge. All inputs and answers fit i128; the intermediate does not. This does not affect the small corpus results checked here, but contradicts exactness as a general checker property.

### 6. Medium — the workspace table measures stored snapshots, not the full computation

I reproduce the advertised snapshot maxima: text **160 rods / 11 places**, reducing **138 / 4**, cross **149 / 11**, one **122 / 4**. However, back-substitution steps append an unchanged board (`src/solve.rs:169`, `:191`), and the report measures only those boards (`src/report.rs:262`). It omits the working dividends, products, and accumulated 實s. Cross multiplication also stores only its final difference, assuming its other products need no board space (`src/board.rs:243`).

For the text method, 法 is 2,033,910,000. Finding 荅 computes:

```
法 × (-210600)       =  -428341446000000
72900 × 黍實        =   889632234000000
difference          = -1317973680000000   (16 digits)
difference / -129600 =      10169550000
```

The tutorial's “needs figures of 11 places” (`tutorial/08-counting.md:132`) is therefore false for the algorithm actually used. The rod totals are correct for the chosen stored arrays; they are not established maximum workspace requirements for a reckoner. A workspace claim needs an explicit allocation model for these missing quantities. I have not substituted an invented exact rod maximum for that missing model.

### 7. Medium — “180,403 takings” is the wrong description of the number

README `:161`–`:162` calls this literal successive column subtraction. The project's own repeated tally is **64 multiplications + 180,330 scalar takings + 9 divisions = 180,403**, not 180,403 takings.

The ten elimination multiplicities are `7, 3, 2, 1, 24, 31, 20, 360, 225, 89100`: **89,773 whole-column repetitions**. Moreover, repeated omits the known cancelling head positions (`src/count.rs:95`). Including those heads gives 270,093 scalar elimination takings; the project's 180,330 also includes ten back-substitution takings. Thus neither the unit nor “read literally” is accurate. The tutorial's more qualified description as a total under the `repeated` rule is better.

### 8. Medium — Dai Zhen's words do not define the full rules attributed to him

The scans really say **145** and **99**. Independent counts reproduce cross **145 plain / 140 worked** and shorter-way **101 plain / 98 worked**. The matching 145 is a result under a cost convention, not a uniquely determined historical count.

On p. 48, `可省乘` and `省其與對行徧乘` authorize economies in **multiplication**. They do not say that every division by ±1 is free. README `:149`–`:150` extends “which he says can be saved” to such divisions. Plain also omits known head **takings**, not just multiplications; this is a defensible additional convention, but is additional. Worked applies these economies during back substitution too. The text does not specify that scope.

`tutorial/08-counting.md:88`–`:94` says a rule retaining just one of the three unit operations would be written to fit 99. That is stronger than the evidence. A uniform alternative is **plain, but unit divisions are read off without a division operation**. It retains the explicit multiplication by −1 and omits the two divisions by ±1. It gives **145 and 99** on the two algorithms. This is a candidate cost model, not proof of Dai's intention; it shows why rejecting every such model as fitted is unwarranted. Plain/worked are intelligible hypotheses, but neither is a transcription of a fully specified counting rule. I found no evidence establishing deliberate numerical fabrication.

There is also an algorithmic choice: p. 49 says `依術得荅價`; `Solve by one` eliminates 荅 first and finds 黍 before 荅. The corpus notes this choice, but the tutorial says the order is unspecified. The passage explicitly names 荅 at this point. Reversing the final two variable positions independently still gives 101/98, so this particular choice does not explain the discrepancy.

### 9. Medium — census pruning preserves no-agreement, but the reading space is much narrower than its size suggests

I reproduce **100,000,000 combinations / 912,143 executed / 229,906 executable / zero agreeing** for the old passage, and **6,115,295,232 / 193 / zero executable / zero agreeing** for the new passage. These are combinations of a small menu of machine operations, not independent philological interpretations of all ambiguous words.

The prefix pruning in `src/readings.rs:102` is sound **for absence of agreement under the encoded semantics**: once a fixed prefix stops or emits a Wrong/Notation finding, changing later alternatives cannot undo that failure. It skips the Cartesian product of suffix choices and advances the odometer at the last alternative at or before the failing line. Board metadata is obtained from the same parsed page for all choices, so future choices do not undermine that prefix argument. The assertion is last in the old page; its reported executable total is sound too.

There is a separate reporting bug: an early *disagreement* does not imply the suffix can execute. `probe-census-count.jz` has a false Gives followed by either halving an odd column or dividing it inexactly. Both full readings fail, yet the census says **“2 can be done to the end”**, having pruned at Gives. Thus `total - stopped()` is not generally an executable census. This does not change the two reported corpus conclusions: old has its assertion at the end; new stops before its first assertion.

Plausible omitted dimensions include:

- 頭位 as a fixed physical position, a stage's active species, the pivot column's position, or even a leading digit, rather than always the target's first nonzero species. The position could change by stage. No alternatives vary this semantic choice.
- 減 as ordinary subtraction versus 去 as completed elimination, with multiplication/reduction inferred from context. Arbitrary independent choices of five operators at every occurrence are not a linguistic model of this distinction.
- Actual progressive placement at `先置第三行`, `次置第二行`, `次置右行`, with temporary placement, copying, renumbering, or retention of an original column. The page instead sets all five original columns first and never models these later 置 clauses as Set operations.
- Different repairs/repunctuation of `及減第三行`; grouping its action with the preceding or following clause, not just selecting a replacement source/target or Nothing.
- Euclidean subtraction terminating in either column and exchanging/repositioning the surviving column. `Board::mutual` insists the designated target vanish; if the actor vanishes first it errors. Its menu is not all material implementations of 相減.

The README's express caveat that omitted readings are untested is fair and should remain. “100 million” supplies no additional historical exhaustiveness. The literal self-subtraction mentioned in the prose is not actually one of line 20's alternatives, either.

I tried an additional exact integer search allowing source-head as well as target-head positions, reciprocal/idle/repaired second clauses, Euclidean operations, and 24 orderings of the first four species. **I did not find a successful reading of the 77 passage.** This is an exploratory negative result, not a proof of corruption; no honest 77-operation total for a successful reconstruction follows from this review. Replacing the passage by ordinary elimination would reach 6, but would not answer the historical question.

For **124**, the first ordinary subtraction gives third `[1,0,4,-3,0,4]`. It can eliminate the 實s 140, 128, 112 exactly by 35, 32, 28 repetitions. Taking the left column from that third once next leaves 實 −91, which cannot exactly remove the left's 95; that is the default failure. The later `右行當左行下位` is retained only as words, with no operational alternative. Copies/reindexing after `廢去第三行`, stage-specific meanings of 下位/頭位, and distinct readings of 去 remain unexplored. The isolated last-part subtraction and fraction arithmetic check, but do not validate the preceding rate-finding or the count 124.

### 10. Low — universal coverage claims exceed the implemented assertions

README `:4`–`:7` promises every described board and every stated figure checked exactly; `tutorial/01-introduction.md:103` repeats the universal figure claim. Counterexamples are already documented in the corpus: Dai's half/double well depths (`dai-zhen-13.jz:4`), 32/8/40/8 in Dai's sparrow working (`dai-zhen-09.jz:12`), the 4:3 weight rates (`liu-hui-09.jz:21`), and the fractional last part (`liu-hui-18-rates.jz:13`). Some are embedded in operations or comments; several are only quoted words and receive no arithmetic assertion.

The reported **38 figures** are 38 successful/failed assertion lines, not 38 individual numerical figures, and include duplicated answers and Count 145. Count 99 gets an information note and is excluded from the checked total; the 77/124 counts are quoted, not Count statements. Consequently “37 agree, 1 does not” cannot be treated as a census of all numerical claims in the witness. The detailed limitations are often candid; the opening promises should match them.

## Independent answer-form derivation

For each original board I performed integer elimination in the declared text order, then rational back substitution. D is the final 法, and D/gcd(D, all 實s) is the shared reduced denominator. Values match the printed answers except Problem 6's intentionally retained signs. Problem 13 gives the six rates 721:265:191:148:129:76; it is not one of the eleven fractional answers.

| Problem | D | 共約 denominator | Printed denominator | Board | 約 | Lowest |
|---|---:|---:|---:|:---:|:---:|:---:|
| 1 | 36 | 4 | 4 | — | ✓ | ✓ |
| 2 | 52 | 52 | 52 | ✓ | ✓ | — |
| 3 | 25 | 25 | 25 | ✓ | ✓ | ✓ |
| 7 | 21 | 21 | 21 | ✓ | ✓ | ✓ |
| 9 | 19 | 19 | 19 | ✓ | ✓ | ✓ |
| 10 | 4 | 2 | 2 | — | ✓ | ✓ |
| 11 | 11 | 11 | 11 | ✓ | ✓ | ✓ |
| 12 | 7 | 7 | 7 | ✓ | ✓ | ✓ |
| 14 | 666 | 111 | 111 | — | ✓ | — |
| 15 | 23 | 23 | 23 | ✓ | ✓ | ✓ |
| 16 | 976 | 122 | 122 | — | ✓ | ✓ |
| Total | | | | **7/11** | **11/11** | **9/11** |

These are fairly defined, consistently applied **formal comparisons under the specified algorithm**. The common denominator is also simply the least common multiple of the denominators of the reduced rational answers. Thus the same output can be obtained by reducing each answer and then expressing all answers over their least common denominator; the table cannot locate an actual historical reduction step, nor establish whether reduction occurred during elimination. README `:126` is sound as a numerical identity, not a unique procedural inference.

Independent operation/space results (multiplication + taking + division, not rod manipulations):

| Method | Plain | Worked | Repeated | Stored rods | Stored places |
|---|---:|---:|---:|---:|---:|
| text | 99+50+9 = 158 | 158 | 64+180330+9 = 180403 | 160 | 11 |
| reducing | 99+50+39 = 188 | 179 | 60+410+34 = 504 | 138 | 4 |
| cross | 90+50+5 = 145 | 140 | 140 | 149 | 11 |
| one | 46+50+5 = 101 | 98 | 13+279+3 = 295 | 122 | 4 |

## Scan ledger

Paths below are relative to the reviewed repository. Page numbers refer to `sources/sibu-congkan/pages/pNN.jpg`, not modern printed pagination. “Match” means the checked wording/numbers agree with that image, **not** that the procedure or program agrees with the stated answer. Normalized signs, zero positions, unit conversions, and rates are distinguished from characters actually printed. Punctuation was not treated as witness evidence. No requested figure was taken solely from the unproofread text.

### All 18 Answer/Rates lines

| File:line | File has | Scan shows | Verdict |
|---|---|---|---|
| corpus/no-01.jz:29 | 9 1/4, 4 1/4, 2 3/4 | p03 九斗四分斗之一; 四斗四分斗之一; 二斗四分斗之三 | Match, including all three denominators |
| corpus/no-02.jz:22 | 1 18/52, 41/52 | pp06–07 一斗五十二分斗之十八 (十/八 across page); 五十二分斗之四十一 | Match; 52 must survive |
| corpus/no-03.jz:24 | 9/25, 7/25, 4/25 | p08 二十五分斗之九 / 之七 / 之四 | Match |
| corpus/no-04.jz:16 | 5, 2 | p11 五升; 二升 | Match, 升 |
| corpus/no-05.jz:15 | 8, 3 | p13 八升; 三升 | Match, 升 |
| corpus/no-06.jz:20 | 8, 3 | p14 八斗; 三斗 | Match, 斗; not −8/−3 |
| corpus/no-07.jz:16 | 1 13/21, 20/21 | p15 一兩二十一分兩之十三; 二十一分兩之二十 | Match |
| corpus/no-08.jz:14 | 1200, 500, 300 | p16 牛價一千二百; 羊價五百; 豕價三百 | Match |
| corpus/no-09.jz:17 | 1 13/19, 1 5/19 | p18 一兩十九分兩之十三; 一兩十九分兩之五 | Match |
| corpus/no-10.jz:18 | 37 1/2, 25 | p19 三十七錢半; 二十五錢 | Match |
| corpus/no-11.jz:18 | 5454 6/11, 1818 2/11 | p20 五千四百五十四錢十一分錢之六; 一千八百一十八錢十一分錢之二 | Match, column continuations included |
| corpus/no-12.jz:16 | 22 6/7, 17 1/7, 5 5/7 | p21 二十二石七分石之六; 十七石七分石之一; 五石七分石之五 | Match; 六 continues next column |
| corpus/no-13.jz:24 | 721 265 191 148 129 76 | pp22–23 井七丈二尺一寸; 甲二丈六尺五寸; 乙一丈九尺一寸; 丙一丈四尺八寸; 丁一丈二尺九寸; 戊七尺六寸 | Match after conversion to 寸; rate interpretation is explicit editorial choice |
| corpus/no-14.jz:20 | 33/111, 28/111, 17/111, 10/111 | p24 一百一十一分斗之三十三 / 之二十八 / 之十七 / 之十 | Match; no individual reduction of 33/111 |
| corpus/no-15.jz:18 | 17/23, 11/23, 10/23 | p25 二十三分石之十七 / 之十一 / 之十 | Match |
| corpus/no-16.jz:16 | 45/122, 41/122, 97/122 | pp26–27 一百二十二分雞之四十五 / 之四十一 / 之九十七 | Match; page continuations included |
| corpus/no-17.jz:18 | 177, 121, 23, 29 | p28 羊一百七十七; 犬一百二十一; 雞二十三; 兔二十九 | Match; 兔 is present |
| corpus/no-18.jz:22 | 7, 4, 3, 5, 6 | pp29–30 麻七錢; 麥四錢; 菽三錢; 荅五錢; 黍六錢, each 一斗 | Match |

### Problem 6 signs, and three independently selected Set lines

| File:line | File has | Scan shows | Verdict |
|---|---|---|---|
| corpus/no-06.jz:9,17 | 益實六斗正; 實 +6 | p14 **益實六斗正**, clear 正 | Match; the anomalous sign is in this witness |
| corpus/no-06.jz:9,18 | 益實一斗正; 實 +1 | p14 **益實一斗正**, clear 正 | Match; not 負 |
| corpus/no-04.jz:14 | 7 −5 25 | p12 次置上禾七秉正下禾五秉負損實二斗五升正 | Match, with 斗→升 conversion |
| corpus/no-08.jz:12 | −5 6 8 −600 | pp16–17 次置牛五負羊六正豕八正不足錢負; statement 不足六百 | Match |
| corpus/no-18.jz:19 | 2 5 3 9 4 112 | p29 麻二斗麥五斗菽三斗**答**九斗黍四斗直錢一百一十二 | Match; quoted statement preserves 答 |

### Commentary figures, including figures retained only as words

| File:line | File has | Scan shows | Verdict |
|---|---|---|---|
| corpus/no-02.jz:18,19 | 7 2 11; 2 8 9 | p07 上禾七秉下禾二秉實十一斗; 上禾二秉下禾八秉實九斗 | Match |
| corpus/no-07.jz:25 | 10 4 20 | p15 牛十羊四直金二十兩 | Match |
| corpus/no-07.jz:26 | 10 25 40 | p15 牛十羊二十五直金四十兩 | Match |
| corpus/no-07.jz:27,29 | 金多20, 羊差21; 0 21 20 | p15 金多二十兩; 羊差二十一; 牛數盡 | Match; zero encodes 數盡 |
| corpus/dai-zhen-08.jz:9 | multipliers 2 and 3 | p41 右行牛二; 中行牛三 | Match |
| corpus/dai-zhen-08.jz:11 | 0 33 −45 3000 | p41 羊三十三正豕四十五負餘錢三千正 | Match; ox zero inferred from elimination |
| corpus/dai-zhen-08.jz:19 | multipliers 2 and 5 | p41 右行牛二; 左行牛五 | Match; sign handling is interpretation of the stated signs |
| corpus/dai-zhen-08.jz:21 | 0 37 −49 3800 | p41 羊三十七正豕四十九負餘錢三千八百正 | Match |
| corpus/dai-zhen-08.jz:25 | multipliers 33 and 37 | p41 右行羊三十三; 左行羊三十七 | Match |
| corpus/dai-zhen-08.jz:26 | 0 0 48 14400 | p41 豕四十八正以爲法錢一萬四千四百正爲實 | Match |
| corpus/dai-zhen-08.jz:28 | 1200, 500, 300 | p42 豕價三百; 羊價五百; 牛價一千二百 | Match; computational answer reordered to 牛羊豕 |
| corpus/dai-zhen-09.jz:6,7,10,11 | 4 1 8; 1 5 8; multiply both by 8 | pp42–43 四雀一燕 / 一雀五燕; 以下實俱重八兩徧乘上雀燕 | Match; 64 is computed, not a printed figure |
| corpus/dai-zhen-09.jz:12 | left 雀8 from right 雀32; right 燕8 from left 燕40; remainders24,32 | p43 左雀八減右雀三十二餘二十四; 右燕八減左燕四十餘三十二 | Match; computed through Take, not separate Gives checks |
| corpus/dai-zhen-09.jz:15 | 24 −32 0 | p43 remainders 二十四 / 三十二; 二物正負相借 | Match as a signed-column encoding; 0 inferred |
| corpus/dai-zhen-09.jz:17 | 3 −4 0 | p43 雀得三燕得四乃三雀當四燕也 | Match as encoding |
| corpus/dai-zhen-09.jz:18 | sparrow/swallow rates 4,3 | p43 雀率重四而燕率重三 | Match; quote is not checked by the program |
| corpus/liu-hui-09.jz:5–9 | 4雀1燕 / 1雀5燕; 1斤; 8兩 each; left head1 | p18 四雀一燕與一雀五燕; 并重一斤故各八兩; 左行頭位其數是一 | Match |
| corpus/liu-hui-09.jz:13 | 1 13/19, 1 5/19 | p18 main-text answer (as above); commentary only 中法下實 | Answer matches; this is not a separately stated commentary answer |
| corpus/liu-hui-09.jz:16,20 | equal weights; 3 −4 0 | p18 四雀一燕與一雀五燕其重等; 三雀四燕重相當 | Match; signs/zero encode equality |
| corpus/liu-hui-09.jz:21 | 雀率4 / 燕率3 | pp18/42–43 雀率重四燕率重三 | Match; quoted only |
| corpus/no-13.jz:27,30 | 法721 / 實76 | p23 其後法得七百二十一實七十六; also 721綆 / 76逮井 / 76 over721 | Match, including repeated figures in the quotation |
| corpus/dai-zhen-13.jz:4 | half-depth360½; ropes132½,95½,74,64½,38 寸 | p44 三丈六尺有半寸; 一丈三尺二寸半; 九尺五寸半; 七尺四寸; 六尺四寸半; 三尺八寸 | Match; independently exactly half the six rates |
| corpus/dai-zhen-13.jz:4 | doubled1442; ropes530,382,296,258,152 寸 | p44 十四丈四尺二寸; 五丈三尺; 三丈八尺二寸; 二丈九尺六寸; 二丈五尺八寸; 一丈五尺二寸 | Match; exactly doubled |
| corpus/dai-zhen-13.jz:8,17 | 721 265 191 148 129 76; 百 as丈 / 十 as尺 discussed | p44 七百二十一; 二百六十五; 一百九十一; 一百四十八; 一百二十九; 七十六; 百爲丈十爲尺 | Match |
| corpus/dai-zhen-13.jz:20,22–28 | set each 實721; 法721; 實54796; quotient76 | pp44–45 法得七百二十一實得五萬四千七百九十六以法除實得用逮之數七十六 | Match; 54796/721 =76 independently |
| corpus/liu-hui-18-new.jz:43 | 菽5 against 荅3 | p33 上得五下得三是菽五當荅三 | Match; not reached |
| corpus/liu-hui-18-new.jz:53 | 荅6 against 黍5 | p33 上得六下得五是爲荅六當黍五 | Match; not reached |
| corpus/liu-hui-18-new.jz:55 | 上2 / 下3 | p33 餘約之上爲二下爲三 | Match; reduced result not separately asserted |
| corpus/liu-hui-18-new.jz:61 | 麥3 against 菽4 | p33 上得三下得四是爲麥三當菽四 | Match; not reached |
| corpus/liu-hui-18-new.jz:64 | 麻4 against 麥7 | p33 上得四下得七是爲麻四當麥七 | Match; not reached |
| corpus/liu-hui-18-new.jz:65 | rates 麻7麥4菽3荅5黍6 | p33 repeats pair quantities and derives 七/四, 四/三, 五/六 | Match; quoted only |
| corpus/liu-hui-18-rates.jz:12 | 麻1 菽4 荅−3 實4 | pp33–34 麻一斗菽四斗荅三斗負下實四正 | Match; independently third minus fourth |
| corpus/liu-hui-18-rates.jz:13 | 3,5 over7; 菽1 5/7; 荅−2 1/7; 法4/7; 實28; 麻7 | p34 菽率三荅率五…麻率七; 一斗七分斗之五正; 二斗七分斗之一負; 七分斗之四; 二十八; 七 | Match; independently 1+12/7−15/7=4/7, then4/(4/7)=7; quote not executed |

### The complete 77 passage and four stated counts

Each row compares the quotation, not an endorsement of its adjacent machine operation.

| File:line | File has | Scan shows | Verdict |
|---|---|---|---|
| corpus/liu-hui-18-old.jz:8 | 以舊術爲之凡應置五行今欲要約 | p31 same, at end of page | Match |
| corpus/liu-hui-18-old.jz:17 | 先置第三行以減第四行 | pp31–32 先置 / 第三行以減第四行 | Match |
| corpus/liu-hui-18-old.jz:20 | 及減第三行 | p32 **及減第三行**, not 第五行 | Match; corruption/interpretation is not an OCR correction |
| corpus/liu-hui-18-old.jz:22 | 次置第二行以第二行減第三行去其頭位 | p32 same | Match |
| corpus/liu-hui-18-old.jz:24 | 次置右行去其頭位 | p32 same | Match; the quoted 置 is not implemented as placement here |
| corpus/liu-hui-18-old.jz:26 | 次以第四行減左行頭位 | p32 same | Match |
| corpus/liu-hui-18-old.jz:28,29 | 次以左行去第四行及第二行頭位 | p32 same | Match |
| corpus/liu-hui-18-old.jz:31 | 次以第五行減第二行頭位 | p32 same, with 五 | Match |
| corpus/liu-hui-18-old.jz:32 | 餘可半 | p32 same | Match |
| corpus/liu-hui-18-old.jz:34 | 次以第二行去第四行頭位 | p32 same | Match |
| corpus/liu-hui-18-old.jz:35 | 餘約之爲法 | p32 same | Match |
| corpus/liu-hui-18-old.jz:38 | 實如法而一得空卽有黍價; asserted result ±(1,6) | p32 **實如法而一得空卽有黍價** | Quotation matches. 空 is printed; 六 is a conjecture inferred from the answer, not this character |
| corpus/liu-hui-18-old.jz:39 | 以法減第二行得荅價左行得麥價第三行麻價右行得菽價 | p32 same | Match; unexecuted back-working |
| corpus/liu-hui-18-old.jz:39 | 如此凡用七十七算 | p32 如此凡用**七十七算** | Match, 77; not a Count assertion |
| corpus/liu-hui-18-new.jz:68 | 如此則凡用一百二十四算也 | p35 same, **一百二十四** | Match, 124; not a Count assertion |
| corpus/dai-zhen-18.jz:23 | 兼乘減除言之則一百四十五算; Count145 | p48 …凡用十算兼乘減除言之則**一百四十五算** | Match, 145; 十算 also present and retained at :20 |
| corpus/dai-zhen-18.jz:44 | 凡用九十九算; Count99 | p49 凡用**九十九算** | Match, 99 |

The 正/正 anomaly, the obscure 空, and the self-referential 第三行 all survive direct image checking. They should not be silently “fixed” to make the computation succeed. The review found implementation and interpretation problems without finding a numerical mistranscription among these requested checks.
