# 4. The 法 and the 實

When the left column holds one thing, the text says:

> 左方下禾不盡者，上爲法，下爲實，實卽下禾之實

What is left of the low grain in the left column: the upper figure is the 法
(divisor), the lower the 實 (dividend); this 實 is the 實 of the low grain.

So the 法 is 36 and the low grain's 實 is 99. The text does not divide yet.
It finds the other things' 實s over the same 法:

> 求中禾，以法乘中行下實，而除下禾之實，餘如中禾秉數而一，卽中禾之實

To find the middle grain: multiply the bottom 實 of the middle column by the
法, take away the 實 of the low grain, and divide what is left by the number
of bundles of middle grain; that is the 實 of the middle grain.

The middle column is 5 1 24. The 法 times 24 is 864; less the low grain's
實 (one bundle of it, 99) is 765; divided by the 5 bundles of middle grain,
153. The top grain is found the same way from the right-hand column, 3 2 1
39: 36 times 39 is 1404, less 2 times 153 and 1 times 99 is 999, and 999 by
3 is 333. Then:

> 實皆如法，各得一斗

Divide each 實 by the 法, and each gives so many 斗.

`Solve` does this: the rest of the procedure, from wherever the page has got
to. It prints the 法 and the 實s it finds, and how many steps it took.
`Answer` gives the text's answer, thing by thing from the top:

```jz --plain
Things 上禾 中禾 下禾 實
Set right   3 2 1 39
Set middle  2 3 1 34
Set left    1 2 3 26
Multiply middle by the head of right.
Take right from middle until the head is gone.
Multiply left by the head of right.
Take right from left until the head is gone.
Multiply left by the 中禾 of middle.
Take middle from left until the head is gone.
Solve.
Answer 9 1/4, 4 1/4, 2 3/4                       "上禾一秉九斗四分斗之一，中禾一秉四斗四分斗之一，下禾一秉二斗四分斗之三"
```

```
Things 上禾 中禾 下禾 實
Set right   3 2 1 39
Set middle  2 3 1 34
Set left    1 2 3 26
Multiply middle by the head of right.
Take right from middle until the head is gone.
Multiply left by the head of right.
Take right from left until the head is gone.
Multiply left by the 中禾 of middle.
Take middle from left until the head is gone.
Solve.                                          法 36; 實 333 153 99; 3 steps
Answer 9 1/4, 4 1/4, 2 3/4                       "上禾一秉九斗四分斗之一，中禾一秉四斗四分斗之一，下禾一秉二斗四分斗之三"  ✓  written over 4: the 法 and 實s divided by what they share (約), 4; or each in its lowest terms (not as the board leaves it, 9 9/36)

1 figure checked, 1 right, 0 wrong.
```

`Solve` on a board that has not been worked does all of it, by the text's
procedure. `jiuzhang solve` does the same from the command line: the
columns from the right, separated by `/`, the 實 last.

```console
$ jiuzhang solve 3 2 1 39 / 2 3 1 34 / 1 2 3 26
Things #1 #2 #3 實
Set right 3 2 1 39
Set middle 2 3 1 34
Set left 1 2 3 26
        left  middle  right
    #1     1       2      3
    #2     2       3      2
    #3     3       1      1
    實    26      34     39
Solve by text.       法 36; 實 333 153 99; 9 steps
    take right from middle, 2 times
        left  middle  right
    #1     1       ·      3
    #2     2       5      2
    #3     3       1      1
    實    26      24     39
    take right from left
        left  middle  right
    #1     ·       ·      3
    #2     4       5      2
    #3     8       1      1
    實    39      24     39
    take middle from left, 4 times
        left  middle  right
    #1     ·       ·      3
    #2     ·       5      2
    #3    36       1      1
    實    99      24     39

plain 37, worked 35, every 35, repeated 39, columns 9
```

It shows each taking with the board after it. The multiplying a taking
needs is done first, and is not shown on its own line; nor is the working
back. The last line is the work counted under each rule for counting 算
(chapter 8).

## How the answer is written

The text writes 9 1/4 (九斗四分斗之一), not 9 9/36, which is 333 over 36 as
the board leaves it. Somewhere the 法 and the 實s were divided by 9. The text
does not say where. But Problem 2 writes its answer over 52:

```jz --plain
Things 上禾 下禾 實
Set right  7 2 11
Set left   2 8 9
Solve.
Answer 1 18/52, 41/52     "上禾一秉實一斗五十二分斗之十八，下禾一秉實五十二分斗之四十一"
```

```
Things 上禾 下禾 實
Set right  7 2 11
Set left   2 8 9
Solve.               法 52; 實 70 41; 4 steps
Answer 1 18/52, 41/52     "上禾一秉實一斗五十二分斗之十八，下禾一秉實五十二分斗之四十一"  ✓  written over 52: the 法 as the board leaves it, 52 (not in lowest terms, 1 9/26)

1 figure checked, 1 right, 0 wrong.
```

1 18/52 is not in lowest terms: that would be 1 9/26. So the text does not
reduce each answer on its own. What fits both problems is 約 (*yue*): divide
the 法 and all the 實s together by what they have in common. In Problem 1
they share 9, and 36 becomes 4. In Problem 2, 52, 70 and 41 share nothing,
and 52 stays. That is why `Answer` checks how a fraction is written, not
only its value: `1 9/26` would be the same number, and would hide this.

`jiuzhang answers` tries three readings of the last step on every answer
with a fraction:

```console
$ jiuzhang answers
How each answer is written, and which reading of the last step writes it so.
board: every 實 over the 法 as the board leaves it.  約: the 法 and all the 實s divided by what
they share.  lowest: each answer in its lowest terms.  ✓: writes it as the text does.

problem  written                   board        約   lowest   法 by the text, after 約
1        1/4 1/4 3/4                36        4 ✓    1/4 ✓   4
2        18/52 41/52                52 ✓     52 ✓   9/26     52
3        9/25 7/25 4/25             25 ✓     25 ✓   9/25 ✓   25
7        13/21 20/21                21 ✓     21 ✓  13/21 ✓   21
9        13/19 5/19                 19 ✓     19 ✓  13/19 ✓   19
10       1/2                         4        2 ✓    1/2 ✓   2
11       6/11 2/11                  11 ✓     11 ✓   6/11 ✓   11
12       6/7 1/7 5/7                 7 ✓      7 ✓    6/7 ✓   7
14       33/111 28/111 17/111 1    666      111 ✓  11/37     111
15       17/23 11/23 10/23          23 ✓     23 ✓  17/23 ✓   23
16       45/122 41/122 97/122      976      122 ✓ 45/122 ✓   122

Of 11 answers with a fraction: board writes 7 as the text does, 約 11, lowest terms 9.
```

As the board leaves them, the 法 and 實s write 7 of the 11 answers as the
text does; each answer in its lowest terms, 9; 約, all 11.
