# 9. Reading a corrupt passage

After his new method, Liu Hui works Problem 18 by the old method "made
short" (要約), clause by clause, and says it uses 77 算. Dai Zhen calls the
passage corrupt and impossible to follow (訛舛不可通). Some clauses cannot be
read as they stand:

> 先置第三行以減第四行，及減第三行

First set the 3rd column, and take it from the 4th, and take [from?] the
3rd.

As written, the second clause takes the 3rd column from itself. And 減, to
take, does not say how many times: once, until the head is gone, as far as
it goes, or something else.

## Readings in the alternative

Where a clause can be read more than one way, the page gives each reading on
the same line, separated by `|`. The page runs with the first. `Nothing`
is a reading that does nothing: the clause is corrupt, or says nothing new.
Besides once, `until the head is gone`, `as far as it goes` and
`crosswise`, a taking can be `in turn` (相減): each column taken from the
other, the larger head from the smaller, until one head is gone, with no
multiplying.

A small page shows how it works. Suppose a commentator had written that the
middle column of Problem 1 comes to 0 5 1 24, and we did not know whether he
multiplied first, or how he took:

```jz readings
Things 上禾 中禾 下禾 實
Set right   3 2 1 39
Set middle  2 3 1 34
Set left    1 2 3 26
Multiply middle by the head of right | Nothing
Take right from middle until the head is gone | Take right from middle in turn | Take right from middle crosswise
Gives middle 0 5 1 24
```

```
6 readings (6 run, the rest counted with one that stopped or disagreed at the same line):
  5 can be done to the end; 4 of them agree with every figure the source states.
  1 stop at line 6
      1: 2 at the 上禾 of middle does not go by 3 exactly
  1 run past line 7 but disagree with the figure there

agrees:
  line   5: Multiply middle by the head of right
  line   6: Take right from middle until the head is gone

agrees:
  line   5: Multiply middle by the head of right
  line   6: Take right from middle in turn

agrees:
  line   5: Nothing
  line   6: Take right from middle in turn

agrees:
  line   5: Nothing
  line   6: Take right from middle crosswise
```

`jiuzhang readings` runs every combination, and lists those that can be
done to the end and agree with every figure stated. Here four of the six do.
The figure does not decide between them.

## The 77

Liu Hui's passage, read every way this project could think of, has 100000000
combinations. The figure it states is the price of the 黍, 6, at the end:

```console
$ jiuzhang readings corpus/liu-hui-18-old.jz
100000000 readings (912143 run, the rest counted with one that stopped or disagreed at the same line):
  229906 can be done to the end; 0 of them agree with every figure the source states.
  30000000 stop at line 17
      10000000: 2 at the 麻 of 4th does not go by 3 exactly
      10000000: 3 at the 麻 of 3rd does not go by 2 exactly
      10000000: the 麻 of 4th is empty: nothing to take with
  21250000 stop at line 20
      8750000: the 麻 of 3rd is empty: nothing to take with
      6875000: the 麻 of 4th is empty: nothing to take with
      1875000: 1 at the 麻 of left does not go by 3 exactly
      … and 3 other reasons
  18625000 stop at line 22
      11125000: the 麻 of 2nd is empty: nothing to take with
      2875000: 1 at the 麻 of 3rd does not go by 7 exactly
      1625000: 3 at the 麻 of 3rd does not go by 7 exactly
      … and 10 other reasons
  9675000 stop at line 24
      2737500: the 麻 of 3rd is empty: nothing to take with
      2312500: the 麻 of 4th is empty: nothing to take with
      1500000: the 麻 of left is empty: nothing to take with
      … and 13 other reasons
  7600000 stop at line 26
      4250000: the 麻 of 4th is empty: nothing to take with
      1835000: 1 at the 麻 of left does not go by 2 exactly
      836250: the 麥 of 4th is empty: nothing to take with
      … and 2 other reasons
  5856250 stop at line 28
      5020000: the 麻 of left is empty: nothing to take with
      368250: the 麥 of left is empty: nothing to take with
      154250: -1 at the 麻 of 4th does not go by 2 exactly
      … and 7 other reasons
  1707900 stop at line 29
      1435800: the 麻 of left is empty: nothing to take with
      87000: 7 at the 麻 of 2nd does not go by -2 exactly
      59400: 7 at the 麻 of 2nd does not go by 2 exactly
      … and 12 other reasons
  784350 stop at line 31
      334760: the 麻 of left is empty: nothing to take with
      163410: the 麥 of left is empty: nothing to take with
      28500: -7 at the 麥 of 2nd does not go by 3 exactly
      … and 66 other reasons
  3993480 stop at line 32
      3993480: 2nd cannot be halved: a figure in it is odd
  278114 stop at line 34
      224904: the 麥 of 2nd is empty: nothing to take with
      23618: the 麻 of 2nd is empty: nothing to take with
      4758: 3 at the 麻 of 4th does not go by -4 exactly
      … and 90 other reasons
  229906 run past line 38 but disagree with the figure there
```

Most readings stop part way: a column has to be taken from another whose
head does not divide it, or a column the text says to halve has an odd
figure in it. 229906 can be done to the end, and none of them gives the 黍's
price. So none reaches a board that 77 算 could be counted on.

This shows that none of the readings offered works. It does not show that
no reading could. The readings are the transcriber's, and a reading not
offered is not tried. The passage may be corrupt in a way none of them
repairs.

Others have repaired it. Chemla and Guo (2004) emend its opening, 及減第三行
among other words, and restore 六 for 空; Guo Shuchun (2020) works the
emended passage through and tallies 77, counting each placing of rods, each
operation on a whole column and each division as one 算. Those emendations are
not among the readings on this page, which is the text as printed. What the
census shows is the unemended passage failing, as Dai Zhen said it would; the
emended passage is the next page to write. See `docs/literature-check.md`.

## The 124

Liu Hui's new method takes the 實s away first, so that each column says
only that some things are worth others, and reads off the rates two things
at a time. He says it takes 124 算. Dai Zhen calls this passage corrupt too
(亦訛舛不可通).

```console
$ jiuzhang readings no liu-hui-18-new
6115295232 readings (193 run, the rest counted with one that stopped or disagreed at the same line):
  0 can be done to the end; 0 of them agree with every figure the source states.
  2038431744 stop at line 19
      2038431744: the 麻 of 4th is empty: nothing to take with
  1358954496 stop at line 24
      1358954496: the 實 of left is empty: nothing to take with
  2038431744 stop at line 25
      679477248: 95 at the 實 of left does not go by -91 exactly
      679477248: the 實 of 3rd is empty: nothing to take with
      679477248: 95 at the 實 of left does not go by 4 exactly
  679477248 stop at line 27
      339738624: the 實 of 4th is empty: nothing to take with
      339738624: the 實 of left is already empty
```

Every reading stops early, by line 27 of the page, long before the first
pair of rates Liu Hui states (菽 5 against 荅 3, on line 43). So the 124
is not tested at all: the passage is not followed far enough for any of its
figures, or its count, to be compared. The last part of the method,
after the rates are found, can be done. The one column Liu Hui states there
is what the board gives; the rest is arithmetic in fractions, off the board,
and the page's note checks it by hand:

```console
$ jiuzhang --plain no liu-hui-18-rates
No. 8.18 (Liu Hui: the new method, the last part)
% Sibu congkan pp. 33-34. After the rates are found (麻 7, 麥 4, 菽 3, 荅 5, 黍 6), Liu Hui sets the
% 3rd column afresh and takes the 4th, as first set, from it, so that one column with 麻 at its
% head is left; the rates turn every thing in it into 麻, and the 麻 price follows.
Things 麻 麥 菽 荅 黍 實
Set right 9 7 3 2 5 140
Set 2nd   7 6 4 5 3 128
Set 3rd   3 5 7 6 4 116                     "更置第三行"
Set 4th   2 5 3 9 4 112
Set left  1 3 2 8 5  95
Take 4th from 3rd.                           "以第四行減之"
Gives 3rd 1 0 4 -3 0 4                       "餘有麻一斗菽四斗荅三斗負下實四正"  ✓
"求其同爲麻之數以菽率三荅率五各乘菽荅斗數如麻率七而一菽得一斗七分斗之五正荅得二斗七分斗之一負則菽荅化爲麻以并之令同名相從異名相消餘爲定麻七分斗之四以爲法置下實四爲實以分母乘之實得二十八而分子化爲法矣以法除得七卽麻一斗之價"
% The rest is arithmetic in fractions, off the board; checked by hand: 4 菽 × 3/7 = 1 5/7 (正),
% 3 荅 × 5/7 = 2 1/7 (負), 1 + 1 5/7 − 2 1/7 = 4/7 (the 法), 4 × 7 = 28, 28 ÷ 4 = 7, the 麻.

1 figure checked, 1 right, 0 wrong.
```

## Two readings that work

Guo Shuchun's emended passage, worked as his notes work it, is on the page
`guo-18-old`. It runs to the end: both halvings go, the 黍 comes out six, and
the prices check.

```console
$ jiuzhang --plain no guo-18-old
No. 8.18 (Guo Shuchun's reading of Liu Hui's 77)
% Liu Hui's old method "made short", as Guo Shuchun emends and works it: 九章筭術譯注 (revised, 2020),
% ch. 8, last problem, notes 22-40; the emendations as in Chemla and Guo, Les neuf chapitres (2004),
% pp. 652-655, nn. 93-107. Read from a transcription of Guo 2020 (figure 8-7 was not in it), by a second
% model, OpenAI Codex, which marked what it read and what it reconstructed; see docs/guo-77.md. The
% operations below are Guo's, block by block. No board figures are asserted that the sources do not
% state: only that the text's halvings go, that the 黍 comes out six, and the answer.
"以舊術爲之凡應置五行今欲要約先置第三行減以第四行又減第五行次置第二行以第二行減第一行又減第四行去其頭位餘可半次置右行及第二行去其頭位次以右行去第四行頭位次以左行去第二行頭位次以第五行去第一行頭位次以第二行去第四行頭位餘可半以右行去第二行頭位以第二行去第四行頭位餘約之爲法實實如法而一得六卽有黍價以法治第二行得荅價右行得菽價左行得麥價第三行麻價如此凡用七十七算"
% The emended text (Guo 2020; Chemla and Guo 2004): 減以 for 以減 (n. 93), 又減第五行 for 及減第三行 (n. 94),
% 第一行 for 第三行 (n. 95), 又減第四行 supplied (n. 96), 餘可半 supplied (n. 97), 及第二行 supplied (n. 97),
% 右行去第四行 for 第四行減左行 (n. 98), 第四行及 deleted (n. 99), 去第一行 for 減第二行 (n. 100), 餘可半
% moved (n. 101), a clause supplied (n. 102), 法實 for 法 (n. 103), 六 for 空 (n. 104, Dai Dunyuan),
% 治 for 減 (n. 105, Li Jimin), and two moves of 右行得菽價 / 左行 (nn. 106-107).
Things 麻 麥 菽 荅 黍 實
Set right 9 7 3 2 5 140
Set 2nd   7 6 4 5 3 128
Set 3rd   3 5 7 6 4 116
Set 4th   2 5 3 9 4 112
Set left  1 3 2 8 5  95
% block 1 (n. 23-24, 3 算)
Take 4th from 3rd.                       "先置第三行減以第四行"
Take 3rd from left.                      "又減第五行"
% block 2 (n. 25-27, 4 算)
Take 2nd from right.                     "次置第二行以第二行減第一行"
Take right from 4th.                     "又減第四行去其頭位"
Halve 4th.                               "餘可半"
% block 3 (n. 28, 11 算)
Take 3rd from right 2 times.             "次置右行及第二行去其頭位"
Take 3rd from 2nd 7 times.
% block 4 (n. 29, 3 算)
Take right from 4th 2 times.             "次以右行去第四行頭位"
% block 5 (n. 30, 3 算)
Take left from 2nd 2 times.              "次以左行去第二行頭位"
% block 6 (n. 31, 3 算)
Multiply right by 3.                     "次以第五行去第一行頭位"
Take left from right.
% block 7 (n. 32, 3 算)
Add 2nd to 4th.                          "次以第二行去第四行頭位"
Halve 4th.                               "餘可半"
% block 8 (n. 33, 21 算)
Multiply 2nd by 25.                      "以右行去第二行頭位"
Take right from 2nd 20 times.
% block 9 (n. 34, 4 算)
Multiply 4th by 140.                     "以第二行去第四行頭位"
Take 2nd from 4th 2 times.
% block 10 (n. 35): reduced by 62, then 實 by 法
Divide 4th by 62.                        "餘約之爲法實"
Gives 4th 0 0 0 0 -5 -30 | Gives 4th 0 0 0 0 5 30    "實如法而一得六卽有黍價"  ✓
% blocks 11-14 (n. 36-39, 3 + 5 + 7 + 4 算): the prices found from the 2nd, right, left and 3rd columns,
% as the text says; the board is already in that order, so `Solve by cross` only works back.
Solve by cross.                          "以法治第二行得荅價右行得菽價左行得麥價第三行麻價"  7, 4, 3, 5, 6; 5 steps
Answer 7, 4, 3, 5, 6                                                   ✓
Count 77                                 "如此凡用七十七算"            no rule counts 77: plain 124, worked 122, readoff 123, every 99, repeated 214, columns 23, guo 67
% Guo's tally: 1+3+4+11+3+3+3+3+21+4+2+3+5+7+4 = 77 (n. 40). His 算 includes placing rods (布算),
% which this page does not model: see `jiuzhang counts`.

2 figures checked, 2 right, 0 wrong.
```

His note 40 says what one 算 is: placing rods, multiplying or dividing a
whole column, taking one column from another once, dividing a 實 by a 法.
The `guo` rule counts all of these except the placings, which the board does
not model: 67. His 77 leaves ten for placing rods.

Roger Hart reads 算 as rods: the most the board holds while the passage is
worked. His path is on the page `hart-18-old`, each `Rods` line the count he
reports after a step.

```console
$ jiuzhang --plain no hart-18-old
No. 8.18 (Hart's reading of Liu Hui's 77: rods)
% Roger Hart, "Tracing Practices Purloined by the 'Three Pillars'", Korean Journal for the History of
% Science 34 (2012), pp. 323-329: 算 read as counting rods, and a reconstruction of the old method on which
% the board never holds more than 77. Columns are set as they are needed; others stay on the board. Each
% Rods line is the count Hart reports after the step; rods are counted as jiuzhang counts them (a digit
% to five is that many rods, six to nine one for five and the rest; no rod for a sign), which is the rule
% his counts follow. Read by a second model, OpenAI Codex; see docs/guo-77.md. Hart's prose says "subtract"
% at steps 7 and 9 where his formulas add; the formulas are followed.
Things 麻 麥 菽 荅 黍 實
% 1
Set 3rd   3 5 7 6 4 116
Set 4th   2 5 3 9 4 112
Rods 44                                                                ✓
% 2
Take 4th from 3rd.
Rods 35                                                                ✓
% 3
Set left  1 3 2 8 5  95
Rods 60                                                                ✓
% 4: 3 left - 2 (4th)
Multiply left by 3.
Take 4th from left 2 times.
Rods 45                                                                ✓
% 5
Set right 9 7 3 2 5 140
Rods 68                                                                ✓
% 6
Take 4th from right.
Rods 60                                                                ✓
% 7: (right + 2 left) / 5
Add left to right 2 times.
Divide right by 5.
Rods 53                                                                ✓
% 8: 4 (4th) - 3 (3rd)
Multiply 3rd by -3.
Add 4th to 3rd 4 times.
Rods 69                                                                ✓
% 9: (3rd + 20 left) / 3
Add left to 3rd 20 times.
Divide 3rd by 3.
Rods 75                                                                ✓
% 10: 3rd + 5 right
Add right to 3rd 5 times.
Rods 53                                                                ✓
% 11: the 2nd set: the most the board holds
Set 2nd   7 6 4 5 3 128
Rods 77                                                                ✓
% 12: 4 (4th) - 3 (2nd)
Multiply 2nd by -3.
Add 4th to 2nd 4 times.
Rods 71                                                                ✓
% 13: (2nd + 2 left) / 3
Add left to 2nd 2 times.
Divide 2nd by 3.
Rods 67                                                                ✓
% 14: (2nd + 5 right) / 2
Add right to 2nd 5 times.
Divide 2nd by 2.
Rods 62                                                                ✓
% 15: (30 (2nd) - 4 (3rd)) / 62: the 黍 alone, worth 6
Multiply 2nd by 30.
Take 3rd from 2nd 4 times.
Divide 2nd by 62.
Gives 2nd 0 0 0 0 1 6                                                  ✓
Rods 56                                                                ✓

16 figures checked, 16 right, 0 wrong.
```

All fifteen agree. But a step is not done in an instant. `jiuzhang rods`
asks how many rods are on the board while a step is being worked:

```console
$ jiuzhang rods no hart-18-old
The most rods on the board at any moment, by how a step is worked:
     77  at the end of each step (a run of lines on one column, up to an assertion or a note)
     86  each line worked on the whole column at once (a product set down, then taken from)
     78  each step worked a position at a time, straight to its new figure, top down
     77  the same, bottom up
     77  each step worked a position at a time, in the best order for it
     86  each take done singly, one multiple at a time, on the whole column
     77  each take done singly, the positions in the best interleaving for each step
```

Worked a position at a time, straight to the new figures, from the bottom,
the board never holds more than 77. Worked from the top, it holds 78 for a
moment; with the whole column multiplied first, 86. Hart's count holds
under one way of working, and the text does not say which way was used.

The last two lines take a repeated subtraction literally. `Add 4th to 2nd 4
times` is four additions, each of one 4th column. Done to the whole column
each time, the board reaches 86. Done a position at a time, each position
passing through every figure its own additions give it, the positions can
be interleaved, and `jiuzhang rods` searches every interleaving for the one
that needs fewest rods. For Hart's working that is still 77.

Neither reading gives 77 on the other's path. `jiuzhang counts` sets them
side by side:

```console
$ jiuzhang counts
dai-zhen-18  8.18 (Dai Zhen, 訂訛: 145 and 99)
  Count 145  plain     145   ✓
  Count 145  worked    140
  Count 145  readoff   145   ✓
  Count 145  every     175
  Count 145  repeated  140
  Count 145  columns   15
  Count 145  guo       54
  Count 99   plain     101
  Count 99   worked    98
  Count 99   readoff   99   ✓
  Count 99   every     81
  Count 99   repeated  295
  Count 99   columns   15
  Count 99   guo       88

guo-18-new  8.18 (Guo Shuchun's reading of Liu Hui's 124)
  16 readings: 5 can be done to the end, 1 agree with every figure the source states
  Count 124  plain     199
  Count 124  worked    199
  Count 124  readoff   199
  Count 124  every     123
  Count 124  repeated  823
  Count 124  columns   35
  Count 124  guo       228

guo-18-old-placed  8.18 (Guo Shuchun's reading of Liu Hui's 77, each column set when first used)
  2 readings: 2 can be done to the end, 1 agree with every figure the source states
  Count 77   plain     124
  Count 77   worked    122
  Count 77   readoff   123
  Count 77   every     99
  Count 77   repeated  214
  Count 77   columns   23
  Count 77   guo       67

guo-18-old  8.18 (Guo Shuchun's reading of Liu Hui's 77)
  2 readings: 2 can be done to the end, 1 agree with every figure the source states
  Count 77   plain     124
  Count 77   worked    122
  Count 77   readoff   123
  Count 77   every     99
  Count 77   repeated  214
  Count 77   columns   23
  Count 77   guo       67

Problem 18 by each method, counted under each rule, and the board it needs:
               plain   worked  readoff    every repeated  columns      guo    rods  places
  text           158      158      158      133   180403       25    89816     160      11
  reducing       188      179      183      163      504       33      171     138       4
  cross          145      140      145      175      140       15       54     149      11
  one            101       98       99       81      295       15       88     122       4
  rods: the most rods on the board at once; places: the most digits in any figure.

liu-hui-18-old: Liu Hui says 77 算. Of 100000000 readings of his working, 229906 can be done to the end and 0 agree with the figures he states, so there is nothing to count.
liu-hui-18-new: Liu Hui says 124 算. Of 6115295232 readings of his working, 0 can be done to the end and 0 agree with the figures he states, so there is nothing to count.

Two published readings of Liu Hui's 77 (see docs/guo-77.md):
  Guo Shuchun (2020), 算 as operations, on his emended text: runs to the prices; under his rule 67 operations, leaving 10 of the 77 for placing rods, which are not modelled (5 columns are set on this page); the board holds 116 rods at most
  Hart (2012), 算 as rods, on his path: runs to the price of 黍, and every count of rods he reports agrees; at the end of each step 77 rods at most; worked a column at a time 86, a position at a time top down 78, bottom up 77, in the best order 77; under Guo's rule 59 operations and 5 placings

Liu Hui's 124, on Guo Shuchun's (2020) working of the new method, to the rates (guo-18-new):
  runs, and every figure the text and his notes state agrees; as operations, before the last part: plain 199, worked 199, readoff 199, every 123, repeated 823, columns 35, guo 228; no rule gives 124
  as rods: at the end of each step 124 at most, first at line 27, just before 訖廢去第三行; worked a column at a time 124, a position at a time top down 130, bottom up 124, in the best order 124
```

## The 124, as Guo works it

Liu Hui gives a second number: his new method takes 124 算. As printed, the
passage stops by its fourth clause under every reading offered (above). Guo
Shuchun works it through (2020, nn. 41-62), and his working is the page
`guo-18-new`. It runs, and every figure the text and his notes state agrees:
菽 5 against 荅 3, 荅 6 against 黍 5, 麥 3 against 菽 4, 麻 4 against 麥 7.

Counted as operations, it cannot be 124. The part that finds the rates,
before the last part, is already 228 under Guo's own rule, which counts a
column taken 35 times as 35 takings, as his tally of the 77 does. Under every
rule it is more than 124, or, for `columns`, far fewer: `every` comes to 123,
and the last part's first taking alone adds 6.

Counted as rods, it is 124:

```console
$ jiuzhang rods corpus/guo-18-new.jz
The most rods on the board at any moment, by how a step is worked:
    124  at the end of each step (a run of lines on one column, up to an assertion or a note)
    124  each line worked on the whole column at once (a product set down, then taken from)
    130  each step worked a position at a time, straight to its new figure, top down
    124  the same, bottom up
    124  each step worked a position at a time, in the best order for it
    133  each take done singly, one multiple at a time, on the whole column
    125  each take done singly, the positions in the best interleaving for each step
    line 29: Take 3rd from left 3 times. needs 125 at best, its takes done singly, more than the board holds at any step's end
```

The new method sets all five columns, 116 rods, and the text fixes every
operation up to the peak: 減 once, 去 until the 實 is gone, 不足減乃止 as far
as it goes, with 右行 read as 左行, as Guo reads it. The board holds 124 rods
after 次以左行減第三行下位 and 次以第三行去左行下位, and the text then says
訖廢去第三行: drop the 3rd column. It never holds more; the last part, with
the columns set afresh, needs about half as many. As with Hart's 77, the
bound holds if each column is worked a position at a time from the bottom,
or a whole column at a time, and not from the top (130).

It does not hold if every take is done singly. 次以第三行去左行下位 takes
the 3rd column from the left three times, until the left column's 實 (3)
is gone. The board holds 124 before it and 124 after; taken one at a time,
the left column passes through

    -22  3  -90  77   5  3     26 rods
    -45  6 -184 157  10  2     32
    -68  9 -278 237  15  1     35
    -91 12 -372 317  20  0     26

and on the whole column the board reaches 133. A position at a time, in
the best interleaving there is, it reaches 125: one more than Liu Hui's
number. No other step goes over 124 with its takes done singly. To
stay at 124, at least one figure must be taken three times at once: the
麻 (-22 straight to -91), the 菽 (-90 to -372) or the 荅 (77 to 317), any
one of them, is enough.

So the 124 needs the reckoner to take a multiple at a time, at one place
at least, where the text's 去 could be read as one subtraction after
another. Hart's 77 needs nothing of the kind. Whether a Han reckoner set a
multiple straight down at a position, or took the column again and again,
the text does not say; the numbers say only that 124 fits the first, and
misses the second by one rod.

So on Guo's own path the 124 is a count of rods, not of operations, and
nobody built the path to make it so: up to the peak, it is the text's.

Little of this is new. Hart (2012, n. 34) reads both 77 and 124 as rods;
Hudeček (2008, p. 206) prints this board's figures; Wei Xuegang and Guo Shirong (2016)
say they begin from the new method to settle what 算 means, and their full
text could not be read, so they may well have counted this peak. A second
model's search ([`docs/literature-124.md`](../docs/literature-124.md)) did
not find the peak stated in anything it could read.
