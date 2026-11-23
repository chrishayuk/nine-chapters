# 2. Setting the board

Problem 1 begins:

> 今有上禾三秉，中禾二秉，下禾一秉，實三十九斗，上禾二秉，中禾三秉，下禾一秉，實三十四斗，上禾一秉，中禾二秉，下禾三秉，實二十六斗，問上中下禾實一秉各幾何

Suppose 3 bundles of top-grade grain, 2 of middle-grade and 1 of low-grade
yield 39 斗 of grain; 2 top, 3 middle and 1 low yield 34 斗; 1 top, 2 middle
and 3 low yield 26 斗. How much grain does one bundle of each grade yield?

And the procedure says how to set it:

> 置上禾三秉，中禾二秉，下禾一秉，實三十九斗，於右方，中左禾列如右方

Set 3 bundles of top grain, 2 of middle, 1 of low, and the yield 39 斗, on
the right; set the middle and left columns of grain as the right.

Each harvest is a condition, and each condition is a column. `Things` names
the positions, top to bottom, and last the 實. `Set` sets a column, every
position from the top.

```jz
No. 8.1
Things 上禾 中禾 下禾 實
Set right   3 2 1 39          "置上禾三秉，中禾二秉，下禾一秉，實三十九斗，於右方"
Set middle  2 3 1 34          "中左禾列如右方"
Set left    1 2 3 26
```

```
No. 8.1
Things 上禾 中禾 下禾 實
Set right   3 2 1 39          "置上禾三秉，中禾二秉，下禾一秉，實三十九斗，於右方"
Set middle  2 3 1 34          "中左禾列如右方"
Set left    1 2 3 26
          left  middle  right
    上禾     1       2      3
    中禾     2       3      2
    下禾     3       1      1
    實      26      34     39
```

The board is printed the way it lies: the right-hand column on the right.
The words in quotation marks are the source's, kept beside the line they
give. `No. 8.1` is a heading.

A board of three has `right`, `middle` and `left`. A larger board counts
its columns from the right: `right`, `2nd`, `3rd`, `4th`, and the last is
`left`. The text's own names (`右行`, `中行`, `左行`, `第二行`) can be used
instead.

## A condition that is not a sum

Problem 2's conditions are not written as sums:

> 今有上禾七秉，損實一斗，益之下禾二秉，而實十斗，下禾八秉，益實一斗，與上禾二秉，而實十斗

Suppose 7 bundles of top-grade grain, their yield lessened by 1 斗, with 2
bundles of low-grade grain added, yield 10 斗; 8 bundles of low-grade grain,
their yield increased by 1 斗, with 2 bundles of top-grade, yield 10 斗.

The procedure says only 如方程，損之曰益，益之曰損: as 方程, and what is
taken off is added, what is put on is taken. So the first column's 實 is
10 + 1 and the second's 10 − 1. Here Liu Hui states the columns himself, and
`Gives` records what he says the board holds. It is checked:

```jz
No. 8.2
Things 上禾 下禾 實
Set right  7 2 11        "損之曰益"
Set left   2 8 9         "益之曰損"
Gives right 7 2 11       "今按實云上禾七秉，下禾二秉，實十一斗"
Gives left  2 8 9        "上禾二秉，下禾八秉，實九斗也"
```

```
No. 8.2
Things 上禾 下禾 實
Set right  7 2 11        "損之曰益"
Set left   2 8 9         "益之曰損"
          left  right
    上禾     2      7
    下禾     8      2
    實       9     11
Gives right 7 2 11       "今按實云上禾七秉，下禾二秉，實十一斗"  ✓
Gives left  2 8 9        "上禾二秉，下禾八秉，實九斗也"          ✓

2 figures checked, 2 right, 0 wrong.
```

A `Gives` line that is wrong is marked ✗, with what the board holds. Most
problems state no column at all, and their pages have no `Gives`: only the
answer can be checked.
