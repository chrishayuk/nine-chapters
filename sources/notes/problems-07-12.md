# Problems 8.7 to 8.12: transcription notes

Witness: Sibu congkan scan, pages read from `sources/sibu-congkan/pages/`.
Every figure below was read from the images; the Wikisource text
(`卷第八.paged.txt`) was used only to find the place.

## One thing that runs through the range: 直

Throughout these pages the Wikisource text leaves out the character 直
("worth"; also in 直除). The scan has it every time. Places in my range:

| page | scan | Wikisource |
|---|---|---|
| p15 (8.7 statement) | 牛五羊二**直**金十兩牛二羊五**直**金八兩問牛羊各**直**金幾何 | 牛五羊二金十兩牛二羊五金八兩問牛羊各金幾何 |
| p15 (8.7 answer) | 牛一**直**金一兩…羊一**直**金二十一分兩之二十 | 牛一金一兩…羊一金二十一分兩之二十 |
| p15 (8.7 Liu Hui) | 更置牛十羊四**直**金二十兩左行牛十羊二十五**直**金四十兩 | 更置牛十羊四金二十兩左行牛十羊二十五金四十兩 |
| p17 (8.8 Liu Hui) | 故下無錢**直**也 | 故下無錢也 |
| p17 (8.8 Liu Hui) | 而以右行**直**除之 | 而以右行除之 |
| p20 (8.11 Liu Hui) | 一牛價**直**一萬也…亦**直**一萬也…三馬二牛**直**錢二萬…二馬五牛**直**錢二萬也 | 一牛價一萬也…亦一萬也…三馬二牛錢二萬…二馬五牛錢二萬也 |
| p41 (Dai Zhen, 8.8) | 故下無錢**直**也…而以右行**直**除之 | 故下無錢也…而以右行除之 |
| p41 (Dai Zhen, 8.8) | 而以**直**除 (three times) | 而以除 |

No figure depends on it. The other punctuated text (`sources/wikisource/jiuzhang.wiki`)
has 直 in 8.7.

## 8.7 (牛羊直金), p. 15

- Figures: statement 5 2 10 / 2 5 8; answer 牛 1 13/21, 羊 20/21 (一兩二十一分兩之十三;
  二十一分兩之二十). Scan and Wikisource agree on all figures.
- Differences: 直 (see above).
- Board: the text says only 如方程. The first condition is the right column, the second the left. Unit 兩.
- Liu Hui's 齊同 working is on the page after the answer. The board is set again from the statement for it.
  He says 左右行相乘. The right column is multiplied by the head of the left (2). The left is
  multiplied by 5, written as a number because the right's head is 10 by then. Gives 10 4 20 ✓,
  10 25 40 ✓. Then 以少行減多行 (take right from left) gives 0 21 20 ✓ (牛數盡, 羊差二十一, 金多二十兩).
- jiuzhang: all ✓. Answer note: written over 21, the 法 as the board leaves it.
- jiuzhang.wiki has 一十三 where the scan has 十三. This does not change the figure.

## 8.8 (賣牛羊買豕), pp. 16-17

- Figures: right 2 5 -13 1000; middle 3 -9 3 0 (錢適足, so the 實 is empty); left -5 6 8 -600, as the
  procedure sets them. The answer is 1200, 500, 300. Scan and Wikisource agree on all figures.
- Differences: 直 twice in Liu Hui's note (see above).
- Liu Hui's note supposes a working (設欲…) but states no figure, so it is not transcribed as operations.
- jiuzhang: Answer ✓.
- jiuzhang.wiki has 豕一十三 where the scan has 豕十三, and 次牛三正 / 次牛五負 where the scan has
  次置牛三正 / 次置牛五負.

### Dai Zhen's working (`corpus/dai-zhen-08.jz`), pp. 41-42

- The figures he states are 羊三十三正 豕四十五負 錢三千正; 羊三十七正 豕四十九負 錢三千八百正;
  豕四十八正 (法) 錢一萬四千四百正 (實); and 豕三百, 羊五百, 牛一千二百.
- Direction, from his signs:
  - First step: 羊 +33 is 3 x right − 2 x middle, so `Take middle from right crosswise` ✓.
  - Second step: `Take right from left crosswise` gives 2 x left + 5 x right = 0 37 -49 3800 ✓,
    because the heads are of opposite sign.
  - Third step: 豕 +48 is 33 x (second result) − 37 x (first result), so the first result is taken
    from the second ✓.
- **The notation cannot express this directly.** Dai Zhen uses the right column as first set twice
  (右行牛二 in both the first and second cross-multiplications), and he lays out the two results as a new
  pair (重列爲左右兩行). `Take … crosswise` writes its result over the column it takes from, so after
  the first step the original right is gone. Two `Set` lines that are not in the text were added, with a
  `%` note. They copy the first result into the middle and set the right again as it first stood. The
  third step is then `Take middle from left crosswise`, where the middle is his new right.
- jiuzhang: 4 figures, all ✓.

## 8.9 (五雀六燕), pp. 17-18

- Figures: answer 雀 1 13/19 (一兩十九分兩之十三), 燕 1 5/19 (一兩十九分兩之五); each pan 八兩;
  一斤. Scan and Wikisource agree.
- Board: the procedure says only 交易質之各重八兩. After the exchange the sparrows' pan is 四雀一燕
  (right column, 4 1 8) and the swallows' pan is 一雀五燕 (left, 1 5 8). Liu Hui reads the board this
  way, and he says the left head is 1 (左行頭位其數是一). Unit 兩, 1 斤 = 16 兩.
- jiuzhang: Answer ✓, written over 19.
- jiuzhang.wiki has 一十九分 where the scan has 十九分.
- Liu Hui (`corpus/liu-hui-09.jz`):
  - First way: `Multiply left by the head of right` (令右行徧乘左行; the right is not multiplied, since
    the left head is 1), then `Take right from left until the head is gone` (左頭位減盡). He states no
    figure for the 法 and 實 (中法下實). Solve and Answer ✓.
  - Second way (按…): the board is set again, then `Take left from right`. This gives 3 -4 0 ✓
    (三雀四燕重相當).
  - **Not expressible:** 雀率重四燕率重三 (sparrow:swallow weights 4:3). `Rates` checks the 法 and 實s of a
    board solved through a 法, not the two coefficients of a column read crosswise. It is kept as a quoted
    line.
- Dai Zhen (`corpus/dai-zhen-09.jz`, p. 43):
  - 以下實俱重八兩徧乘上雀燕: each column is multiplied by the other's 實 (8). The left is written
    `Multiply left by 8`, because the right's 實 is 64 by then.
  - He states 右雀三十二, 左雀八, 左燕四十, 右燕八, but not the 實s (64), so there is no Gives for the
    multiplied columns.
  - 以左雀八減右雀三十二餘二十四以右燕八減左燕四十餘三十二 is written as `Take left from right`. This gives
    24 -32 0 ✓. The 0 is mine: he says only 二物 (two things left).
    - Strictly, he takes the left 雀 from the right 雀 and the right 燕 from the left 燕. This is the same
      as a single take that leaves 燕 negative (正負相借).
  - 因而約之 (Reduce) gives 3 -4 0 ✓.
  - 對易其數卽雀率重四而燕率重三 is a quoted line. It cannot be checked, as above.

## 8.10 (甲乙持錢), pp. 18-19

- Figures: answer 甲 37 1/2 (三十七錢半), 乙 25. Liu Hui's columns: 二甲一乙而錢一百; 二甲三乙而錢一百五十.
- **Difference:** the procedure is 術曰如方程**損益**之 in the scan. Wikisource has 損**妓**之, which is a
  misreading; 益 is clear on the image.
- Board: the statement's conditions have fractions (乙半, 甲太半), so the columns are set as Liu Hui gives
  them after 各以分母乘其全内子: right 2 1 100, left 2 3 150. Unit 錢.
- jiuzhang: Answer ✓. Its note: "written over 2: the 法 and 實s divided by what they share (約), 2; or
  each in its lowest terms (not as the board leaves it, 37 2/4)". The board's 法 is 4.

## 8.11 (馬牛價), pp. 19-20

- Figures: answer 馬 5454 6/11 (五千四百五十四錢十一分錢之六), 牛 1818 2/11
  (一千八百一十八錢十一分錢之二). Liu Hui's columns: 三馬二牛直錢二萬; 二馬五牛直錢二萬. Scan and
  Wikisource agree on all figures.
- Differences: 直 four times in Liu Hui (see above).
- Board: the procedure says only 損益之. The columns are set as Liu Hui gives them (通分内子): right 3 2
  20000, left 2 5 20000, with 馬 above 牛 as in his columns. Unit 錢.
- jiuzhang: Answer ✓, written over 11.
- jiuzhang.wiki has 一十一分 where the scan has 十一分.

## 8.12 (武中下馬), pp. 21-22

- Figures: 一匹, 二匹, 三匹, 四十石; answer 武 22 6/7, 中 17 1/7, 下 5 5/7 (七分石之六 / 之一 / 之五).
  The answer for 武馬 breaks over a column (…七分石之 | 六). Scan and Wikisource agree.
- Board: 各置所借. Each team with the horse it borrows is a column, in statement order:
  - right 1 1 0 40 (武 + 中)
  - middle 0 2 1 40 (中 x 2 + 下)
  - left 1 0 3 40 (下 x 3 + 武)

  Unit 石.
- No commentary with figures on these pages.
- jiuzhang: Answer ✓, written over 7.

## Unclear characters

None that affect a figure. On p. 16, 豕 is printed in a form close to 家, but the sense and the strokes
read 豕.
