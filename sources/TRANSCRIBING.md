# Transcribing a problem

You are transcribing problems of chapter 8 (方程) of the *Nine Chapters on
the Mathematical Art* into `.jz` pages, the notation in
[`../docs/notation.md`](../docs/notation.md). Read that first.

## The witness

The witness is the **scan**: the Sibu congkan (四部叢刊) photographic
reprint of the Weiboxie (微波榭) edition, with Liu Hui's commentary (in
small double-column characters after the main text), Li Chunfeng's notes
(臣淳風等謹按) and, from page 36, Dai Zhen's collation notes (訂訛).

- Page images: `sources/sibu-congkan/pages/pNN.jpg` (djvu page NN of
  `Sibu Congkan0392-劉徽-九章算術-3-3.djvu`, Wikimedia Commons). Read them.
- A text of the same pages, with page markers `【p.NN】`:
  `sources/sibu-congkan/卷第八.paged.txt`. It is Wikisource's
  transcription, marked **unproofread**. Use it to find your place and to
  save typing; **every figure must be read from the image**. Where it and
  the image differ, follow the image and say so in your notes.
- Another, punctuated, text of the main text only (no commentary), from a
  different Wikisource page: `sources/wikisource/jiuzhang.wiki`, lines
  1380-1560. A third witness for the main text; it is not the same edition.

Text is read in columns, top to bottom, right to left. Each page has a
folio header; small double-column characters are commentary.

## What to write

One file per problem, `corpus/no-NN.jz` (two digits). In it, in order:

1. `No. 8.N` and a `%` note giving the scan pages (`% Sibu congkan pp. 6-7`).
2. The problem statement (今有…問…), as a quoted line, in the edition's
   characters, with no punctuation added beyond `，` between clauses if you
   wish. Then an English translation of it, as a `%` note. Your own
   translation: do not copy a published one.
3. The answer (荅曰…), quoted, then translated.
4. The procedure (術曰…), quoted.
5. `Things …` and the `Set` lines: the board the procedure sets. Where the
   procedure says how to set it (置上禾五秉正，下禾七秉負…), set it so, and
   quote those words at the end of the `Set` line. Where it says only 如方程
   or 以正負術入之, set the columns from the statement, the first condition
   as the right-hand column, as Problem 1 does, and say in a note how you
   turned each condition into a column. Choose one unit for the page and say
   which.
6. `Solve.`
7. `Answer …`: the answer exactly as the text writes it: whole number, and
   the fraction with **the denominator the text uses**. 五十二分斗之一十八 is
   `18/52`; never reduce it. A figure written as 半 is `1/2`.
8. Any figure the commentary (Liu Hui, Li Chunfeng, or Dai Zhen in the
   collation notes) states for this problem's board: write the operations it
   describes, with the commentary's words quoted, and its figures as `Gives`
   (a column) or `Rates` lines. Only where the commentary gives figures; do
   not invent working the text does not describe.

## Rules

- **Never correct the text.** If the text's figure looks wrong, transcribe
  it as it is, and let `jiuzhang` find it. Say in your notes what you saw.
- Every figure from the image. If a character is unclear, say so in a `%`
  note on that line, and in your notes.
- Run `cargo run -q -- corpus/no-NN.jz` and `cargo run -q -- --check
  corpus/no-NN.jz` on each file. If a figure the text states disagrees with
  the board, first re-read the image. If the image says what you wrote, leave
  it, and record it in your notes. Do not change a figure to make it agree.
- Do not edit anything outside the files you were given.
- Write notes for your range in `sources/notes/problems-AA-BB.md`: for each
  problem, the pages, every place the image and the Wikisource text differ,
  every unclear character, how you set the board where the text does not say,
  and anything `jiuzhang` flagged and what the image says there.
