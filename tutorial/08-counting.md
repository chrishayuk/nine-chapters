# 8. Counting 算

Problem 18 has five things and five columns: hemp, wheat, soybeans, small
beans and millet (麻, 麥, 菽, 荅, 黍), bought five times for so many cash.
Liu Hui and Dai Zhen each say how much work a way of solving it takes, in
算: Liu Hui 77 for the old method made short and 124 for his new method;
Dai Zhen 145 for the plain method and 99 for a shorter one.

Nobody says what one 算 is. A rod? A step? One figure multiplied? `jiuzhang`
records what every operation does to every position, and counts it under
several rules:

```console
$ jiuzhang rules
plain     each multiplication, taking or division of one figure, except a head known to cancel; a taking several times over is a multiplication and a taking
worked    each multiplication, taking or division of one figure that has to be worked (not by one, not a head known to cancel); a taking several times over is a multiplication and a taking
readoff   as plain, but a division by one is read off, not worked (proposed by the review, after the counts were known)
every     each multiplication, taking or division of one figure, by one or not
repeated  as worked, but a taking several times over is that many takings, one at a time
columns   each operation on a whole column
guo       Guo Shuchun's (2020, n. 40): each multiplication or division of a whole column, each taking of one column from another, each step of finding a thing; placing rods not counted here
```

`Count` states a count from the source. It counts the operations since the
last `Count` (setting the board is not counted), under every rule, and says
which rules give the figure.

## Dai Zhen's 145 and 99

> 先化五爲四次化四爲三次化三爲二次化二爲一凡用十算兼乘減除言之則一百四十五算

First five become four, then four three, three two, two one: ten
[eliminations] in all; counting multiplications, takings and divisions
together, 145 算.

His plain method is cross-multiplication, as in Problem 8 (chapter 7). He
says the heads, once cross-multiplied, are always equal, and need not be
worked (凡上一位互乘其數必同可省乘). His shorter way works from a column whose
head is one, and takes equal heads directly, without multiplying.

```jz --plain
No. 8.18 (Dai Zhen)
Things 麻 麥 菽 荅 黍 實
Set right 9 7 3 2 5 140
Set 2nd   7 6 4 5 3 128
Set 3rd   3 5 7 6 4 116
Set 4th   2 5 3 9 4 112
Set left  1 3 2 8 5  95
Solve by cross.
Answer 7, 4, 3, 5, 6
Count 145                                    "兼乘減除言之則一百四十五算"
Set right 9 7 3 2 5 140
Set 2nd   7 6 4 5 3 128
Set 3rd   3 5 7 6 4 116
Set 4th   2 5 3 9 4 112
Set left  1 3 2 8 5  95
Solve by one.
Answer 7, 4, 3, 5, 6
Count 99                                     "凡用九十九算"
```

```
No. 8.18 (Dai Zhen)
Things 麻 麥 菽 荅 黍 實
Set right 9 7 3 2 5 140
Set 2nd   7 6 4 5 3 128
Set 3rd   3 5 7 6 4 116
Set 4th   2 5 3 9 4 112
Set left  1 3 2 8 5  95
Solve by cross.                                              7, 4, 3, 5, 6; 15 steps
Answer 7, 4, 3, 5, 6                                         ✓
Count 145                                    "兼乘減除言之則一百四十五算"  ✓  plain, readoff: plain 145, worked 140, readoff 145, every 175, repeated 140, columns 15, guo 54
Set right 9 7 3 2 5 140
Set 2nd   7 6 4 5 3 128
Set 3rd   3 5 7 6 4 116
Set 4th   2 5 3 9 4 112
Set left  1 3 2 8 5  95
Solve by one.                                                7, 4, 3, 5, 6; 15 steps
Answer 7, 4, 3, 5, 6                                         ✓
Count 99                                     "凡用九十九算"  ✓  readoff: plain 101, worked 98, readoff 99, every 81, repeated 295, columns 15, guo 88

4 figures checked, 4 right, 0 wrong.
```

One rule, `plain`, gives 145 for the plain method exactly: every
multiplication, taking and division of one figure, leaving out only the
heads that cross-multiplying makes equal, as Dai Zhen says they may be.
(He says the heads need not be *multiplied*; leaving out their takings too is
this program's convention.) `worked` leaves out multiplications by one as
well, and the left column's head is 1, so it gives 140.

For the shorter way, `plain` gives 101 and `worked` 98. Three operations in it
are by one: in finding the 麥, a multiplication by -1 and a division by -1; in
finding the 麻, a division by 1. `plain` counts all three, `worked` none.
Dai Zhen's economies are about multiplying; he says nothing about dividing by
one. `readoff` is `plain` with a division by one read off rather than worked:
it gives 145 and 99, both of his figures. It was proposed by the independent
review after the figures were known, so it is a rule fitted after the fact,
and a candidate, not Dai Zhen's rule. Which convention he used, the text does
not say.

His arrangement is not given in full either. He says 依術得荅價, "by the
procedure find the price of 荅", where `Solve by one` finds the 黍 first; the
review found that finding them the other way round does not change the
counts.

## The cost of the literal reading

`jiuzhang counts` counts Problem 18 by each method under each rule, and
says how big the board has to be. It takes a little while: it also counts
Liu Hui's figures (chapter 9).

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

On the board as it stands after each step, the text's own procedure,
multiplying the whole column and then taking until the head is gone, holds
figures of 11 places. The working of the last step, which is not on the
board, goes further: finding the 荅 forms a product of 16 places. Counted
under the `repeated` rule, where a taking done several times over is that
many takings of single figures, it comes to 180403 operations (180330 of them
takings), or 89773 takings of a whole column. With each worked column reduced (約) as it goes
(`reducing`), the largest figure has 4 places, and `repeated` counts 504.
The text does not say to reduce. Done literally, its words are a great deal
of work.
