# 5. 正 and 負

Taking one column from another can leave a figure below nothing. The text
gives a rule for it, after Problem 3, the 正負術 (the rule of signs):

> 正負術曰，同名相除，異名相益，正無入負之，負無入正之，其異名相除，同名相益，正無入正之，負無入負之

When taking a column from another: like signs subtract, unlike signs add; a
positive with nothing opposite becomes negative, a negative with nothing
opposite becomes positive. When adding: unlike signs subtract, like signs
add; a positive with nothing opposite stays positive, a negative stays
negative.

This is what taking and adding signed numbers does, empty positions
included. `jiuzhang` uses it as signed arithmetic; it does not test it. A
figure is written `-3` for 負, and `3` for 正.

## Problem 3

> 今有上禾二秉，中禾三秉，下禾四秉，實皆不滿斗，上取中，中取下，下取上，各一秉而實滿斗

Suppose 2 bundles of top-grade grain, 3 of middle-grade and 4 of low-grade
each yield less than a 斗. If the top takes one bundle of the middle, the
middle one of the low, and the low one of the top, each yields a full 斗.

Liu Hui says how to set it: each grain's own bundles in its own column,
and the bundle it takes, and the 斗, each in its place.

```jz
Things 上禾 中禾 下禾 實
Set right   2 1 0 1      "置上禾二秉爲右行之上"
Set middle  0 3 1 1      "中禾三秉爲中行之中"
Set left    1 0 4 1      "下禾四秉爲左行之下，所取一秉及實一斗各從其位"
Solve.
Answer 9/25, 7/25, 4/25
```

```
Things 上禾 中禾 下禾 實
Set right   2 1 0 1      "置上禾二秉爲右行之上"
Set middle  0 3 1 1      "中禾三秉爲中行之中"
Set left    1 0 4 1      "下禾四秉爲左行之下，所取一秉及實一斗各從其位"
          left  middle  right
    上禾     1       ·      2
    中禾     ·       3      1
    下禾     4       1      ·
    實       1       1      1
Solve.                                                                   法 25; 實 9 7 4; 7 steps
    take right from left
          left  middle  right
    上禾     ·       ·      2
    中禾    -1       3      1
    下禾     8       1      ·
    實       1       1      1
    add middle to left
          left  middle  right
    上禾     ·       ·      2
    中禾     ·       3      1
    下禾    25       1      ·
    實       4       1      1
Answer 9/25, 7/25, 4/25                                                  ✓  written over 25: the 法 as the board leaves it, 25; or each in its lowest terms

1 figure checked, 1 right, 0 wrong.
```

In the first step the left column, multiplied by 2, has nothing at its
middle grain, and the right has 1 there: 正無入負之, and the left's middle
grain becomes -1. In the second, -3 and 3 are added, and 異名相除: they
cancel. When the text's procedure takes a column a negative number of times,
`jiuzhang` prints it as adding.

## Problem 6

> 術曰，如方程，置上禾三秉正，下禾十秉負，益實六斗正，次置上禾二秉負，下禾五秉正，益實一斗正，以正負術入之

As 方程: set 3 bundles of top grain 正, 10 of low grain 負, the added 6 斗
正; next set 2 bundles of top grain 負, 5 of low grain 正, the added 1 斗 正;
work it by the rule of signs.

This procedure says what sign every figure has. The page sets them so:

```jz
No. 8.6
Things 上禾 下禾 實
Set right  3 -10 6       "置上禾三秉正，下禾十秉負，益實六斗正"
Set left   -2 5 1        "次置上禾二秉負，下禾五秉正，益實一斗正"
Solve.                   "以正負術入之"
Answer 8, 3              "上禾一秉實八斗，下禾一秉實三斗"
```

```
No. 8.6
Things 上禾 下禾 實
Set right  3 -10 6       "置上禾三秉正，下禾十秉負，益實六斗正"
Set left   -2 5 1        "次置上禾二秉負，下禾五秉正，益實一斗正"
          left  right
    上禾    -2      3
    下禾     5    -10
    實       1      6
Solve.                   "以正負術入之"                            法 -5; 實 40 15; 4 steps
    add right to left, 2 times
          left  right
    上禾     ·      3
    下禾    -5    -10
    實      15      6
Answer 8, 3              "上禾一秉實八斗，下禾一秉實三斗"          ✗ the board gives -8 for 8, -3 for 3

1 figure checked, 0 right, 1 wrong.
```

The board gives -8 and -3, not 8 and 3. The question says 3 bundles of top
grain with 6 斗 added are worth 10 of low grain: 3 times 8, and 6, is 30,
which is 10 times 3. Set with the top grain 正 and the low 負, the 6 斗 is on
the low grain's side, and should be 負. So should the 1 斗 of the second
column. With those signs the board gives the text's answer:

```console
$ jiuzhang solve 3 -10 -6 / -2 5 -1
Things #1 #2 實
Set right 3 -10 -6
Set left -2 5 -1
        left  right
    #1    -2      3
    #2     5    -10
    實    -1     -6
Solve by text.      法 -5; 實 -40 -15; 4 steps
    add right to left, 2 times
        left  right
    #1     ·      3
    #2    -5    -10
    實   -15     -6

plain 13, worked 13, every 12, repeated 13, columns 4
```

-40 over -5 is 8, and -15 over -5 is 3.

The scan of the Sibu congkan edition reads 正 in both places; it was read
again to be sure. Another edition (the punctuated Wikisource text) reads 負
in both. The page follows the scan, and the ✗ stays: it is what the board
says about this edition's text. Every other problem in the chapter comes out
as the text says (`jiuzhang verify`).
