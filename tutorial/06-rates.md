# 6. More unknowns than conditions

Problem 13 is the well shared by five households:

> 今有五家共井，甲二綆不足如乙一綆，乙三綆不足如丙一綆，丙四綆不足如丁一綆，丁五綆不足如戊一綆，戊六綆不足如甲一綆，各得所不足一綆皆逮，問井深綆長各幾何

Five households share a well. Two of A's ropes fall short of the water by
one of B's ropes; three of B's fall short by one of C's; four of C's by one
of D's; five of D's by one of E's; six of E's by one of A's. Each, given the
one rope it lacks, just reaches. How deep is the well, and how long is each
rope?

There are six unknowns, the depth and five ropes, and five conditions. The
procedure says only 如方程，以正負術入之. Liu Hui says how the board was set:
名各一逮井, each column comes to one well-depth. So the depth is the 實 of
every column, and is 1:

```jz
No. 8.13
Things 甲 乙 丙 丁 戊 實
Set right 2 1 0 0 0 1     "甲二綆不足如乙一綆"
Set 2nd   0 3 1 0 0 1     "乙三綆不足如丙一綆"
Set 3rd   0 0 4 1 0 1     "丙四綆不足如丁一綆"
Set 4th   0 0 0 5 1 1     "丁五綆不足如戊一綆"
Set left  1 0 0 0 6 1     "戊六綆不足如甲一綆"
Solve.
Rates 721 265 191 148 129 76
Gives left 0 0 0 0 721 76     "其後法得七百二十一，實七十六"
```

```
No. 8.13
Things 甲 乙 丙 丁 戊 實
Set right 2 1 0 0 0 1     "甲二綆不足如乙一綆"
Set 2nd   0 3 1 0 0 1     "乙三綆不足如丙一綆"
Set 3rd   0 0 4 1 0 1     "丙四綆不足如丁一綆"
Set 4th   0 0 0 5 1 1     "丁五綆不足如戊一綆"
Set left  1 0 0 0 6 1     "戊六綆不足如甲一綆"
        left  4th  3rd  2nd  right
    甲     1    ·    ·    ·      2
    乙     ·    ·    ·    3      1
    丙     ·    ·    4    1      ·
    丁     ·    5    1    ·      ·
    戊     6    1    ·    ·      ·
    實     1    1    1    1      1
Solve.                                                        法 721; 實 265 191 148 129 76; 13 steps
    take right from left
        left  4th  3rd  2nd  right
    甲     ·    ·    ·    ·      2
    乙    -1    ·    ·    3      1
    丙     ·    ·    4    1      ·
    丁     ·    5    1    ·      ·
    戊    12    1    ·    ·      ·
    實     1    1    1    1      1
    add 2nd to left
        left  4th  3rd  2nd  right
    甲     ·    ·    ·    ·      2
    乙     ·    ·    ·    3      1
    丙     1    ·    4    1      ·
    丁     ·    5    1    ·      ·
    戊    36    1    ·    ·      ·
    實     4    1    1    1      1
    take 3rd from left
        left  4th  3rd  2nd  right
    甲     ·    ·    ·    ·      2
    乙     ·    ·    ·    3      1
    丙     ·    ·    4    1      ·
    丁    -1    5    1    ·      ·
    戊   144    1    ·    ·      ·
    實    15    1    1    1      1
    add 4th to left
        left  4th  3rd  2nd  right
    甲     ·    ·    ·    ·      2
    乙     ·    ·    ·    3      1
    丙     ·    ·    4    1      ·
    丁     ·    5    1    ·      ·
    戊   721    1    ·    ·      ·
    實    76    1    1    1      1
Rates 721 265 191 148 129 76                                  ✓  the 法 721 and the 實s as the board leaves them
Gives left 0 0 0 0 721 76     "其後法得七百二十一，實七十六"  ✓

2 figures checked, 2 right, 0 wrong.
```

The text's answer is 7 丈 2 尺 1 寸 for the well, and 2 丈 6 尺 5 寸 for A's
rope, down to 7 尺 6 寸 for E's: in 寸, 721, 265, 191, 148, 129 and 76. These
are the 法 and the 實s. Each rope is so many 721ths of the depth, and the text
has written the 721 as the depth. Liu Hui says so:

> 此率初如方程爲之，名各一逮井，其後法得七百二十一，實七十六，是爲七百二十一綆而七十六逮井，而戊一綆逮之數定，逮七百二十一分之七十六，是故七百二十一爲井深，七十六爲戊綆之長，舉率以言之

These rates were first found by 方程, each column named as one well-depth.
Then the 法 came to 721 and the 實 to 76: 721 of E's ropes make 76
well-depths, so one of E's ropes is 76/721 of the depth. So 721 is taken as
the depth and 76 as E's rope: the answer is given as rates (舉率以言之).

`Rates` checks the answer as rates (率): the 法 and the 實s, divided by what
they share, against the figures given. Here they share nothing, and the board
leaves them as the text writes them. The `Gives` line is Liu Hui's column:
the left, when only 戊 and the 實 are left in it.

## Dai Zhen's two boards

Dai Zhen says the question cannot fix the length in 丈 and 尺:

> 由是言之，問旣不定，以丈尺依術推求，先得七百二十一，無以定百爲丈十爲尺也

So the question is not determined: working in 丈 and 尺 by the procedure,
721 comes first, and there is no way to fix the hundreds as 丈 and the tens
as 尺.

He gives the same answer as rates, and half and double the text's figures
as answers just as good. He calls Liu Hui's sentence corrupt, and works the
board again, with 721 as every column's 實:

```jz --plain
No. 8.13 (Dai Zhen)
Things 甲 乙 丙 丁 戊 實
Set right 2 1 0 0 0 721     "以此率列各行下，爲各行之下實"
Set 2nd   0 3 1 0 0 721
Set 3rd   0 0 4 1 0 721
Set 4th   0 0 0 5 1 721
Set left  1 0 0 0 6 721
Solve.                      "重求之"
Gives left 0 0 0 0 721 54796    "法得七百二十一，實得五萬四千七百九十六"
```

```
No. 8.13 (Dai Zhen)
Things 甲 乙 丙 丁 戊 實
Set right 2 1 0 0 0 721     "以此率列各行下，爲各行之下實"
Set 2nd   0 3 1 0 0 721
Set 3rd   0 0 4 1 0 721
Set 4th   0 0 0 5 1 721
Set left  1 0 0 0 6 721
Solve.                      "重求之"                                      法 721; 實 191065 137711 106708 93009 54796; 13 steps
Gives left 0 0 0 0 721 54796    "法得七百二十一，實得五萬四千七百九十六"  ✓

1 figure checked, 1 right, 0 wrong.
```

54796 divided by 721 is 76, E's rope. Both commentators' figures are what
the board gives.
