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
