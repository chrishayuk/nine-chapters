# 10. Reference

## Commands

```console
$ jiuzhang --help
jiuzhang: chapter 8 of the Nine Chapters (九章算術, 方程), run on a counting board

  jiuzhang FILE              the board, worked a line at a time, with every figure the source states checked
  jiuzhang no 13             a problem of chapter 8 (also: jiuzhang no liu-hui-18-old; jiuzhang list)
  jiuzhang --plain FILE      the page, checked, without the boards
  jiuzhang --check FILE      what is wrong with it, a line each
  jiuzhang verify            run every page and compare with the source
  jiuzhang answers           how each answer is written, and which reading of the last step writes it so
  jiuzhang counts            the counts of 算 the commentators give, under every rule
  jiuzhang readings FILE     every combination of the readings a page gives in the alternative
  jiuzhang rods FILE         the most rods the board needs, by how each step is worked
  jiuzhang solve 3 2 1 39 / 2 3 1 34 / 1 2 3 26 [by text|reducing|cross|one]
  jiuzhang methods           the procedures `Solve` can follow
  jiuzhang rules             the rules for counting 算
  jiuzhang decisions         the choices this reconstruction makes that the text does not
  jiuzhang lsp               a language server on standard input and output
  jiuzhang json FILE         the run, as JSON, for the playground

  FILE may be - for standard input.
```

`jiuzhang list` names every page in `corpus/`; `jiuzhang no` takes the name
without `corpus/` and `.jz`, or a problem's number.

```console
$ jiuzhang list
dai-zhen-08        8.8
dai-zhen-09        8.9
dai-zhen-13        8.13 (Dai Zhen, 訂訛)
dai-zhen-18        8.18 (Dai Zhen, 訂訛: 145 and 99)
guo-18-new         8.18 (Guo Shuchun's reading of Liu Hui's 124)
guo-18-old         8.18 (Guo Shuchun's reading of Liu Hui's 77)
hart-18-old        8.18 (Hart's reading of Liu Hui's 77: rods)
liu-hui-09         8.9
liu-hui-18-new     8.18 (Liu Hui: the new method, 124 算)
liu-hui-18-old     8.18 (Liu Hui: the old method made short, 77 算)
liu-hui-18-rates   8.18 (Liu Hui: the new method, the last part)
no-01              8.1
no-02              8.2
no-03              8.3
no-04              8.4
no-05              8.5
no-06              8.6
no-07              8.7
no-08              8.8
no-09              8.9
no-10              8.10
no-11              8.11
no-12              8.12
no-13              8.13
no-14              8.14
no-15              8.15
no-16              8.16
no-17              8.17
no-18              8.18
```

`jiuzhang verify` runs every page. For each it gives the figures right and
wrong, and the lines where something is wrong or where the page stops.

```console
$ jiuzhang verify
dai-zhen-08        8.8                                          4 right, 0 wrong
dai-zhen-09        8.9                                          2 right, 0 wrong
dai-zhen-13        8.13 (Dai Zhen, 訂訛)                          2 right, 0 wrong
dai-zhen-18        8.18 (Dai Zhen, 訂訛: 145 and 99)              4 right, 0 wrong
guo-18-new         8.18 (Guo Shuchun's reading of Liu Hui's 124) 7 right, 0 wrong, 4 lines read in the alternative
guo-18-old         8.18 (Guo Shuchun's reading of Liu Hui's 77) 2 right, 0 wrong, 1 lines read in the alternative
hart-18-old        8.18 (Hart's reading of Liu Hui's 77: rods)  16 right, 0 wrong
liu-hui-09         8.9                                          2 right, 0 wrong
liu-hui-18-new     8.18 (Liu Hui: the new method, 124 算)        0 right, 0 wrong, stops at line 25, 29 lines read in the alternative
    line 25: 95 at the 實 of left does not go by -91 exactly
liu-hui-18-old     8.18 (Liu Hui: the old method made short, 77 算) 0 right, 0 wrong, stops at line 32, 10 lines read in the alternative
    line 32: 2nd cannot be halved: a figure in it is odd
liu-hui-18-rates   8.18 (Liu Hui: the new method, the last part) 1 right, 0 wrong
no-01              8.1                                          1 right, 0 wrong
no-02              8.2                                          3 right, 0 wrong
no-03              8.3                                          1 right, 0 wrong
no-04              8.4                                          1 right, 0 wrong
no-05              8.5                                          1 right, 0 wrong
no-06              8.6                                          0 right, 1 wrong
    line 20: the board gives -8 for 8, -3 for 3
no-07              8.7                                          4 right, 0 wrong
no-08              8.8                                          1 right, 0 wrong
no-09              8.9                                          1 right, 0 wrong
no-10              8.10                                         1 right, 0 wrong
no-11              8.11                                         1 right, 0 wrong
no-12              8.12                                         1 right, 0 wrong
no-13              8.13                                         2 right, 0 wrong
no-14              8.14                                         1 right, 0 wrong
no-15              8.15                                         1 right, 0 wrong
no-16              8.16                                         1 right, 0 wrong
no-17              8.17                                         1 right, 0 wrong
no-18              8.18                                         1 right, 0 wrong

29 pages; 64 figures checked, 63 right, 1 wrong.
```

`jiuzhang --check FILE` gives only what is wrong, a line each. `jiuzhang
json FILE` gives the run as JSON, and `jiuzhang lsp` is a language server
for editors.

## Statements

| line | the text | what is done |
|---|---|---|
| `Things 上禾 中禾 下禾 實` | | names the positions, top to bottom, the 實 last |
| `Set right 3 2 1 39` | 置 | set a column, every position top to bottom |
| `Multiply middle by 3` | 徧乘 | multiply the whole of a column |
| `Multiply middle by the head of right` | 以右行上禾徧乘中行 | by the first figure of another column that is not empty |
| `Multiply middle by the 中禾 of right` | | by a named position of another column |
| `Take right from middle` | 減 | take one column from another, once |
| `Take right from middle until the head is gone` | 直除, 去其頭位 | as many times as empties the head of the column taken from |
| `Take right from middle until 中禾 is gone` | | as many times as empties that position |
| `Take right from middle as far as it goes` | 不足減乃止 | as many times as goes without passing zero at the head |
| `Take right from middle as far as the 菽 goes` | | the same, at a named position |
| `Take right from middle in turn` | 相減 | each from the other, the larger head from the smaller, until the head of the second is gone |
| `Take right from middle in turn until 實 is gone` | | the same, at a named position |
| `Take right from middle crosswise` | 互乘 | each multiplied by the other's head, then taken |
| `Add right to middle` | 益, 并 | add one column to another |
| `Halve middle` | 半 | halve a column (every figure must be even) |
| `Reduce middle` | 約 | divide a column by what all its figures have in common |
| `Divide middle by 3` | | divide a column (every figure must go) |
| `Drop 3rd` | 廢去 | take a column off the board |
| `Nothing` | | do nothing: a reading of a clause that does not act |
| `Solve` | 方程術 | the rest of the procedure, by the text's method |
| `Solve by reducing` / `by cross` / `by one` | | the same by another method |

The source's figures, checked and marked ✓ or ✗:

| line | checked against |
|---|---|
| `Gives middle 0 5 1 24` | the column as the board now holds it |
| `Answer 9 1/4, 4 1/4, 2 3/4` | what `Solve` found, thing by thing, top to bottom, and how each fraction is written |
| `Rates 721 265 191 148 129 76` | the 法 and the 實s, divided by what they share |
| `Count 77` | the 算 of the operations since the last `Count`, under every rule |

## Figures and the rest of a line

A figure is a whole number: `3` is 正, `-3` 負, `0` or `·` empty. An answer
is a whole number and a fraction, `1 18/52`, written with the text's
denominator. A half (半) is `1/2`, 少半 `1/3`, 太半 `2/3`.

| written | means |
|---|---|
| `right`, `middle`, `left`; `2nd`, `3rd`, … | the columns, from the right; `右行`, `中行`, `左行`, `第二行` … also |
| `A \| B` | readings in the alternative; the page runs with A |
| `"…"` on its own line | the source's words, kept and not computed |
| `"…"` after an operation | the words of the source it transcribes |
| `% …` | a note |
| `No. 8.1` | a heading |

The full account of the notation is `docs/notation.md`.

## Methods and rules

```console
$ jiuzhang methods
text      方程術 as stated: 徧乘 then 直除, right to left; the things found through the 法
reducing  the same, each worked column divided by what its figures have in common (約)
cross     Dai Zhen's 互乘: the two columns multiplied by each other's head, then taken; found by value from the bottom up
one       Dai Zhen's shorter way: work from a column whose head is one; equal heads taken directly
$ jiuzhang rules
plain     each multiplication, taking or division of one figure, except a head known to cancel; a taking several times over is a multiplication and a taking
worked    each multiplication, taking or division of one figure that has to be worked (not by one, not a head known to cancel); a taking several times over is a multiplication and a taking
readoff   as plain, but a division by one is read off, not worked (proposed by the review, after the counts were known)
every     each multiplication, taking or division of one figure, by one or not
repeated  as worked, but a taking several times over is that many takings, one at a time
columns   each operation on a whole column
guo       Guo Shuchun's (2020, n. 40): each multiplication or division of a whole column, each taking of one column from another, each step of finding a thing; placing rods not counted here
```

## The decisions

```console
$ jiuzhang decisions
The choices this reconstruction makes that the text does not fix.

1. The board. A column is a list of signed whole numbers, top to bottom, the
   實 last; columns are counted from the right, as the text sets them. Red
   and black rods are the signs; an empty position is zero. Nothing about
   the board's size or the rods' layout is assumed by the checks.

2. The 正負術 is signed arithmetic. The text's rule of signs (同名相除，異名
   相益，正無入負之，負無入正之 …) is read as what taking and adding signed
   numbers does, including the cases of an empty position (無入). `jiuzhang`
   does not test the rule; it uses it.

3. 徧乘 and 直除. "以右行上禾徧乘中行而以直除" is read as: multiply every
   figure of the middle column by the head of the right, then take the right
   column from it as many times as empties its head. The scan reads 直除;
   Wikisource's unproofread transcription of it drops 直 throughout.

4. The last step. The text finds each thing through the 法: the 法 times a
   column's 實, less the 實s already found, divided by the column's own
   count; then "實皆如法". Whether, and how, the 法 and 實s are reduced
   before the answer is written is not said. `jiuzhang answers` tries three
   readings: as the board leaves them; the 法 and all the 實s divided by what
   they share (約); each answer in its lowest terms.

5. Head (頭位) is the first position of the column worked on that is not
   empty, above the 實.

6. Counting 算. What one 算 is, nobody says. `jiuzhang rules` lists the
   candidates; every count is reported under every rule, and none is chosen.
   No rule was written to make a count come out, with one exception, said
   wherever it appears: `readoff`, proposed by the independent review after
   Dai Zhen's 145 and 99 were known, gives both. Under the others his 99
   comes out 98 or 101, and is reported so. `guo` is Guo Shuchun's rule
   (2020, n. 40), without the placings of rods.

7. Readings in the alternative. Where a clause of the commentary can be read
   more than one way, the page gives the readings, and `jiuzhang readings`
   runs every combination. The readings offered are the transcriber's; a
   reading not offered is not tried, and that none of those offered works
   does not show that none does.

8. Units. Each page works in one unit, said in a note (升 where the text
   mixes 斗 and 升, 寸 for Problem 13, 兩 for Problem 9, 錢 for coins).

9. The text. Figures are read from the scans of the Sibu congkan edition (the
   Weiboxie printing, with Liu Hui's commentary, Li Chunfeng's notes and Dai
   Zhen's collation notes), checked against the Wikisource transcription of
   those pages, which is marked unproofread. Where they differ, the scan is
   followed and the line has a note.
```
