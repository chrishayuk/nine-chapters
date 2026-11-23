# jiuzhang

Chapter 8 of the *Nine Chapters on the Mathematical Art* (九章算術, 方程),
run on a counting board. Every problem of the chapter, and every board its
commentators describe, set out as the text sets it and worked a step at a
time as the text says; every figure the text and its commentators state is
compared, exactly, with what the board gives.

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
as new: the literature on chapter 8 is large (below), and this project has not
read it.

`jiuzhang verify` runs 26 pages: the 18 problems and 8 pages of commentary.
Of 38 figures the text and its commentators state that the board reaches, 37
agree and 1 does not. Five more are not reached: they come after a step that
cannot be done.

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

So the answers are what the board gives with one common factor taken out:
not the board's 法 as it stands (Problem 1's board leaves 36, the text writes
quarters; Problem 16's 976, the text 122), and not each fraction reduced on
its own (52 and 111 survive). Problem 13's answer is the extreme case: the
depth of the well, 721, is the 法 itself, given as a rate (舉率以言之), and
Liu Hui's 法得七百二十一實七十六 is the board's left column, checked.

### Problem 6

In this edition (the Weiboxie text), Problem 6's procedure sets its 實 as
正 in both columns: 益實六斗正 … 益實一斗正. Run as printed, the board gives
−8 and −3 where the answer is 8 and 3. The other text consulted has 負 in both
places, and with 負 the board gives the answer. The scan was read twice.

### Dai Zhen's counts

In his collation notes (訂訛, 1770s), Dai Zhen says that Problem 18 by the
plain method — cross-multiplying each pair of columns (互乘), then finding the
prices from the bottom up — takes "145 算, counting multiplications, takings
and divisions together", and that a shorter way, working from a column whose
head is one, takes 99. One rule of counting gives 145 exactly: every
multiplication, taking and division of one figure, leaving out the heads that
cross-multiplying makes equal, which he says need not be multiplied. The same
rule gives 101 for his shorter way; leaving out also everything multiplied or
divided by one, which he says can be saved, gives 98. Three operations by ±1
separate the two, and no rule here counts two of them and not the third. No
rule was written to give 99.

| Problem 18 by | plain | worked | the most rods on the board | the widest figure |
|---|---|---|---|---|
| the text's 徧乘 and 直除 | 158 | 158 | 160 | 11 places |
| the same, with 約 after each step | 188 | 179 | 138 | 4 places |
| Dai Zhen's 互乘 | **145** | 140 | 149 | 11 places |
| Dai Zhen's shorter way | 101 | 98 | 122 | 4 places |

Read literally, as taking one column from another one time after another,
the text's 直除 on Problem 18 is 180,403 takings.

### Liu Hui's 77

In his commentary to Problem 18, Liu Hui works it "by the old method, made
short" (以舊術爲之…今欲要約), clause by clause, and says it takes 77 算. Dai
Zhen calls the passage corrupt (訛舛不可通). `corpus/liu-hui-18-old.jz` gives
each clause in every reading this project could find: the columns as written,
and, for the clause that takes the 3rd column from itself, other columns or
nothing; and each 減 or 去 done once, until the head is gone, as far as it goes,
in turn (taking each column from the other, without multiplying), or
crosswise. That is 100,000,000 readings. 229,906 of them can be done to the
end. None leaves the 4th column giving the price of 黍, as the passage says it
should. So none of these readings of the passage works; that does not show
that no reading does, and Liu Hui's 77 is not tested.

His new method (124 算) was given the same treatment and stops, under every
reading offered, by the passage's fourth clause. Its last part, where the
columns are set afresh, checks: the 3rd less the 4th leaves 麻 1, 菽 4, 荅 −3,
實 4, as he says.

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

## Tutorial

[`tutorial/`](tutorial/01-introduction.md) has ten chapters: the board,
multiplying and taking, the last step, negative numbers, rates, the
commentators' boards, counting 算, reading a corrupt passage, and a
reference. Every page and command in it is run by `cargo test` and compared
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
chapitres* (2004); R. Hart, *The Chinese Roots of Linear Algebra* (2011),
on the board and the procedures of this chapter; and, on the gap between the
written procedures and what was done with the rods, Yiwen Zhu, "The interplay
between textual procedures and material operations from the viewpoint of
Chinese mathematical texts", *Science in Context* (2025). This project has
read none of them beyond Zhu's abstract, and so does not say how its readings
compare: the emendations of Problem 6, of 得空 in the 77 passage, and of the
77 and 124 passages themselves, are very likely discussed there. Nothing here
is claimed as new until it has been checked against that literature.

It follows the method of the other reconstructions in this collection (the
Rhind papyrus, JOSS, Turing's ACE and others): rebuild from the documents
that survive, run what they say, and treat each place the run and the text
disagree as evidence.

## Licence

Apache License 2.0 - see [LICENSE](LICENSE).
