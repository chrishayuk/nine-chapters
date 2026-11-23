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
every     each multiplication, taking or division of one figure, by one or not
repeated  as worked, but a taking several times over is that many takings, one at a time
columns   each operation on a whole column
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
Count 145                                    "兼乘減除言之則一百四十五算"  ✓  plain: plain 145, worked 140, every 175, repeated 140, columns 15
Set right 9 7 3 2 5 140
Set 2nd   7 6 4 5 3 128
Set 3rd   3 5 7 6 4 116
Set 4th   2 5 3 9 4 112
Set left  1 3 2 8 5  95
Solve by one.                                                7, 4, 3, 5, 6; 15 steps
Answer 7, 4, 3, 5, 6                                         ✓
Count 99                                     "凡用九十九算"  no rule counts 99: plain 101, worked 98, every 81, repeated 295, columns 15

3 figures checked, 3 right, 0 wrong.
```

One rule, `plain`, gives 145 for the plain method exactly: every
multiplication, taking and division of one figure, leaving out only the
heads that cross-multiplying makes equal, as Dai Zhen says they may be. `worked` leaves out multiplications by one as
well, and the left column's head is 1, so it gives 140.

No rule gives 99 for the shorter way. `worked` gives 98, `plain` 101. Three
operations in it are by one: in finding the 麥, a multiplication by -1 and a
division by -1; in finding the 麻, a division by 1. `plain` counts all three,
`worked` none of them. Dai Zhen's 99 is between. A rule that counted just one
of the three would give 99, but it would be written to fit the figure, and
there is no reason in the text to count one and not the others. So the count
is reported as it comes out, 98 under `worked` and 101 under `plain`; the
line has no ✓, and is not among the figures checked.
His arrangement is not given in full either: he does not say, for one thing,
whether the 黍 is found before the 荅.

## The cost of the literal reading

`jiuzhang counts` counts Problem 18 by each method under each rule, and
says how big the board has to be. It takes a little while: it also counts
Liu Hui's figures (chapter 9).

```console
$ jiuzhang counts
dai-zhen-18  8.18 (Dai Zhen, 訂訛: 145 and 99)
  Count 145  plain     145   ✓
  Count 145  worked    140
  Count 145  every     175
  Count 145  repeated  140
  Count 145  columns   15
  Count 99   plain     101
  Count 99   worked    98
  Count 99   every     81
  Count 99   repeated  295
  Count 99   columns   15

Problem 18 by each method, counted under each rule, and the board it needs:
               plain   worked    every repeated  columns    rods  places
  text           158      158      133   180403       25     160      11
  reducing       188      179      163      504       33     138       4
  cross          145      140      175      140       15     149      11
  one            101       98       81      295       15     122       4
  rods: the most rods on the board at once; places: the most digits in any figure.

liu-hui-18-old: Liu Hui says 77 算. Of 100000000 readings of his working, 229906 can be done to the end and 0 agree with the figures he states, so there is nothing to count.
liu-hui-18-new: Liu Hui says 124 算. Of 6115295232 readings of his working, 0 can be done to the end and 0 agree with the figures he states, so there is nothing to count.
```

The text's own procedure, multiplying the whole column and then taking
until the head is gone, needs figures of 11 places. Counted with 直除 as
taking one column away again and again, a taking at a time (the `repeated`
rule), it comes to 180403. With each worked column reduced (約) as it goes
(`reducing`), the largest figure has 4 places, and `repeated` counts 504.
The text does not say to reduce. Done literally, its words are a great deal
of work.
