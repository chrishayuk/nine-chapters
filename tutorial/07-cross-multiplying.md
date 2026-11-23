# 7. Dai Zhen's board

Problem 8 buys and sells animals:

> 今有賣牛二羊五以買十三豕有餘錢一千，賣牛三豕三以買九羊錢適足，賣羊六豕八以買五牛錢不足六百，問牛羊豕價各幾何

Now someone sells 2 oxen and 5 sheep to buy 13 pigs, and has 1000 cash over;
sells 3 oxen and 3 pigs to buy 9 sheep, and the cash is just enough; sells 6
sheep and 8 pigs to buy 5 oxen, and is 600 cash short. What is the price of
an ox, a sheep and a pig?

The procedure sets what is sold 正 and what is bought 負, the cash over 正
and the cash short 負. The middle column's cash is just enough, and its 實 is
empty.

Liu Hui's note on this problem supposes a working but gives no figure for
it. Dai Zhen calls the note corrupt (此注訛脫不分曉), and works the board
himself, stating every column he gets. He does not multiply one column and
take the other from it. He multiplies each column by the other's head, and
takes one from the other: 互乘, cross-multiplying. `Take … crosswise` does
that.

```jz
No. 8.8 (Dai Zhen)
Things 牛 羊 豕 錢
Set right   2  5 -13 1000
Set middle  3 -9   3    0
Set left   -5  6   8 -600
Take middle from right crosswise.   "先令右行牛二徧乘中行復令中行牛三徧乘右行而以直除"
Gives right 0 33 -45 3000           "得羊三十三正豕四十五負餘錢三千正"
```

```
No. 8.8 (Dai Zhen)
Things 牛 羊 豕 錢
Set right   2  5 -13 1000
Set middle  3 -9   3    0
Set left   -5  6   8 -600
        left  middle  right
    牛    -5       3      2
    羊     6      -9      5
    豕     8       3    -13
    錢  -600       ·   1000
Take middle from right crosswise.   "先令右行牛二徧乘中行復令中行牛三徧乘右行而以直除"
        left  middle  right
    牛    -5       3      ·
    羊     6      -9     33
    豕     8       3    -45
    錢  -600       ·   3000
Gives right 0 33 -45 3000           "得羊三十三正豕四十五負餘錢三千正"  ✓

1 figure checked, 1 right, 0 wrong.
```

Let the 2 oxen of the right multiply the whole of the middle, and the 3 oxen
of the middle the whole of the right, and take: he gets 33 sheep 正, 45 pigs
負, 3000 cash 正. That is 3 times the right less 2 times the middle, so the
page takes the middle from the right.

He goes on with the right as first set, against the left, and then lays out
his two results as a new pair (重列爲左右兩行). `Take` writes its result over
the column it takes from, so the page moves the first result into the
middle and sets the right again. Those two `Set` lines are the page's, not
his.

```jz --plain
No. 8.8 (Dai Zhen)
Things 牛 羊 豕 錢
Set right   2  5 -13 1000
Set middle  3 -9   3    0
Set left   -5  6   8 -600
Take middle from right crosswise.
Gives right 0 33 -45 3000           "得羊三十三正豕四十五負餘錢三千正"
Set middle  0 33 -45 3000
Set right   2  5 -13 1000
Take right from left crosswise.     "次令右行牛二徧乘左行復令左行牛五徧乘右行而以直除"
Gives left  0 37 -49 3800           "得羊三十七正豕四十九負餘錢三千八百正"
Take middle from left crosswise.    "先令右行羊三十三徧乘左行復令左行羊三十七徧乘右行而以直除"
Gives left  0 0 48 14400            "得豕四十八正以爲法錢一萬四千四百正爲實"
Solve.                              "實如法而一得豕價三百轉減而上得羊價五百牛價一千二百"
Answer 1200, 500, 300
```

```
No. 8.8 (Dai Zhen)
Things 牛 羊 豕 錢
Set right   2  5 -13 1000
Set middle  3 -9   3    0
Set left   -5  6   8 -600
Take middle from right crosswise.
Gives right 0 33 -45 3000           "得羊三十三正豕四十五負餘錢三千正"  ✓
Set middle  0 33 -45 3000
Set right   2  5 -13 1000
Take right from left crosswise.     "次令右行牛二徧乘左行復令左行牛五徧乘右行而以直除"
Gives left  0 37 -49 3800           "得羊三十七正豕四十九負餘錢三千八百正"  ✓
Take middle from left crosswise.    "先令右行羊三十三徧乘左行復令左行羊三十七徧乘右行而以直除"
Gives left  0 0 48 14400            "得豕四十八正以爲法錢一萬四千四百正爲實"  ✓
Solve.                              "實如法而一得豕價三百轉減而上得羊價五百牛價一千二百"  法 48; 實 57600 24000 14400; 3 steps
Answer 1200, 500, 300                                                   ✓

4 figures checked, 4 right, 0 wrong.
```

In the second step the heads are 2 and -5, of unlike sign. 2 times the left
less -5 times the right adds them: 異名相除. Every figure he states is what
the board gives. The pig's price is 14400 over 48, 300, and he works back
upwards (轉減而上) for the sheep and the ox.

## Solve by cross

`Solve by cross` works a whole board this way: each column after the first
cross-multiplied with it and taken, then the next, and the things found by
value from the bottom up, without the text's 法.

```jz
Things 牛 羊 豕 錢
Set right   2  5 -13 1000
Set middle  3 -9   3    0
Set left   -5  6   8 -600
Solve by cross.
Answer 1200, 500, 300
```

```
Things 牛 羊 豕 錢
Set right   2  5 -13 1000
Set middle  3 -9   3    0
Set left   -5  6   8 -600
        left  middle  right
    牛    -5       3      2
    羊     6      -9      5
    豕     8       3    -13
    錢  -600       ·   1000
Solve by cross.            1200, 500, 300; 6 steps
    cross-multiply right and middle, and take
        left  middle  right
    牛    -5       ·      2
    羊     6     -33      5
    豕     8      45    -13
    錢  -600   -3000   1000
    cross-multiply right and left, and take
        left  middle  right
    牛     ·       ·      2
    羊    37     -33      5
    豕   -49      45    -13
    錢  3800   -3000   1000
    cross-multiply middle and left, and take
          left  middle  right
    牛       ·       ·      2
    羊       ·     -33      5
    豕     -48      45    -13
    錢  -14400   -3000   1000
Answer 1200, 500, 300      ✓

1 figure checked, 1 right, 0 wrong.
```

The columns come out with the other sign from Dai Zhen's where he took the
other way round: -33 45 -3000 where he has 33 -45 3000, and -48 -14400 where
he has 48 14400. A column and its negative say the same thing, and the
answer is the same. `jiuzhang methods` lists the procedures `Solve` can
follow:

```console
$ jiuzhang methods
text      方程術 as stated: 徧乘 then 直除, right to left; the things found through the 法
reducing  the same, each worked column divided by what its figures have in common (約)
cross     Dai Zhen's 互乘: the two columns multiplied by each other's head, then taken; found by value from the bottom up
one       Dai Zhen's shorter way: work from a column whose head is one; equal heads taken directly
```
