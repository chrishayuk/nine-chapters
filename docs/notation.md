# The notation

A `.jz` file is a counting board and what is done to it, one operation to a
line, in the order the text gives them. It is a transcription of a procedure,
not a program: where the text sets a column, the file sets it; where the text
says "take the right-hand column from the middle until its head is gone", the
file says that. `jiuzhang` does each operation on the board and compares every
figure the source states with what the board holds.

```
No. 8.1
"今有上禾三秉，中禾二秉，下禾一秉，實三十九斗；……"
Things 上禾 中禾 下禾 實
Set right   3 2 1 39          "置上禾三秉，中禾二秉，下禾一秉，實三十九斗，於右方"
Set middle  2 3 1 34          "中、左禾列如右方"
Set left    1 2 3 26
Multiply middle by the head of right.            "以右行上禾徧乘中行"
Take right from middle until the head is gone.   "而以直除"
Solve.
Answer 9 1/4, 4 1/4, 2 3/4
```

## The board

Columns (行) are set from the right, as the text sets them: `right` is the
first column, 右行. A board of three has `right`, `middle` and `left`. On a
larger board the columns are counted from the right: `right`, `2nd`, `3rd`,
`4th`, … and the last is also `left`. The text's own names are accepted too:
`右行`, `中行`, `左行`, `第二行` … `第九行`.

Each column is a list of positions (位), top to bottom: one for each kind of
thing, then the 實 (shi), what the column totals. `Things` names them, last
the 實:

```
Things 麻 麥 菽 荅 黍 實
```

A figure is a whole number. Positive is 正 (red rods), negative 負 (black
rods); `0` or `·` is an empty position. The 正負術 (the rule of signs) is
what whole-number arithmetic does, and `jiuzhang` uses it without being told.

## Operations

| line | the text | what is done |
|---|---|---|
| `Set right 3 2 1 39` | 置 | set a column, every position top to bottom |
| `Multiply middle by 3` | 徧乘 | multiply the whole of a column |
| `Multiply middle by the head of right` | 以右行上禾徧乘中行 | by the first figure of another column that is not empty |
| `Multiply middle by the 中禾 of right` | | by a named position of another column |
| `Take right from middle` | 減 | take one column from another, once |
| `Take right from middle until the head is gone` | 直除, 去其頭位 | as many times as empties the head of the column taken from |
| `Take right from middle until 中禾 is gone` | | as many times as empties that position |
| `Take right from middle as far as it goes` | 不足減乃止 | as many times as goes without passing zero at the head |
| `Take right from middle as far as the 菽 goes` | | the same, at a named position |
| `Take right from middle crosswise` | 互乘 | each multiplied by the other's head, then taken |
| `Take right from middle in turn` | 相減 | each taken from the other in turn, until the head is gone |
| `Take right from middle 3 times` | | three times over |
| `Add right to middle` | 益, 并 | add one column to another (`Add right to middle 2 times`) |
| `Halve middle` | 半 | halve a column (every figure must be even) |
| `Reduce middle` | 約 | divide a column by what all its figures have in common |
| `Divide middle by 3` | | divide a column (every figure must go) |
| `Drop 3rd` | 廢去 | take a column off the board |
| `Nothing` | | do nothing: the reading that takes a clause as corrupt or idle |
| `Solve` | 方程術 | the rest of the procedure, done by `jiuzhang` |
| `Solve by reducing` / `by cross` / `by one` | | the same by another reading (`jiuzhang methods`) |

"Head" (頭位) is the first position of the column being worked that is not
empty. Where the text says "until the head is gone" and the figures do not
divide, the operation cannot be done, and `jiuzhang` says so and stops.

## What the source states

These lines are what the text or a commentator says the board gives. They
are checked, and marked ✓ or ✗.

| line | checked against |
|---|---|
| `Gives middle 0 5 1 24` | the column as the board now holds it |
| `Answer 9 1/4, 4 1/4, 2 3/4` | what `Solve` found, thing by thing, top to bottom; and how each fraction is written |
| `Rates 721 265 191 148 129 76` | the 法 and the 實s, divided by what they share: for an answer given as rates (率) |
| `Rods 77` | the rods the board now holds: a digit to five is that many rods, six to nine one for five and the rest |
| `Count 77` | the number of 算 the operations since the last `Count` take, under each rule in `jiuzhang rules` |

An answer is written as the text writes it: a whole number, then the
fraction, with the denominator the text uses. `1 18/52` is 一斗五十二分斗之
一十八, and is not to be reduced to `1 9/26`: how the text writes it is part
of what is checked. A half (半) is `1/2`, 少半 `1/3`, 太半 `2/3`. Units are
the transcriber's choice, said in a note (`% in 升: 1 斗 is 10 升`).

## Readings in the alternative

Where a clause can be read more than one way, the line gives each reading,
separated by `|`:

```
Take 3rd from 4th | Take 4th from 3rd | Take 3rd from 4th until the head is gone.
```

The page runs with the first. `jiuzhang readings FILE` runs every
combination, and lists those that can be done to the end and agree with every
figure the source states.

## The rest of a line

- `"…"` on a line of its own is the source's words, kept and not computed.
- `"…"` at the end of an operation is the words of the source it transcribes.
- `% …` is a note, to the end of the line.
- `No. 8.1` is a heading.
- A full stop at the end of an operation is allowed and means nothing.
