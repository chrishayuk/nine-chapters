# 1. The Nine Chapters, and a first page

The *Nine Chapters on the Mathematical Art* (九章算術, *Jiuzhang suanshu*) is
a Chinese book of problems and procedures, put together under the Han
dynasty from older material. It has 246 problems in nine chapters: fields,
grain, shares, volumes, taxes, and more. Each problem gives its question,
its answer, and a procedure (術) for finding it.

Liu Hui wrote a commentary on it in 263 CE, explaining and justifying the
procedures. Li Chunfeng and his colleagues added notes in the 7th century.
In the 1770s Dai Zhen collated the text again, and added notes of his own
(訂訛, "correcting errors") where he thought the text or the commentary had
gone wrong.

Chapter 8 is 方程 (*fangcheng*), usually translated "rectangular arrays".
Its eighteen problems each give several conditions on several unknown
quantities: so many bundles of three grades of grain yield so much, so many
oxen and sheep cost so much. The procedure sets the conditions out on a
counting board, one column to each, and works the columns against each
other until one thing is left alone with its value.

## The counting board

The board is a surface on which numbers are laid out in counting rods. A
column (行) is set for each condition, and the first is set on the right:
the text says 置……於右方, "set … on the right". Down each column there is a
position (位) for each kind of thing, and at the foot the 實 (*shi*), what
the column comes to. Positive numbers are 正, negative 負; Liu Hui says
positive rods are red and negative ones black.

`jiuzhang` keeps the board as the text does: columns from the right, figures
top to bottom, the 實 last.

```jz
Things 上禾 中禾 下禾 實
Set right   3 2 1 39
Set middle  2 3 1 34
Set left    1 2 3 26
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
```

This is the board of Problem 1: three grades of grain (上禾, 中禾, 下禾:
top, middle and low), and three harvests. The right-hand column says 3
bundles of top grain, 2 of middle and 1 of low yield 39 斗 (*dou*).

## What jiuzhang does

A page (a `.jz` file) is a transcription of a procedure: one operation to a
line, in the order the text gives them. `jiuzhang` does each operation on the
board, and where the text or a commentator states a figure (an answer, a
column, a count), it compares the figure with the board and marks it ✓ or ✗.

```jz --plain
No. 8.1
Things 上禾 中禾 下禾 實
Set right   3 2 1 39
Set middle  2 3 1 34
Set left    1 2 3 26
Solve.
Answer 9 1/4, 4 1/4, 2 3/4
```

```
No. 8.1
Things 上禾 中禾 下禾 實
Set right   3 2 1 39
Set middle  2 3 1 34
Set left    1 2 3 26
Solve.                      法 36; 實 333 153 99; 9 steps
Answer 9 1/4, 4 1/4, 2 3/4  ✓  written over 4: the 法 and 實s divided by what they share (約), 4; or each in its lowest terms (not as the board leaves it, 9 9/36)

1 figure checked, 1 right, 0 wrong.
```

`Solve` works the board by the text's procedure, and `Answer` is the text's
answer: 9 1/4 斗 a bundle of top grain, 4 1/4 of middle, 2 3/4 of low. It is
right. `jiuzhang no 1` prints the page of Problem 1 as transcribed, with the
board after every line. The whole chapter is in `corpus/`, read from the
scans of the Sibu congkan edition: the eighteen problems, and the passages of
Liu Hui and Dai Zhen that state figures.

## What it does not claim

It does not translate the chapter, or say what the procedures meant to the
people who used them. Where the text does not say how something is done, the
page says what was chosen, in a note, and `jiuzhang decisions` lists the
choices the program itself makes. A ✓ says that one reading of the text gives
the figure the text states. It does not say that this reading is the one the
author meant.

The mathematics is well known, and nothing here is claimed as new. What the
program adds is that every figure is checked by doing the work again, step
by step, on a board.
