# jiuzhang

Chapter 8 of the *Nine Chapters on the Mathematical Art* (九章算術, 方程),
run on a counting board. Every problem of the chapter, and the boards its
commentators work, set out as the text sets it and worked a step at a time as
the text says; the figures the text and its commentators state, where the
notation can hold them, are compared exactly with what the board gives. Some
it cannot hold (rates read crosswise, arithmetic in fractions off the board)
and those are kept as quoted words, unchecked.

```
$ jiuzhang no 2
…
Set right  7 2 11        "損之曰益"
Set left   2 8 9         "益之曰損"
Solve.                   法 52; 實 70 41; 4 steps
Answer 1 18/52, 41/52    ✓  written over 52: the 法 as the board leaves it, 52 (not in lowest terms, 1 9/26)
```

*Fangcheng* (方程) is the chapter on what we would call simultaneous linear
equations. The conditions are set out as columns of counting rods, right to
left; the procedure multiplies a whole column by the head of another (徧乘)
and takes that column from it until its head is gone (直除), column after
column, until one figure is left over another: the 法 (divisor) over the 實.
Negative quantities are black rods, positive red, and a rule of signs
(正負術) says how to take and add them.

`jiuzhang` does three things. It **runs** a page: each operation on the
board, as transcribed. It **checks** what the source states: a column a
commentator says the board now holds, an answer as the text writes it, a
count of operations (算). And it **reads in the alternative**: where a clause
of a commentary can be read more than one way, the page gives every reading,
and `jiuzhang readings` runs every combination.

This is not a solver for linear equations. It is a reading of one chapter:
what the text says to do, what doing it gives, and where they part.

**Try it:** the [playground](https://chrishayuk.com/demos/nine-chapters) runs
every page on a three-dimensional counting board in the browser, and the
[notebook entry](https://chrishayuk.com/notebook/the-nine-chapters-writes-18-52-as-the-board-leaves-it)
tells the story.

## What it found, in brief

Each line links to its section below, which says what the result rests on.
Who had it first is in [Earlier work](#earlier-work); nothing here is claimed
as new.

- **38 of 39** statements of the text and its commentators that the board
  reaches agree with it. The one that does not is [Problem 6](#problem-6),
  whose signs in this edition give −8 and −3 for 8 and 3 (Li Huang corrected
  them).
- [**The answers are written as the board leaves them**](#how-the-answers-are-written),
  over one denominator each and not always in lowest terms: 1 18/52, not
  1 9/26. Dividing the 法 and the 實s by what they share (約) writes all 11
  fractional answers as the text does; the board's 法 as it stands writes 7;
  lowest terms writes 9.
- [**Dai Zhen's 145**](#dai-zhens-counts) is what his plain method comes to
  if each multiplication, taking and division of a single figure is one 算.
  His 99 is reached only by a rule proposed after both numbers were known.
- [**Liu Hui's 77, as printed, does not run**](#liu-huis-77): none of
  100,000,000 combinations of the readings offered reaches the price of 黍,
  as Dai Zhen said (訛舛不可通).
- [**Both published reconstructions of the 77 run**](#two-published-readings-of-the-77).
  Guo Shuchun's emended text (算 as operations) reaches the prices, and his own
  rule accounts for 67 of the 77, leaving 10 for placing rods. Every one of
  Hart's fifteen rod counts (算 as rods) agrees with the board. But his
  77-rod bound holds only if each step is worked a position at a time from the
  bottom; worked from the top it needs 78. Each reading gives 77 only on its
  own path, so the number cannot choose between them.
- [**Liu Hui's 124 is a count of rods**](#liu-huis-124-as-guo-works-it), on
  Guo's own working of the new method: the board, every operation fixed by
  the text, peaks at exactly 124 rods, just before the text says to drop the
  3rd column. Counted as operations, the same working cannot be 124 (Guo's
  rule gives 228 before the last part). That 124 means rods is Hart's and
  Wei and Guo's reading; the peak itself was not found stated in what could
  be read.

## Install

You need [Rust](https://rustup.rs):

```bash
cargo install --git https://github.com/chrishayuk/nine-chapters
```

The whole chapter, as transcribed, is compiled into it:

```bash
jiuzhang no 1            # Problem 1, worked on the board
jiuzhang verify          # every page
jiuzhang answers         # how each answer is written, and which reading writes it so
jiuzhang counts          # the commentators' counts of 算, under every rule
```

For Visual Studio Code, `editors/vscode/jiuzhang-language-0.1.0.vsix`
(`code --install-extension jiuzhang-language-0.1.0.vsix`): colours, the
figures the board does not give as problems while you type, hover that shows
what an operation did to each position by the rule of signs and the board
after it, and a run button. It carries its own copy of `jiuzhang`. For
Neovim, Helix and Emacs see [`editors/README.md`](editors/README.md). Or open
the playground, `web/`, which runs the same program in the page and draws the
board in three dimensions (below).

## Using it

```
jiuzhang FILE              the board, worked a line at a time, with every figure the source states checked
jiuzhang no 13             a problem of chapter 8 (also: jiuzhang no liu-hui-18-old; jiuzhang list)
jiuzhang --plain FILE      the page, checked, without the boards
jiuzhang --check FILE      what is wrong with it, a line each
jiuzhang verify            run every page and compare with the source
jiuzhang answers           how each answer is written, and which reading of the last step writes it so
jiuzhang counts            the counts of 算 the commentators give, under every rule
jiuzhang readings FILE     every combination of the readings a page gives in the alternative
jiuzhang rods FILE         the most rods on the board, by how each step is worked
jiuzhang solve 3 2 1 39 / 2 3 1 34 / 1 2 3 26 [by text|reducing|cross|one]
jiuzhang methods           the procedures `Solve` can follow
jiuzhang rules             the rules for counting 算
jiuzhang decisions         the choices this reconstruction makes that the text does not
jiuzhang lsp               a language server on standard input and output
```

## The notation

A `.jz` file is a board and what is done to it.
[`docs/notation.md`](docs/notation.md) has all of it. Problem 1:

```
No. 8.1
Things 上禾 中禾 下禾 實
Set right   3 2 1 39          "置上禾三秉，中禾二秉，下禾一秉，實三十九斗，於右方"
Set middle  2 3 1 34          "中、左禾列如右方"
Set left    1 2 3 26
Multiply middle by the head of right.            "以右行上禾徧乘中行"
Take right from middle until the head is gone.   "而以直除"
Solve.
Answer 9 1/4, 4 1/4, 2 3/4
```

A transcription never corrects the text. Where the text's signs give the
wrong answer, the file has the text's signs.

## What running it found

These are results against this project's transcription, under the choices in
`jiuzhang decisions`. The transcription was made by a coding agent and its
sub-agents, from the scans; no person has checked it. Nothing here is claimed
as new. A literature search by a second model (below, and
[`docs/literature-check.md`](docs/literature-check.md)) found that several of
these results are already in the literature, and says which.

`jiuzhang verify` runs 30 pages. The 18 problems and 8 pages of commentary
make 39 checked statements (a column a commentator gives, an answer, a set
of rates, a count): 38 agree with the board and 1 does not. Five more are not
reached: they come after a step that cannot be done. The other four pages
are modern reconstructions of Liu Hui's 77 and 124 (below), whose 27
statements all agree; with them `verify` reports 66, 65 right. A statement can hold several
figures, and some figures the sources give are quoted and not checked, so
this is not a census of every number in the text.

### How the answers are written

Eleven answers have fractions. The text writes them over one denominator,
and not always in lowest terms: Problem 2 is 1 斗 and 18/52, not 9/26;
Problem 14 is 33/111, not 11/37. Which reading of the last step writes them
so?

| reading | answers written as the text writes them |
|---|---|
| every 實 over the 法 as the board leaves it | 7 of 11 |
| each answer in its lowest terms | 9 of 11 |
| the 法 and all the 實s divided by what they share (約) | 11 of 11 |

So the answers are written over one denominator for each problem, and it is
not the board's 法 as it stands (Problem 1's board leaves 36, the text writes
quarters; Problem 16's 976, the text 122), and not each fraction in its own
lowest terms (52 and 111 survive). The 約 reading gives the same denominators
as reducing each answer and then writing them all over the least of their
common denominators: the table shows how the answers are written, not where
in the working a reduction was made. That the denominators reflect the board
is known: Hart (2011, pp. 173-174) discusses Problem 14's 33/111, not 11/37,
and its relation to the determinant. The count across all eleven answers was
not found in the literature searched. Problem 13's answer is the extreme case: the
depth of the well, 721, is the 法 itself, given as a rate (舉率以言之), and
Liu Hui's 法得七百二十一實七十六 is the board's left column, checked.

### Problem 6

In this edition (the Weiboxie text), Problem 6's procedure sets its 實 as
正 in both columns: 益實六斗正 … 益實一斗正. Run as printed, the board gives
−8 and −3 where the answer is 8 and 3. The other text consulted has 負 in both
places, and with 負 the board gives the answer. The scan was read twice.
This is known: Li Huang (李潢, 九章算術細草圖說, juan 8) says "正正皆當作負", and
Chemla and Guo (2004, p. 632, n. 51) print 負, credit him, and record 正 in
other witnesses too, so it is not peculiar to this edition.

### Dai Zhen's counts

In his collation notes (訂訛, 1770s), Dai Zhen says that Problem 18 by the
plain method — cross-multiplying each pair of columns (互乘), then finding the
prices from the bottom up — takes "145 算, counting multiplications, takings
and divisions together" (兼乘減除言之), and that a shorter way, working from a
column whose head is one, takes 99. He does not say what one 算 is beyond
that, so `jiuzhang rules` tries several conventions and prefers none:

| rule | the plain method | the shorter way |
|---|---|---|
| plain: each multiplication, taking and division of one figure, less the heads that cross-multiplying makes equal (上一位互乘其數必同可省乘) | **145** | 101 |
| worked: also saving everything multiplied or divided by one | 140 | 98 |
| readoff: as plain, but a division by one is read off, not worked | **145** | **99** |

Plain is the nearest to his words, but leaving out the heads' takings as well
as their multiplications is this project's convention, not his. Worked
extends his economies (上一位遇一則省其與對行徧乘, which are about
multiplying) to dividing, which he does not mention. Readoff was proposed by
the independent review after both numbers were known: it gives both, and is
reported as a rule fitted after the fact, not as Dai Zhen's.

| Problem 18 by | plain | worked | readoff | the most rods on the recorded boards | the widest figure on them |
|---|---|---|---|---|---|
| the text's 徧乘 and 直除 | 158 | 158 | 158 | 160 | 11 places |
| the same, with 約 after each step | 188 | 179 | 183 | 138 | 4 places |
| Dai Zhen's 互乘 | **145** | 140 | **145** | 149 | 11 places |
| Dai Zhen's shorter way | 101 | 98 | **99** | 122 | 4 places |

The rods and places are those of the board as recorded after each step. The
working of the last step is not on it: finding the 荅 by the text's method
forms a product of 16 places, and cross-multiplying forms products the board
never shows, so these are not the workspace a reckoner needed. Under the
`repeated` rule, which counts a taking done several times over as that many
takings of single figures, the text's method on Problem 18 comes to 180,403
operations (180,330 of them takings), or 89,773 takings of a whole column.

### Liu Hui's 77

In his commentary to Problem 18, Liu Hui works it "by the old method, made
short" (以舊術爲之…今欲要約), clause by clause, and says it takes 77 算. Dai
Zhen calls the passage corrupt (訛舛不可通). `corpus/liu-hui-18-old.jz` gives
each clause in every reading this project could find: the columns as written,
and, for the clause 及減第三行, which as written takes the 3rd column from
itself, other columns or nothing; and each 減 or 去 done once, until the head is gone, as far as it goes,
in turn (taking each column from the other, without multiplying), or
crosswise. That is 100,000,000 combinations of a small menu of operations,
not 100,000,000 interpretations. 229,906 of them can be done to the end. None
leaves the 4th column giving the price of 黍, as the passage says it should.
So none of these readings of the passage works; that does not show that no
reading does, and Liu Hui's 77 is not tested.

The passage has been emended and worked by others. Chemla and Guo (2004,
pp. 652-655, nn. 93-104) repair its opening (及減第三行 to 又減第五行, among
other changes) and restore 六 for 空 (crediting Dai Dunyuan); Guo Shuchun
(*九章筭術譯注*, revised 2020, ch. 8, nn. 22-40) works it through and tallies
77, counting each placing of rods, each operation on a whole column and each
division of a 實 by a 法 as one 算. Wei Xuegang and Guo Shirong (2016) and
Zhu Yiwen (2025) read 算 as counting rods. The census here ran the text as
printed: it says only that the unemended passage does not work under the
readings offered, which is what Dai Zhen said. The published reconstructions
are run in the next section. Among what the menu leaves out
(the review lists more): 頭位 as anything but the first figure of the column
worked on; the columns set one at a time, as 先置第三行 … 次置第二行 …
次置右行 says, with copies kept; other repairs of 及減第三行; mutual
subtraction ending in either column. The independent review tried a wider
search of its own (heads of either column, other orders) and found no
working reading either.

### Two published readings of the 77

Two published reconstructions read 算 differently, and both are run here, from
a second model's extraction of the sources ([`docs/guo-77.md`](docs/guo-77.md),
which marks what it read and what it reconstructed). The pages assert only
what the sources state.

**Guo Shuchun (2020): 算 as operations.** Guo emends the passage (with
Chemla and Guo 2004) and works it block by block; his note 40 counts as one
算 each placing of rods, each multiplication or division of a whole column,
each taking of one column from another, and each division of a 實 by a 法.
`corpus/guo-18-old.jz` follows his emended text: it runs to the end, both
halvings (餘可半) go exactly, the 黍 comes out six and the prices check. Under
his rule, without the placings, it is 67 operations; his 77 leaves 10 for
placing rods, which this page does not model, and how his figure 8-7 assigns
them was not readable.

**Roger Hart (2012): 算 as rods.** Hart reconstructs the old method on a
board where columns are placed as they are needed, and counts the rods on it
after each step: never more than 77. `corpus/hart-18-old.jz` follows his
steps, and every one of the fifteen counts he reports agrees with the board,
counted with jiuzhang's rule of rods (a digit to five is that many rods, six
to nine one for five and the rest), which was written before his paper was
read. His path ends with the 黍 at six.

`jiuzhang rods` asks how many rods a step needs while it is worked, not only
after. Hart's 77 holds if each column is worked a position at a time,
straight to its new figures, from the bottom (or in the best order for each
step). Worked from the top, his twelfth step briefly needs 78; worked a
column at a time, setting down a product and then taking from it, 86. So his
reconstruction keeps to 77 rods, under an assumption about how a step is
worked that the text does not state.

Each reading gives 77 only on its own path: Guo's working holds 116 rods at
most (all five columns are set at the start, as his tally's first placing
has it), and Hart's is 59 operations under Guo's rule, with 5 placings. The
number 77 does not choose between them. Wei Xuegang and Guo Shirong (2016)
and Zhu Yiwen (2025) also read 算 as rods; their reconstructions were not
readable.

His new method (124 算), which Guo (2020, nn. 41-82) also works through, was
given the same treatment and stops, under every
reading offered, by the passage's fourth clause. Its last part, where the
columns are set afresh, checks: the 3rd less the 4th leaves 麻 1, 菽 4, 荅 −3,
實 4, as he says, and as Guo (2020, nn. 63-69) explains.

### Liu Hui's 124, as Guo works it

Liu Hui says his new method takes 124 算. As printed, the passage stops by its
fourth clause under every reading offered. Guo Shuchun works it through
(2020, nn. 41-62), and `corpus/guo-18-new.jz` follows his working, from the
same source extraction ([`docs/guo-77.md`](docs/guo-77.md), section 3): it
runs, and all seven figures the text and his notes state agree (菽 5 against
荅 3, 荅 6 against 黍 5, 麥 3 against 菽 4, 麻 4 against 麥 7, and the
figures before each reduction). Its last board is the extraction's
reconstruction, column for column.

**As operations it cannot be 124.** Guo gives no tally for the 124 (n. 82
states the total only). His rule, applied as his tally of the 77 applies it,
counts a column taken 35 times as 35 takings: the part that finds the rates
is already 228. Every other rule is over 124 or, for `columns` (35), far
under; `every` is 123 before the last part, whose first taking alone adds 6.

**As rods it is 124 exactly.** The new method sets all five columns, 116
rods. Up to the peak the text fixes every operation: 減 once, 去 until the
實 is gone, 不足減乃止 as far as it goes, with the printed 右行 read as 左行, as
Guo reads it. After 次以左行減第三行下位 the board holds 124 rods, and after
次以第三行去左行下位 still 124; then the text says 訖廢去第三行, and the 3rd
column, 27 rods, goes. The board never holds more; the last part, the
columns set afresh, needs about half as many. As with Hart's 77, the bound
holds worked a position at a time from the bottom, in the best order, or a
whole column at a time, and not from the top (130).

```
$ jiuzhang rods corpus/guo-18-new.jz
The most rods on the board at any moment, by how a step is worked:
    124  at the end of each step (a run of lines on one column, up to an assertion or a note)
    124  each line worked on the whole column at once (a product set down, then taken from)
    130  each step worked a position at a time, straight to its new figure, top down
    124  the same, bottom up
    124  each step worked a position at a time, in the best order for it
```

**What is known.** A second model's search
([`docs/literature-124.md`](docs/literature-124.md)) found the reading
already published: Hart (2012, n. 34) reads both 77 and 124 as rods, and Wei
Xuegang and Guo Shirong (2016), cited by Zhu (2025), read them so and say
they begin from the new method to settle what 算 means. Their full text could
not be opened, so they may well have counted this peak. Hudeček (2008, p. 206)
prints this board's figures (without the 實), and his note 71 records failed attempts to reach 124 by
counting operations. Chemla and Guo (2004) and Guo (2020) read 124 as steps.
The explicit peak, on Guo's path and under a stated rule of rods, was not
found in what could be read. One discrepancy is open: Zhu (2010, p. 82)
counts 110 rods for setting out the problem, where this rule counts 116.

The same question asked of the old method: Guo's emended working, with each
column set only when an operation first uses it
(`corpus/guo-18-old-placed.jz`), peaks at 100 rods, not 77. A rods reading
of the 77 needs a path like Hart's, not Guo's emended working.

So the second number does what the first could not. The 77 has a reading
for each meaning of 算; on the one published working of the new method, the
124 is reached by rods and not by operations.

## Reading the text

The text read is the **Sibu congkan** photographic reprint of the
**Weiboxie** (微波榭) edition, with Liu Hui's commentary (263), Li Chunfeng's
notes (656) and Dai Zhen's collation notes, from Wikimedia Commons, read from
the page images. Wikisource's transcription of the same pages is marked
unproofread, and was used only to find places: it drops 直 (in 直除, and
直 "worth") throughout, drops 兔 throughout Problem 17, and has 八之 for 入之
and 損妓之 for 損益之. A second, punctuated text of the main text, from
another edition, is a third witness. The notes on each range are in
[`sources/notes/`](sources/notes/), the brief the transcribers worked to in
[`sources/TRANSCRIBING.md`](sources/TRANSCRIBING.md), and where the sources
came from in [`sources/README.md`](sources/README.md).

## The decisions

`jiuzhang decisions` lists every choice the text does not fix: that the rule
of signs is signed arithmetic, how 徧乘 and 直除 are read, what "head" means,
the three readings of the last step, that no rule of counting is preferred,
that the readings offered for a corrupt passage are the transcriber's, and
the units.

## In the browser

`web/` is a playground that runs `jiuzhang`, compiled unchanged for WASI
(`wasm32-wasip1`), in the page. It draws the board in three dimensions with
[three.js](https://threejs.org): each figure in rods, the units upright and
the tens lying (一縱十橫), five as one rod across, red for 正 and black for
負, the columns right to left. Step through a page, or play it: a rod that
stays stays put, a new one is dropped into place, and the rods taken off the
board are handed to [Rapier](https://rapier.rs) and knocked off the felt. The
column taken and the column taken from are lit; the running count of 算, the
rods on the board and the widest figure are shown. Beside it, the page in an
editor whose colours, problems and hover come from `jiuzhang lsp` running in
the page too.

```bash
node web/build.mjs                  # builds web/jiuzhang.wasm and web/corpus.json
node web/test.mjs                   # every page through the wasm build, compared with the native one
python3 -m http.server -d web 8000  # then open http://localhost:8000
```

## Independent review

A second model, OpenAI's Codex, reviewed commit 8532977 independently: it
checked every answer, the Problem 6 signs, the commentary figures, the 77
passage and the four counts against the scans, and found no mistranscription;
it reproduced the answers table, the counts and the censuses with programs of
its own; and it wrote probe pages that made the checker give a false ✓ (a
contradictory board, an answer checked against a board since changed, an
answer written in a form no reading gives) and false or missing marks. Every
finding was checked here and acted on; what was done about each is in
[`docs/independent-review.md`](docs/independent-review.md), and its report is
[`docs/independent-review-report.md`](docs/independent-review-report.md).
The numbers in this README are after those changes.

## Tutorial

[`tutorial/`](tutorial/01-introduction.md) has ten chapters: the board,
multiplying and taking, the last step, negative numbers, rates, the
commentators' boards, counting 算, reading a corrupt passage (and running
the two published readings of it), and a reference. Every page and command in it is run by `cargo test` and compared
with what the chapter says it prints.

## Tests

```bash
cargo test
```

Unit tests for the board, the methods, the counts, the reader and the
language server; that every page of the transcription is in the notation;
that `verify`, `answers` and `counts` print what they printed when last
accepted (`JIUZHANG_BLESS=1 cargo test` accepts a change, which then shows
in the diff); and the tutorial. `node web/test.mjs` and, in
`editors/vscode`, `npm test` test the WebAssembly builds.

## Earlier work

Chapter 8 and its commentaries have been edited, translated and studied at
length: Qian Baocong's and Guo Shuchun's critical editions; Shen Kangshen,
J. N. Crossley and A. W.-C. Lun, *The Nine Chapters on the Mathematical Art:
Companion and Commentary* (1999); K. Chemla and Guo Shuchun, *Les neuf
chapitres* (2004); R. Hart, *The Chinese Roots of Linear Algebra* (2011), and
"Tracing Practices Purloined by the 'Three Pillars'" (*Korean Journal for
the History of Science* 34, 2012, pp. 287-357, with Liu Hui's 77 on
pp. 322-329); Guo Shuchun, *九章筭術譯注*
(revised 2020);
and on the rods and the cost of the procedures, Zhu Yiwen, "數：筭與術" (漢學研究
28, 2010) and "The interplay between textual procedures and material
operations" (*Science in Context*, 2025).

This project did not read them. A second model, OpenAI's Codex with web
search, looked for each finding in what it could reach (parts of Hart,
Chemla and Guo, Guo 2020, Zhu, and Li Huang); its report, with sources, is
[`docs/literature-check.md`](docs/literature-check.md). In short:

| finding | in the literature? |
|---|---|
| Problem 6's 正 should be 負 | known: Li Huang; Chemla and Guo 2004 |
| the answers' denominators and the board | partly: Hart 2011 (33/111, the determinant); not the count of eleven |
| Dai Zhen's 145 as a count of single-figure operations | not found (Shen et al. and Guo's 彙校 could not be read) |
| Liu Hui's 77 and 124 passages | known: emended and worked by Chemla and Guo, and Guo 2020, who tallies 77 as operations; 算 read as rods by Hart 2012, who keeps to 77 rods, and by Wei and Guo 2016 and Zhu 2025 |
| Guo's and Hart's reconstructions, run: Guo's rule gives 67 + 10 placings; Hart's 15 rod counts agree; his bound needs bottom-up working (78 top down) | not found (Guo's figure 8-7 was not readable) |
| the 124 as the most rods on Guo's working of the new method; not reachable as operations | partly: 124 as rods is Hart 2012 and Wei and Guo 2016 (whose full text could not be read); the board is in Hudeček 2008; the peak not found stated |
| the cost of the literal reading | partly: Zhu 2010 on rods against steps; not these figures |
| Liu Hui's rates 7:4:3:5:6 | known: Guo 2020; Chemla and Guo 2004 |

"Not found" means not found in what could be read, not that a finding is new.

It follows the method of the other reconstructions in this collection (the
Rhind papyrus, JOSS, Turing's ACE and others): rebuild from the documents
that survive, run what they say, and treat each place the run and the text
disagree as evidence.

## Licence

Apache License 2.0 - see [LICENSE](LICENSE).
