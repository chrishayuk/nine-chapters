# 3. 徧乘 and 直除

The procedure of Problem 1 goes on:

> 以右行上禾徧乘中行，而以直除

With the top grain of the right-hand column, multiply the whole of the
middle column (徧乘), and take directly (直除).

徧乘 is to multiply every figure of a column. 直除 is to take one column
from another, again and again, until the figure at the head is gone.
`Multiply` and `Take` say the same:

```jz
Things 上禾 中禾 下禾 實
Set right   3 2 1 39
Set middle  2 3 1 34
Set left    1 2 3 26
Multiply middle by the head of right.            "以右行上禾徧乘中行"
Take right from middle until the head is gone.   "而以直除"
```

```
Things 上禾 中禾 下禾 實
Set right   3 2 1 39
Set middle  2 3 1 34
Set left    1 2 3 26
          left  middle  right
    上禾     1       2      3
    中禾     2       3      2
    下禾     3       1      1
    實      26      34     39
Multiply middle by the head of right.            "以右行上禾徧乘中行"
          left  middle  right
    上禾     1       6      3
    中禾     2       9      2
    下禾     3       3      1
    實      26     102     39
Take right from middle until the head is gone.   "而以直除"
          left  middle  right
    上禾     1       ·      3
    中禾     2       5      2
    下禾     3       1      1
    實      26      24     39
```

The middle column, multiplied by 3, is 6 9 3 102. Taking the right twice
empties its head, and leaves 5 1 24: the top grain is gone from it. An
empty position is printed `·`.

The head (頭位) is the first position of the column that is not empty. The
multiplying is what makes the taking come out: without it, the middle's
head is 2 and the right's 3, and no number of takings empties it.
`jiuzhang` says so, and stops:

```jz --plain
Things 上禾 中禾 下禾 實
Set right   3 2 1 39
Set middle  2 3 1 34
Set left    1 2 3 26
Take right from middle until the head is gone.
```

```
Things 上禾 中禾 下禾 實
Set right   3 2 1 39
Set middle  2 3 1 34
Set left    1 2 3 26
Take right from middle until the head is gone.  ✗ 2 at the 上禾 of middle does not go by 3 exactly
stopped at line 5: nothing after it was run.
```

## Problem 1, as the text says it

> 又乘其次，亦以直除，然以中行中禾不盡者，徧乘左行，而以直除

Multiply the next as well, and again take directly; then with what is left
of the middle grain of the middle column, multiply the whole of the left
column, and take directly.

```jz
Things 上禾 中禾 下禾 實
Set right   3 2 1 39
Set middle  2 3 1 34
Set left    1 2 3 26
Multiply middle by the head of right.            "以右行上禾徧乘中行"
Take right from middle until the head is gone.   "而以直除"
Multiply left by the head of right.              "又乘其次"
Take right from left until the head is gone.     "亦以直除"
Multiply left by the 中禾 of middle.             "然以中行中禾不盡者，徧乘左行"
Take middle from left until the head is gone.    "而以直除"
```

```
Things 上禾 中禾 下禾 實
Set right   3 2 1 39
Set middle  2 3 1 34
Set left    1 2 3 26
          left  middle  right
    上禾     1       2      3
    中禾     2       3      2
    下禾     3       1      1
    實      26      34     39
Multiply middle by the head of right.            "以右行上禾徧乘中行"
          left  middle  right
    上禾     1       6      3
    中禾     2       9      2
    下禾     3       3      1
    實      26     102     39
Take right from middle until the head is gone.   "而以直除"
          left  middle  right
    上禾     1       ·      3
    中禾     2       5      2
    下禾     3       1      1
    實      26      24     39
Multiply left by the head of right.              "又乘其次"
          left  middle  right
    上禾     3       ·      3
    中禾     6       5      2
    下禾     9       1      1
    實      78      24     39
Take right from left until the head is gone.     "亦以直除"
          left  middle  right
    上禾     ·       ·      3
    中禾     4       5      2
    下禾     8       1      1
    實      39      24     39
Multiply left by the 中禾 of middle.             "然以中行中禾不盡者，徧乘左行"
          left  middle  right
    上禾     ·       ·      3
    中禾    20       5      2
    下禾    40       1      1
    實     195      24     39
Take middle from left until the head is gone.    "而以直除"
          left  middle  right
    上禾     ·       ·      3
    中禾     ·       5      2
    下禾    36       1      1
    實      99      24     39
```

其次, "the next", is read as the left column, as Liu Hui reads it. The
left column now holds one thing only: 36 bundles of low grain yield 99 斗.
The board is a triangle, and the last step can begin.

Dai Zhen read each 徧乘 … 直除 as mutual: the middle multiplied by the
right's head, and the right by the middle's, before the taking, the text
naming only one of the two multiplications. That is `Take … crosswise`
(chapter 7). He gives no figures for Problem 1, so there is nothing on
this problem to check his reading against.
