<!-- Source extraction by a second model (OpenAI Codex, with web search), 24 November 2026, for corpus/guo-18-old.jz and corpus/hart-18-old.jz. It marks what it read (S), what it reconstructed (R) and what it could not read (U). The pages assert only what the sources state. -->

# Liu Hui’s 77 and 124: source extraction and arithmetic checks

Read on 2026-11-24. **The principal recoverable result is Guo 2020’s operation-count reconstruction of 77. It must be distinguished from reconstructions counting physical rods.**

## What I could actually read

* **[Guo 2020, Scribd transcription](https://www.scribd.com/document/1021862496/):** the classical passage, notes 21–82, and captions/references to figures 8-7 and 8-8. **The figures’ contents are absent from the accessible transcription.** Its relevant text is around transcription lines 13653–13687 and 13773–13907. The scan’s printed pagination was not recoverable here.
* **[Chemla & Guo 2004, Scribd transcription](https://fr.scribd.com/document/398756962/LesNeufChapitres-Karine-Chemla-Guo-Shuchun):** the Chinese passage and apparatus on pp. 652–655, including notes 93–107, and French notes 156–157 and 168, pp. 876–877. Relevant transcription lines: 25101–25122, 25168–25199, 37127–37192. Chinese OCR is substantially worse than Guo 2020’s.
* **[Wei Xuegang 魏雪刚 & Guo Shirong 郭世荣 2016](https://m.fx361.com/news/2016/1216/16730222.html):** abstract and opening paragraphs only. The page ends by requiring its app for the full text. **No reconstruction or figure from this paper was readable.**
* **[Zhu Yiwen 2025](https://journals.sagepub.com/doi/10.1177/20966083251414789):** the article, especially §3. It discusses 77/124 as rod requirements but gives its worked board example for **8.7, oxen and sheep**, not 8.18. No 77/124 board reconstruction appears there.
* **[Roger Hart 2012](https://www.rhart.org/papers/KJHS-Hart.pdf):** an independent 77-rod reconstruction, pp. 323–329, including numerical formulas and rod diagrams. Also his reproduction/discussion of Shen, Lun & Crossley, pp. 322–323. I did **not** read Li Jimin’s or Shen et al.’s books directly.

Evidence labels below: **S** = source statement; **R** = my arithmetic reconstruction; **U** = unavailable or uncertain. Numerical arrays marked R are not transcriptions of missing figures. Mathematical formulas are my notation for the reported operations, not quotations of modern prose.

## 1. Emended Chinese passage

### Guo 2020: readable transcription, simplified characters retained

The following preserves the readable classical text, including **为**, **麦**, **余**, **实**, **荅**, and final **筭**. I removed superscript note markers and line breaks, and one intrusive OCR fragment between notes 29 and 30 (`次 第 头 次 第 头 次`). I have not checked a page image, so this is a character-level transcription of the accessible OCR, **not a certified facsimile transcription of the printing**. The source has **如此凡用七十七筭**, not merely 凡用七十七算. [Source: Guo 2020](https://www.scribd.com/document/1021862496/).

> 以旧术为之，凡应置五行。今欲要约。先置第三行，减以第四行，又减第五行；次置第二行，以第二行减第一行，又减第四行，去其头位；余，可半；次置右行及第二行，去其头位；次以右行去第四行头位；次以左行去第二行头位；次以第五行去第一行头位；次以第二行去第四行头位；余，可半；以右行去第二行头位；以第二行去第四行头位。余，约之为法、实，实如法而一，得六，即有黍价。以法治第二行，得荅价，右行得菽价，左行得麦价，第三行麻价。如此凡用七十七筭。

### Historical emendations: Chemla & Guo’s apparatus

This apparatus supplies the historical changes; Guo 2020’s explanatory notes 22–40 are **not** numbered textual-emendation notes. `∅` denotes absence. Traditional forms in this table identify readings, not a silent traditionalization of the Guo quotation. **汇校本** is the edition credited by the apparatus; no earlier individual proposer is named where that is all the apparatus says. Attributions below report adoption, not priority beyond what I could read. [Source: Chemla & Guo, pp. 652–655](https://fr.scribd.com/document/398756962/LesNeufChapitres-Karine-Chemla-Guo-Shuchun).

| Marker | [original → emended, note, proposer/adopter] | Scope or uncertainty |
|---|---|---|
| E93 | [以減 → 減以, 93, 汇校本 / Guo] | In `先置第三行，減以第四行`. |
| E94 | [及 → 又; 三? → 五, 94, 汇校本 / Guo] | In `又減第五行`. **U:** apparatus OCR shows original `王`, whereas the received clause in the user’s reference and Hart’s quotation is `及減第三行`. Thus `[及減第三行 → 又減第五行]` is the clause-level identification, but the original numeral is not facsimile-verified. |
| E95 | [三 → 一, 95, 汇校本 / Guo] | `以第二行減第一行`. |
| E96 | [∅ → 又減第四行, 96, 譯注本 adopted here] | Five supplied characters. Apparatus says 汇校本 had `反減第四行`; the present edition follows 譯注本. |
| E97a | [∅ → 餘，可半, 97, 譯注本 adopted here] | First halving, after `去其頭位`. |
| E97b | [∅ → 及第二行, 97, 汇校本; retained here] | `次置右行及第二行`. Apparatus distinguishes this from Dai Dunyuan’s different supplement. |
| E98 | [第四 → 右; 減左行 → 去第四行, 98, 汇校本 / Guo] | Produces `次以右行去第四行頭位`. |
| E99 | [第四行及 → ∅, 99, 汇校本 / Guo] | Delete after `次以左行去`, producing `次以左行去第二行頭位`. |
| E100 | [減 → 去; 二 → 一, 100, 汇校本 / Guo] | `次以第五行去第一行頭位`. |
| E101 | [餘，可半 before the preceding clause → after 次以第二行去第四行頭位, 101, relocation reported here] | Second halving. The note reports misplacement in 大典本 / 楊輝本; it does not explicitly name a separate first proposer for this relocation. |
| E102 | [∅ → 以右行去第二行頭位，以第二行去第四行頭位, 102, 譯注本 adopted here] | Nineteen characters excluding punctuation. The apparatus rejects a different 汇校本 supplement involving 左行. |
| E103 | [法 → 法、實, 103, present editors] | Supply 實 after 法 in `餘，約之為法、實`. |
| E104 | [空 → 六, 104, 戴敦元 Dai Dunyuan] | `得六，即有黍價`. |
| E105 | [減 → 治, 105, 李繼閔 Li Jimin’s restoration adopted] | **Edition-specific:** 減 is attributed to 孔刻本; 治 restores 大典本 / 楊輝本. It is not a claim that every received witness had 減. |
| E106 | [右行得菽價 after 第三行麻價 → immediately after 得荅價, 106, present editors] | Move five characters. Dai Dunyuan also moved them but changed 右 to 左; that change is not adopted. |
| E107 | [右 → 左, 107, 汇校本 restoring 大典本 / 楊輝本 against Dai Dunyuan] | `左行得麥價`; a restoration against one modern emendation. |

Annotated alignment below is an **editorial aid, not another source quotation**. Read each bracket’s right-hand side as the adopted text. It uses Guo 2020’s simplified character forms; the apparatus table above preserves the distinctions between witnesses and restorations.

> 以旧术为之，凡应置五行。今欲要约。先置第三行，[以减 → 减以；93；汇校本/郭书春]第四行，[及减第三行?（原字OCR作王） → 又减第五行；94；汇校本/郭书春]；次置第二行，以第二行减第[三 → 一；95；汇校本/郭书春]行，[∅ → 又减第四行；96；译注本，本书采用]，去其头位；[∅ → 余，可半；97；译注本，本书采用]；次置右行[∅ → 及第二行；97；汇校本]，去其头位；次以[第四行减左行 → 右行去第四行；98；汇校本/郭书春]头位；次以左行去[第四行及 → ∅；99；汇校本/郭书春]第二行头位；次以第五行[减 → 去；100；汇校本/郭书春]第[二 → 一；100；汇校本/郭书春]行头位；次以第二行去第四行头位；[原在上一句之前的余可半 → 余，可半移至此；101；本书报告移位，未署另名]；[∅ → 以右行去第二行头位，以第二行去第四行头位；102；译注本，本书采用]。余，约之为法[∅ → 、实；103；本书校补]，实如法而一，得[空 → 六；104；戴敦元]，即有黍价。以法[孔刻本减 → 治；105；李继闵恢复旧本]第二行，得荅价，[原在第三行麻价之后的右行得菽价 → 右行得菽价移至此；106；本书校正]，[戴敦元改右 → 左；107；汇校本恢复旧本]行得麦价，第三行麻价。如此凡用七十七筭。

All changes disclosed by the consulted apparatus within the requested passage are listed, including **105–107**, which extend beyond the initially supplied 93–104 range. E94 and E101 retain the uncertainties stated above. A bracket may cover an entire clause for alignment; it does not imply that every character within that clause changed.

Chemla & Guo’s running Chinese OCR repeatedly misreads 減 as 滅 and corrupts grain names (for example 黍 as 泰, 菽 as 寂, 荅 as 苔). Those are **OCR defects, not historical emendations**. I have therefore not presented a fabricated character-perfect traditional quotation. Nor have I substituted the user’s 爲 for the transcription’s 為.

## 2. Guo 2020’s old method: operations and boards

**S:** note numbers, figure references, and block totals below. **R:** formulas, multipliers not expressly numbered in the notes, and every complete board array. Figure 8-7 itself was unavailable. [Source: notes 22–40](https://www.scribd.com/document/1021862496/).

Let `C1,…,C5` retain their original right-to-left identities; `C1` is 右行 and `C5` is 左行. Each vector is `(麻,麥,菽,荅,黍; price)`. The assignments are sequential; a source column means its **current** value. `P` means one placement event in a proposed decomposition, `M` one multiplication, `D` one division, `S` one subtraction of a whole column, `A` one addition. `kS` means **k separate subtractions**, not one combined subtraction of a scalar multiple.

| Block | Note / fig. 8-7 | Operations, in order | Source tally | Decomposition **R**, subject to caveat below |
|---|---|---|---:|---|
| 0 | 22 / (1) | Initial five-column arrangement | 1 | `P=1` |
| 1 | 23–24 / (2) | `C3←C3−C4`; `C5←C5−C3` | 3 | `P+S+S=3` |
| 2 | 25–27 / (3) | `C1←C1−C2`; `C4←C4−C1`; `C4←C4/2` | 4 | `P+S+S+D=4` |
| 3 | 28 / (4) | `C1←C1−2C3`; `C2←C2−7C3` | 11 | `2P+2S+7S=11` |
| 4 | 29 / (5) | `C4←C4−2C1` | 3 | `P+2S=3` |
| 5 | 30 / (6) | `C2←C2−2C5` | 3 | `P+2S=3` |
| 6 | 31 / (7) | **First** `C1←3C1`, **then** `C1←C1−C5` | 3 | `P+M+S=3` |
| 7 | 32 / (8) | `C4←C4+C2`; `C4←C4/2` | 3 | `P+A+D=3` |
| 8 | 33 / (9) | **First** `C2←25C2`, **then subtract C1 twenty times** | 21 | `M+20S=21` |
| 9 | 34 / (10) | **First** `C4←140C4`, **then subtract C2 twice** | 4 | `P+M+2S=4` |
| 10 | 35 | `C4←C4/62`; `v←(−30)/(−5)=6` | 2 | `D+D=2` |
| 11 | 36 | `u←[−470−(−195)·6]/140=5` | 3 | `M+S+D=3` |
| 12 | 37 | `z←[−79−(−2)·5−1·6]/(−25)=3` | 5 | `2M+2S+D=5` |
| 13 | 38 | `y←[91−(−2)·3−11·5−5·6]/3=4` | 7 | `3M+3S+D=7` |
| 14 | 39 / cf. (11) | `x←4−4·3−(−3)·5=7` | 4 | `2M+2S=4`; unit pivot |

**Placement caveat, R:** the numerical decompositions fit the published block totals but are not an explicit event ledger printed by Guo. In particular, note 33 mentions setting out C2, yet its **21** already equals one scaling plus twenty subtractions. Charging an *additional* placement there gives 22. The notes do not resolve this accounting convention. Do not turn every occurrence of 布置 into an extra counted event automatically. Likewise the factor `1·6` is retained when matching note 37’s five operations.

The complete logical board after each block follows. **All arrays are R**, computed from the notes and the supplied initial board; they do not certify which columns were physically present simultaneously. Between the displayed endpoints, the table above specifies the individual subtract/multiply/divide events.

```text
            C1                      C2                       C3                    C4                       C5
(1)  (9,7,3,2,5;140)        (7,6,4,5,3;128)         (3,5,7,6,4;116)       (2,5,3,9,4;112)          (1,3,2,8,5;95)
(2)  (9,7,3,2,5;140)        (7,6,4,5,3;128)         (1,0,4,-3,0;4)        (2,5,3,9,4;112)          (0,3,-2,11,5;91)
(3)  (2,1,-1,-3,2;12)       (7,6,4,5,3;128)         (1,0,4,-3,0;4)        (0,2,2,6,1;50)           (0,3,-2,11,5;91)
(4)  (0,1,-9,3,2;4)         (0,6,-24,26,3;100)      (1,0,4,-3,0;4)        (0,2,2,6,1;50)           (0,3,-2,11,5;91)
(5)  (0,1,-9,3,2;4)         (0,6,-24,26,3;100)      (1,0,4,-3,0;4)        (0,0,20,0,-3;42)         (0,3,-2,11,5;91)
(6)  (0,1,-9,3,2;4)         (0,0,-20,4,-7;-82)      (1,0,4,-3,0;4)        (0,0,20,0,-3;42)         (0,3,-2,11,5;91)
(7)  (0,0,-25,-2,1;-79)      (0,0,-20,4,-7;-82)      (1,0,4,-3,0;4)        (0,0,20,0,-3;42)         (0,3,-2,11,5;91)
(8)  (0,0,-25,-2,1;-79)      (0,0,-20,4,-7;-82)      (1,0,4,-3,0;4)        (0,0,0,2,-5;-20)         (0,3,-2,11,5;91)
(9)  (0,0,-25,-2,1;-79)      (0,0,0,140,-195;-470)   (1,0,4,-3,0;4)        (0,0,0,2,-5;-20)         (0,3,-2,11,5;91)
(10) (0,0,-25,-2,1;-79)      (0,0,0,140,-195;-470)   (1,0,4,-3,0;4)        (0,0,0,0,-310;-1860)     (0,3,-2,11,5;91)
/62  (0,0,-25,-2,1;-79)      (0,0,0,140,-195;-470)   (1,0,4,-3,0;4)        (0,0,0,0,-5;-30)         (0,3,-2,11,5;91)
```

Intermediate arrays useful for assertions, **R**:

```text
block 2 before halving: C4=(0,4,4,12,2;100)
block 6 after scaling:  C1=(0,3,-27,9,6;12)
block 7 before halving: C4=(0,0,0,4,-10;-40)
block 8 after scaling: C2=(0,0,-500,100,-175;-2050)
block 9 after scaling: C4=(0,0,0,280,-700;-2800)
```

After back-substitution the unknowns are `(x,y,z,u,v)=(7,4,3,5,6)`. The notes do not supply a readable post-substitution board here; the scalar assignments above are my explicit arithmetic realization.

### Counting rule

Note 40, **verbatim operative excerpt** (line wrapping removed; excerpt ends before the note ends):

> 一算即一次运算，如布算，以某数乘或除某一整行，行与行的一度减，实除以法

The rest of the note treats these as individual operations and totals the calculation at 77. Thus placement, whole-column scaling/division, one column subtraction, and final division count individually. This is an **operation count**, not a digit-by-digit arithmetic count or rod inventory. [Guo 2020, note 40](https://www.scribd.com/document/1021862496/).

```text
1+3+4+11+3+3+3+3+21+4+2+3+5+7+4 = 77
```

The passage quotation and apparatus record classical text; the excerpt above is deliberately limited to the test-relevant part of the modern note, rather than reproducing the whole modern commentary.

## 3. New method: 124, more briefly

The following is an **R expansion of the classical procedure**, cross-checked against Guo’s notes 42–62. Multiplicities are calculated, not transcribed from figure 8-8. No preliminary cross-multiplication is needed in this expansion; scalar multiples below abbreviate repeated addition/subtraction. [Classical text and notes 41–82](https://www.scribd.com/document/1021862496/).

| Note / fig. 8-8 | Expanded operations **R** |
|---|---|
| 42 / (1) | `C3←C3−C4` |
| 43 / (2) | `C1←C1−35C3`; `C2←C2−32C3`; `C4←C4−28C3` |
| 44 / (3) | `C5←C5−23C3` |
| 45 / (4) | `C3←C3−C5` |
| 46 / (5) | `C5←C5−3C3`; discard `C3` |
| 47 / (6) | `C5←C5−5C4`; `C1←C1−C4` |
| 48 / (7) | `C2←C2−3C1`; `C4←C4−4C1` |
| 49 / (8) | `C4←C4−C2`; `C5←C5+C2` |
| 50 / (9) | `C5←C5−3C4` |
| 51 / (10) | `C2←(C2+C5)/4`; classical text says 再半, two halvings |
| 52 / (11) | `C5←C5+17C4`; `C2←C2−2C4` |
| 53 / (12) | `C5←C5+11C2`; `C5←C5/62` |
| 54 / (13) | `C2←C2−19C5`; `C4←C4+8C5`; `C1←C1−5C5` |
| 55 / (14) | `C2←C2−2C1` |
| 56 / (15) | `C1←C1−2C2` |
| 57 / (16) | `C1←C1−3C5` |
| 58 / (17) | `C1←C1+2C5`; `C1←C1/5` |
| 59 / (18) | `C2←C2+2C1` |
| 60 / (19) | `C4←C4+C2`; `C5←C5+C2` |
| 61 / (20) | `C2←C2+2C5`; see sign issue below |
| 62 / (21) | After choosing the source’s sign for `C2`: `C4←C4−C2`; `C2←C2−4C4` |

Selected complete reconstructed states, **R**. Omitted columns retain their previous values; `C3` is absent after (5). Each row below lists **all active columns**.

```text
(2):  C1=(-26,7,-137,107,5;0)  C2=(-25,6,-124,101,3;0)
      C3=(1,0,4,-3,0;4)       C4=(-26,5,-109,93,4;0)  C5=(1,3,2,8,5;95)
(5):  C1=(-26,7,-137,107,5;0)  C2=(-25,6,-124,101,3;0)
      C4=(-26,5,-109,93,4;0)   C5=(-91,12,-372,317,20;0)
(7):  C1=(0,2,-28,14,1;0)      C2=(-25,0,-40,59,0;0)
      C4=(-26,-3,3,37,0;0)     C5=(39,-13,173,-148,0;0)
(10): C1=(0,2,-28,14,1;0)      C2=(-2,-1,-9,9,0;0)
      C4=(-1,-3,43,-22,0;0)    C5=(17,-4,4,-23,0;0)
(11): C1=(0,2,-28,14,1;0)      C2=(0,5,-95,53,0;0)
      C4=(-1,-3,43,-22,0;0)    C5=(0,-55,735,-397,0;0)
(12): C1=(0,2,-28,14,1;0)      C2=(0,5,-95,53,0;0)
      C4=(-1,-3,43,-22,0;0)    C5=(0,0,-5,3,0;0)
(13): C1=(0,2,-3,-1,1;0)       C2=(0,5,0,-4,0;0)
      C4=(-1,-3,3,2,0;0)       C5=(0,0,-5,3,0;0)
(16): C1=(0,0,0,-6,5;0)        C2=(0,1,6,-2,-2;0)
      C4=(-1,-3,3,2,0;0)       C5=(0,0,-5,3,0;0)
(17): C1=(0,0,-2,0,1;0)        C2=(0,1,6,-2,-2;0)
      C4=(-1,-3,3,2,0;0)       C5=(0,0,-5,3,0;0)
(19): C1=(0,0,-2,0,1;0)        C2=(0,1,2,-2,0;0)
      C4=(-1,-2,5,0,0;0)       C5=(0,1,-3,1,0;0)
(20): C1=(0,0,-2,0,1;0)        C2=(0,-3,4,0,0;0) [source sign chosen]
      C4=(-1,-2,5,0,0;0)       C5=(0,1,-3,1,0;0)
(21): C1=(0,0,-2,0,1;0)        C2=(4,-7,0,0,0;0)
      C4=(-1,1,1,0,0;0)        C5=(0,1,-3,1,0;0)
```

**U / sign issue:** literal addition at step (20) gives `C2=(0,3,-4,0,0;0)`. Note 61 explicitly reports the two nonzero values as `−3,4`. I used the globally negated equation to match that report. The missing figure prevents checking whether it records a sign reversal or the note contains an error. The relation is unchanged; the extra sign reversal must not be silently introduced into an operation-count test.

Source-reported numerical checkpoints: note 53 has `−310,186`, reduced by 62 to `−5,3`; note 57 has `−6,5`; note 58 has `−10,5`, reduced by 5 to `−2,1`; note 61 has `−3,4`; note 62 has `4,−7`. The first of these follows from the reconstructed (11) state exactly.

The resulting equivalences are `4x=7y`, `3y=4z`, `5z=3u`, `6u=5v`, hence `x:y:z:u:v=7:4:3:5:6`. Reintroduce the original third and fourth columns: `C3−C4=(1,0,4,−3,0;4)`. Then

```text
x+4z−3u=4
[1+4·(3/7)−3·(5/7)]x=4
(4/7)x=4  ⇒  x=7
```

Alternatively, the classical 其一術 uses the ratio vector as weights: divisor `1·7+4·3−3·5=4`, equal to the chosen constant 4, so the prices equal the weights. **S:** note 82 reports **124**. **U:** notes 41–82 give no corresponding block tally explaining 124, and the missing figure cannot be inspected. I have not manufactured a 124-event ledger. Guo’s transcription also labels the fifth item in note 80 麻 although its index is 5 and its value is `4×6`; that is a source/OCR anomaly, not evidence of a second hemp variable.

## 4. Other published readings and reconstructions

### Wei & Guo 2016; Zhu 2025

The accessible Wei & Guo abstract argues that 算 can mean counting rods and announces a different collation of the old method. **U:** its exact counting rule, emended passage, board states, and treatment of 124 were not available in the readable preview. I cannot quote or reconstruct those on their behalf. [Article preview](https://m.fx361.com/news/2016/1216/16730222.html).

Zhu 2025 explicitly calls the two totals **77 counting rods** and **124 counting rods**, citing Wei & Guo. His worked arrays concern 8.7 and the need for an extra copied column under mutual multiplication. They do not supply a test fixture for 8.18. His bibliography’s Chinese author name for Guo SR is 郭书春; the Wei & Guo article preview and Wei’s institutional bibliography instead give **郭世荣**. Treat Zhu’s author-name rendering as a bibliographic inconsistency. [Zhu, §3 and bibliography](https://journals.sagepub.com/doi/10.1177/20966083251414789); [Wei’s institutional publications list](https://ihst.imnu.edu.cn/info/1090/4417.htm).

### Hart 2012: an actual 77-rod construction

Hart counts rods needed to **display successive result boards**, with a maximum of 77. His path differs from Guo’s. The following formulas/numbers are readable in his pp. 323–329; `place` adds an original column and other columns persist. **S:** reported endpoint counts. This is not a bound established for every temporary multiplication workspace. [Hart’s paper](https://www.rhart.org/papers/KJHS-Hart.pdf).

| Step | Operation | Reported rods |
|---:|---|---:|
| 1 | place original `C3,C4` | 44 |
| 2 | `C3←C3−C4=(1,0,4,−3,0;4)` | 35 |
| 3 | place original `C5` | 60 |
| 4 | `C5←3C5−2C4=(−1,−1,0,6,7;61)` | 45 |
| 5 | place original `C1` | 68 |
| 6 | `C1←C1−C4=(7,2,0,−7,1;28)` | 60 |
| 7 | `C1←(C1+2C5)/5=(1,0,0,1,3;30)` | 53 |
| 8 | `C3←4C4−3C3=(5,20,0,45,16;436)` | 69 |
| 9 | `C3←(C3+20C5)/3=(−5,0,0,55,52;552)` | 75 |
| 10 | `C3←C3+5C1=(0,0,0,60,67;702)` | 53 |
| 11 | place original `C2` | **77** |
| 12 | `C2←4C4−3C2=(−13,2,0,21,7;64)` | 71 |
| 13 | `C2←(C2+2C5)/3=(−5,0,0,11,7;62)` | 67 |
| 14 | `C2←(C2+5C1)/2=(0,0,0,8,11;106)` | 62 |
| 15 | `C2←(30C2−4C3)/62=(0,0,0,0,1;6)` | 56 |

**R:** the reported counts match decimal rod costs `r(0)=0`, `r(d)=d` for `1≤d≤5`, and `r(d)=d−4` for `6≤d≤9`, summed over digits of absolute values in every present column. No extra sign rod is charged. For example, step 11 contains

```text
C1=(1,0,0,1,3;30)       cost 8
C2=(7,6,4,5,3;128)      cost 24
C3=(0,0,0,60,67;702)    cost 12
C4=(2,5,3,9,4;112)      cost 23
C5=(-1,-1,0,6,7;61)     cost 10
                         total 77
```

Hart’s prose says “subtract” at steps 7 and 9 while its displayed formulas have addition; the table follows the formulas, which check arithmetically. Some extracted matrices also have damaged entries, extra `−1`s, or wrong constants for unchanged C4. Those corruptions are not carried into the table. Its formulas are independently consistent with the supplied solution.

### Shen, Lun & Crossley; Li Jimin

**Indirect evidence only:** Hart reproduces a final board attributed to Shen, Lun & Crossley, *The Nine Chapters on the Mathematical Art* (1999), and refers to pp. 426–438. In right-to-left column order his reproduced numerical matrix is

```text
C1=(0,1,-9,3,2;4)
C2=(0,0,0,-4,-21;-146)
C3=(1,0,4,-3,0;4)
C4=(0,0,0,0,31;186)
C5=(0,0,0,5,2;37)
```

It is not Guo’s final board. Their original step sequence and counting rule were **not read**, so I do not attribute an operation tally or a rod-capacity rule to them. Hart also cites Li Jimin’s *九章算术校证* (1993) for comparison of collations; Li’s reconstruction and counting rule were **not read**. Chemla & Guo’s note 105 does directly credit Li with the restoration of 治 described above. [Hart, pp. 322–323](https://www.rhart.org/papers/KJHS-Hart.pdf); [Chemla & Guo, apparatus](https://fr.scribd.com/document/398756962/LesNeufChapitres-Karine-Chemla-Guo-Shuchun).

Chemla & Guo’s French note 156 emphasizes that reconstructions differ substantially; 157 distinguishes kinds of column operations; 168 says the interpretation of the final count remains difficult. They therefore do not establish a unique 77/124 counting convention. [French notes, pp. 876–877](https://fr.scribd.com/document/398756962/LesNeufChapitres-Karine-Chemla-Guo-Shuchun).

## Sources and limits for an executable test

1. Karine Chemla & Guo Shuchun, *Les Neuf Chapitres* (Dunod, 2004), Chinese pp. 652–655, nn. 93–107; French pp. 876–877, nn. 156–157, 168. [Accessible transcription](https://fr.scribd.com/document/398756962/LesNeufChapitres-Karine-Chemla-Guo-Shuchun). OCR read; no facsimile verification.
2. 郭书春, *九章筭术译注（修订本）* (2020), chapter 8, final 麻麥 problem, notes 21–82, references to figures 8-7/8-8. [Accessible transcription, ISBN 9787532597994](https://www.scribd.com/document/1021862496/). Text read; figure arrays unavailable.
3. 魏雪刚、郭世荣, “《九章算术》方程章‘麻麦’问刘徽注中‘算’字新释及方程‘旧术’新校”, *自然科学史研究* 35(1) (2016): 10–17. [Abstract/opening preview](https://m.fx361.com/news/2016/1216/16730222.html). Full paper not read.
4. Yiwen Zhu, “A new perspective on Liu Hui’s mathematical achievements”, *Cultures of Science* 8(S2) (2025), DOI 10.1177/20966083251414789. [Full article](https://journals.sagepub.com/doi/10.1177/20966083251414789). Article read; no 8.18 reconstruction supplied.
5. Roger Hart, “Tracing Practices Purloined by the ‘Three Pillars’”, *The Korean Journal for the History of Science* 34(2) (2012): 287–357, particularly 322–329. [Author-hosted PDF](https://www.rhart.org/papers/KJHS-Hart.pdf). Text, formulas, extracted diagrams read; OCR problems identified above.
6. Shen Kangshen, John N. Crossley & Anthony W. C. Lun, *The Nine Chapters on the Mathematical Art: Companion and Commentary* (1999), pp. 426–438: **not directly consulted**; only Hart’s quoted board used.
7. 李继闵, *九章算术校证* (1993): **not directly consulted**; only the attributions explicitly identified above used.

**Verification performed (R):** exact integer arithmetic for the reconstructed old-method column operations and new-method states; every reconstructed equation satisfies `(7,4,3,5,6)`; the old tally sums to 77; Hart’s endpoint rod costs agree with his reported counts. The remaining obstacles to a source-faithful executable test are E94’s original numeral, missing Guo figures, placement accounting at note 33, the new-method sign at note 61, and the absence of a readable derivation of the 124 tally.
