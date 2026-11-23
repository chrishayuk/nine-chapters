<!-- A literature search by a second model, as written, 23 November 2026. It searched for prior work on each of the project's findings. NOT FOUND means not found in what it could read, never that a finding is new. What was done about it is in the README, under "Earlier work". -->

# Literature check: Nine Chapters, chapter 8

**Researcher:** OpenAI Codex, based on GPT-6 (AI, not a human historian). **Date:** 23 November 2026.

Read the project's `README.md`, `docs/independent-review.md`, `tutorial/08-counting.md`, and `tutorial/09-a-corrupt-passage.md` at `/Users/christopherhay/chris-source/esoteric/nine-chapters`. The project was not modified. This is a literature search, not a rerun or independent verification of its arithmetic.

## Search and evidence limits

Searched English, Chinese (simplified and traditional), and French, combining author/title searches with: `益實六斗正`, `益实六斗 正 负 校`, `及減第三行`, `得空 得六`, `凡用七十七算`, `麻麦 旧术 新校`, `一百四十五算`, `九十九算`, `戴震 145 99`, `Dai Zhen 145 operations/calculs`, `Liu Hui 77 124`, `fangcheng common denominator`, `fangcheng denominators Hart`, `九章 五十二 分母 行列式`, `Les neuf chapitres dénominateur 111`, and numerical-size/repeated-subtraction queries. Searched within accessible books and papers for the counts, denominators, problem numbers, and emendations.

**Full-text passages read:** Relevant sections of Li Huang's chapter 8, Hart 2011, Chemla–Guo 2004, and Guo Shuchun 2020 through online transcriptions; Zhu 2010, pp. 77–82 and 87; Zhu's 2025 *Cultures of Science* article, especially §3; and the introduction, methodological discussion and conclusion of Zhu's 2025 *Science in Context* article. “Full text accessible” below does not mean that I read every page of the book. Scribd transcriptions contain OCR errors and sometimes omit diagrams; I used readable prose and numbered notes, not their AI descriptions. PDF image requests failed, so these are text-level checks, not fresh inspections of the printed pages.

**Partial access:** Wei–Guo 2016 abstract and opening paragraphs only; the page explicitly requires its app for the remainder. Shen–Crossley–Lun's chapter 8 could not be opened. Guo's *Huijiao*, Qian's editions, Li Jimin's and Bai Shangshu's books were bibliographic leads, not independently read books. Their contents are not inferred here. NOT FOUND means no match in this search, never proof of novelty.

## Verdicts

| Claim | Verdict | What the evidence establishes |
|---|---|---|
| 1. 8.6: 正 / 負 | **KNOWN** | Li Huang explicitly corrects both signs; Chemla–Guo records the variant and credits him. |
| 2. Common written denominator reflects 法 / determinant | **PARTLY KNOWN** | Hart explicitly discusses 33/111 rather than 11/37 and the determinant; the project's complete eleven-answer gcd census was not found. |
| 3. Dai's 145/99, particularly the entrywise 145 rule | **NOT FOUND** | The historical numbers are readily identifiable, but I found no accessible scholarly reconstruction establishing this exact convention. Unread books remain unresolved. |
| 4. Liu's reconstructed passages and 77/124 | **PARTLY KNOWN** | Both passages are edited and worked; Guo gives an explicit 77 tally. Counting interpretations differ; I did not obtain an explicit 124 tally. |
| 5. Intermediate growth and physical cost | **PARTLY KNOWN** | Number growth, reduction, repeated subtraction and rod economy are discussed; the exact 11/4-digit and ~180,000-operation comparison was not found. |
| 6. Restored third column and rates 7:4:3:5:6 | **KNOWN** | Explicitly explained in Guo's annotations, also printed in Chemla–Guo. |

## 1. Problem 8.6: the two signs

**KNOWN.** Li Huang 李潢, *九章算術細草圖說*, juan 8, the problem beginning 今有上禾三秉益實六斗, “潢按” and the following 草/說, preserves the 正 readings and says **“正正皆当作负”**. He then works with negative constants and obtains lower grain 3, upper grain 8. His explanation identifies a printing error and observes that the transmitted signs leave divisor and dividend with different signs. This is prior diagnosis and repair of the same difficulty. [S1](https://www.shidianguji.com/book/CADAL02094024/chapter/1lcpweflpsmy3)

Karine Chemla and Guo Shuchun, *Les neuf chapitres* (Paris: Dunod, 2004), **p. 632, Chinese editorial note 51**, prints 負 twice, records 正 in the 大典 and 楊輝 witnesses, and credits Li Huang's correction. It is not presented as a variant peculiar to Weiboxie. [S2](https://fr.scribd.com/document/398756962/LesNeufChapitres-Karine-Chemla-Guo-Shuchun)

I did not find the literal result **−8, −3** expressly stated in the scholarship read. The sign error and corrective reading are nevertheless known. A search snippet from Lao Naixuan's *古籌筭考釋*, juan 4, actually disputes Li's correction under an alternative sign convention; its readable snippet is a lead to historical disagreement, not evidence that all scholars accepted the emendation. [S9](https://www.shidianguji.com/book/NCL06403A/chapter/1lzvlf1ced9o3)

## 2. Written denominators, 法 and determinants

**PARTLY KNOWN.** Roger Hart, *The Chinese Roots of Linear Algebra* (Baltimore: Johns Hopkins University Press, 2011), **pp. 173–174, step 14 and note 14**, explicitly notices **198/666 reduced to 33/111 rather than 11/37**. All displayed answers use 111. He identifies the common denominator of the general solution with the coefficient determinant: **p. 173; p. 174, equations (8.26)–(8.29)**. [S3](https://www.scribd.com/document/983816077/The-Chinese-Roots-of-Linear-Algebra)

Hart's **pp. 97–99, “Back Substitution,” final step**, reconstructs all answers over the final pivot, followed by reduction, explicitly labeling the physical arrangement hypothetical.

I did **not** locate an explicit statement that **all eleven fractional answers** equal the final 法 and every 實 reduced by their joint gcd, nor discussion specifically explaining **18/52 versus 9/26**. A determinant-related denominator and this exhaustive textual rule should remain separate claims; a written denominator need not equal the raw determinant without reduction/scaling.

## 3. Dai Zhen's 145 and 99

**NOT FOUND in the accessible literature searched.** The primary note is visible in the indexed Sibu page, including the 145 total and the saving of equal cross-multiplied heads. This establishes the historical statement, not a later reconstruction of its accounting. [S10](https://wk.100ke.info/wiki/Page%3ASibu_Congkan0392-%E5%8A%89%E5%BE%BD-%E4%B9%9D%E7%AB%A0%E7%AE%97%E8%A1%93-3-3.djvu/48)

Searched the Chinese number phrases, Arabic 145/99, Dai Zhen/戴震 with 方程/麻麦, and French and English operation-count terms; also searched accessible Hart, Chemla–Guo and Guo transcriptions. No passage read identifies **145 as entrywise multiplication, subtraction and division, omitting both multiplication and subtraction of the cancelling heads**, or establishes the project's division-by-one convention yielding 99.

Hart's **Table 2.2, p. 24**, gives **175** operations for generic five-variable Gaussian elimination; this does not establish Dai's 145. [S3](https://www.scribd.com/document/983816077/The-Chinese-Roots-of-Linear-Algebra)

**Access limitation:** I cannot exclude a reconstruction in Shen–Crossley–Lun 1999, Guo's *Huijiao*, or other unavailable Chinese editions. Wei–Guo's accessible abstract concerns Liu's counts, and does not establish that it treats Dai's.

## 4. Liu Hui's 77 and 124; textual repairs

**PARTLY KNOWN, with substantial direct prior reconstruction.**

**Emendations:** Chemla–Guo 2004, **pp. 652–655, editorial notes 93–104**, replaces the opening sequence with 第三行減以第四行、又減第五行 and makes further changes, additions and relocations. Note 94 specifically repairs 三/及 to 五/又. Note 104 restores **六 for 空**, crediting **Dai Dunyuan 戴敦元**, not Dai Zhen. The French notes **pp. 876–877, notes 156–157**, discuss competing reconstructions and the operations involved. **P. 877, note 168** explicitly says that the meaning of the count remains difficult to determine. [S2](https://fr.scribd.com/document/398756962/LesNeufChapitres-Karine-Chemla-Guo-Shuchun)

**An explicit 77 reconstruction:** Guo Shuchun 郭书春, *九章筭术译注（修订本）* (2020; full citation below), **chapter 8, last problem, notes 22–40, figure 8-7**, tallies 77. Note 40 counts placing rods, a whole-column multiplication/division, one column subtraction, or 實/法 division as one 算. Successive block counts are:

`1 + 3 + 4 + 11 + 3 + 3 + 3 + 3 + 21 + 4 + 2 + 3 + 5 + 7 + 4 = 77`.

**Notes 41–82, figure 8-8**, work the new method; note 82 repeats 124 without a comparable tally. [S4](https://www.scribd.com/document/1021862496/九章算术译注-修订本-中国古代科技名著译注丛书-9787532597994-48629)

**A competing interpretation:** Wei Xuegang 魏雪刚 and **Guo Shirong 郭世荣** (not Guo Shuchun), “《九章算术》方程章‘麻麦’问刘徽注中‘算’字新释及方程‘旧术’新校,” *自然科学史研究* **35(1) (2016): 10–17**. Its accessible **abstract and opening** propose interpreting 算 as counting rods, offer another old-method collation, and report disagreement over previous readings and counts. I could not read its detailed reconstruction or reproduce its accounting. Author identification is corroborated by Wei's university bibliography. [S5](https://m.fx361.com/news/2016/1216/16730222.html), [S11](https://ihst.imnu.edu.cn/info/1090/4417.htm)

Yiwen Zhu, “A new perspective on Liu Hui's mathematical achievements,” *Cultures of Science* **8(S2) (2025): 161–171**, **§3, p. 167**, explicitly interprets **77 and 124 as numbers of counting rods**, citing Wei–Guo. It discusses a tradeoff between steps and rods, but does not give the detailed 77/124 tally there. [S6](https://journals.sagepub.com/doi/pdf/10.1177/20966083251414789)

Consequently, the passages' corruption and repair are known; failure of the project's particular reading menus does not establish failure of published reconstructions. The unit of counting remains disputed in the sources read.

## 5. Number growth, reduction and physical cost

**PARTLY KNOWN.** Zhu Yiwen 朱一文, “數：筭與術——以九數之方程為例,” *漢學研究* **28(4) (December 2010): 73–105**, **pp. 80–82**, directly discusses large numbers, numerous operations and numerous rods as forms of 繁, with reduction as a means of economy. **P. 82** contrasts repeated-subtraction 直除, which saves rods but takes multiple subtractions, with cross-multiplication, which saves subtraction steps but requires more rods. **P. 87** discusses unnecessary growth in written calculations. This is directly relevant prior work, not merely a generic mention of counting rods. [S7](https://ccsndb.ncl.edu.tw/nclccs/pdf/01_028_004_01_03.pdf)

Hart **pp. 101–103** considers common-divisor reductions and warns that finding a gcd for large numbers may be difficult; **pp. 108–109** examines the effects of reduction on integer-preserving back substitution. [S3](https://www.scribd.com/document/983816077/The-Chinese-Roots-of-Linear-Algebra)

Zhu, “The interplay between textual procedures and material operations from the viewpoint of Chinese mathematical texts,” *Science in Context* (2025), DOI **10.1017/S0269889725100860**, **advance PDF pp. 1–4 and 18–19**, addresses the text/material-operation distinction. Its conclusion warns that an operational reconstruction fitting a text need not be historically unique or correct. Its main worked subjects are root extraction and later counting diagrams. [S8](https://www.cambridge.org/core/services/aop-cambridge-core/content/view/7E654BC8863452F26F2E0C43892699F4/S0269889725100860a.pdf/div-class-title-the-interplay-between-textual-procedures-and-material-operations-from-the-viewpoint-of-chinese-mathematical-texts-div.pdf)

**Not found:** the exact **11 versus 4 digits**, **180,330 individual subtractions / 180,403 operations**, or **89,773 whole-column subtractions** for 8.18. These remain measurements of the project's selected implementation, not established historical workload. The project's own qualification matters: recorded-board sizes exclude temporary products, including its reported 16-digit product.

## 6. Restored third column and the five rates

**KNOWN.** Guo 2020, **chapter 8, last problem, notes 63–69**, explicitly derives the four pairwise relations, then **麻:麥:菽:荅:黍 = 7:4:3:5:6**. Note 69 explains resetting the third column and subtracting the fourth as **x + 4z − 3u = 4**, exactly the stated 麻1、菽4、荅−3、實4. [S4](https://www.scribd.com/document/1021862496/九章算术译注-修订本-中国古代科技名著译注丛书-9787532597994-48629)

Chemla–Guo also prints and translates this passage: **pp. 656–657**, editorial notes **115–119**, and the following explanation of conversion to hemp. [S2](https://fr.scribd.com/document/398756962/LesNeufChapitres-Karine-Chemla-Guo-Shuchun)

## Sources and URLs used

These are source URLs used for evidence, bibliographic leads, or access checks. Incidental irrelevant search hits are excluded. Section and note references above identify the actual passages read.

### Evidence sources

1. **S1 — Li Huang 李潢, *九章算術細草圖說*, juan 8.** Read online chapter transcription: <https://www.shidianguji.com/book/CADAL02094024/chapter/1lcpweflpsmy3>. Traditional-text search version: <https://www.shidianguji.com/zh/book/CADAL02094024/chapter/1lcpweflpsmy3>. The transcribed copy's exact imprint was not established; the work predates modern scholarship.
2. **S2 — Chemla, Karine, and Guo Shuchun. 2004. *Les neuf chapitres: le classique mathématique de la Chine ancienne et ses commentaires*. Paris: Dunod.** Read relevant book transcription: <https://fr.scribd.com/document/398756962/LesNeufChapitres-Karine-Chemla-Guo-Shuchun>.
3. **S3 — Hart, Roger. 2011. *The Chinese Roots of Linear Algebra*. Baltimore: Johns Hopkins University Press.** Read relevant book transcription: <https://www.scribd.com/document/983816077/The-Chinese-Roots-of-Linear-Algebra>. Publisher metadata/contents: <https://www.press.jhu.edu/books/title/10232/chinese-roots-linear-algebra>. MUSE contents, restricted downloads: <https://muse.jhu.edu/book/487>.
4. **S4 — Guo Shuchun 郭书春. 2020. *九章筭术译注（修订本）*. Shanghai: 上海古籍出版社. ISBN 9787532597994.** Read copyright information and relevant text/annotations: <https://www.scribd.com/document/1021862496/九章算术译注-修订本-中国古代科技名著译注丛书-9787532597994-48629>. Canonical indexed spelling: <https://www.scribd.com/document/1021862496/%E4%B9%9D%E7%AB%A0%E7%AE%97%E6%9C%AF%E8%AF%91%E6%B3%A8-%E4%BF%AE%E8%AE%A2%E6%9C%AC-%E4%B8%AD%E5%9B%BD%E5%8F%A4%E4%BB%A3%E7%A7%91%E6%8A%80%E5%90%8D%E8%91%97%E8%AF%91%E6%B3%A8%E4%B8%9B%E4%B9%A6-9787532597994-48629>.
5. **S5 — Wei Xuegang 魏雪刚 and Guo Shirong 郭世荣. 2016. “《九章算术》方程章‘麻麦’问刘徽注中‘算’字新释及方程‘旧术’新校.” *自然科学史研究* 35(1): 10–17.** Abstract/opening only: <https://m.fx361.com/news/2016/1216/16730222.html>. Desktop alternative failed: <https://www.fx361.com/page/2016/1216/16730222.shtml>.
6. **S6 — Zhu, Yiwen. 2025. “A new perspective on Liu Hui's mathematical achievements.” *Cultures of Science* 8(S2): 161–171. DOI 10.1177/20966083251414789.** PDF text: <https://journals.sagepub.com/doi/pdf/10.1177/20966083251414789>; indexed article/section: <https://journals.sagepub.com/doi/10.1177/20966083251414789>; full-text route also surfaced: <https://journals.sagepub.com/doi/full/10.1177/20966083251414789>.
7. **S7 — Zhu Yiwen 朱一文. 2010. “數：筭與術——以九數之方程為例.” *漢學研究* 28(4): 73–105.** PDF text: <https://ccsndb.ncl.edu.tw/nclccs/pdf/01_028_004_01_03.pdf>. Abstract/metadata: <https://toaj.stpi.niar.org.tw/index/journal/volume/article/4b1141f9894fbc3f0189520477c6004c>.
8. **S8 — Zhu, Yiwen. 2025. “The interplay between textual procedures and material operations from the viewpoint of Chinese mathematical texts.” *Science in Context*. DOI 10.1017/S0269889725100860.** Read portions of advance PDF, paginated 1–22: <https://www.cambridge.org/core/services/aop-cambridge-core/content/view/7E654BC8863452F26F2E0C43892699F4/S0269889725100860a.pdf/div-class-title-the-interplay-between-textual-procedures-and-material-operations-from-the-viewpoint-of-chinese-mathematical-texts-div.pdf>. DOI route failed: <https://doi.org/10.1017/S0269889725100860>. Bibliographic/search leads give issue 36(3): 213–234 and sometimes year 2023; I cite 2025 because the PDF's own citation instruction does so. Leads: <https://philpapers.org/rec/ZHUTIB>; <https://www.researchgate.net/publication/396360069_The_interplay_between_textual_procedures_and_material_operations_from_the_viewpoint_of_Chinese_mathematical_texts>.
9. **S9 — Lao Naixuan 勞乃宣, *古籌筭考釋*, juan 4.** Relevant sign discussion read as search snippet: <https://www.shidianguji.com/book/NCL06403A/chapter/1lzvlf1ced9o3>. Alternative opened text has poor OCR; other portions read: <https://www.shidianguji.com/zh/book/NCL06403A/chapter/1lzvlf1ced9o3>. No clean printed-page verification of the sign passage.
10. **S10 — Sibu congkan primary-text page, Dai's note.** Indexed transcription only; direct page fetch failed: <https://wk.100ke.info/wiki/Page%3ASibu_Congkan0392-%E5%8A%89%E5%BE%BD-%E4%B9%9D%E7%AB%A0%E7%AE%97%E8%A1%93-3-3.djvu/48>.
11. **S11 — Wei's university bibliography.** Author attribution: <https://ihst.imnu.edu.cn/info/1090/4417.htm>.

### Other bibliographic leads and access checks

- **Shen Kangshen, John N. Crossley and Anthony W.-C. Lun. 1999. *The Nine Chapters on the Mathematical Art: Companion and Commentary*. Oxford University Press / Science Press.** Book metadata/contents read: <https://academic.oup.com/book/52966>. Chapter 8 failed to open: <https://academic.oup.com/book/52966/chapter/421961670>. Review surfaced, not a substitute for the chapter: <https://www.cambridge.org/core/services/aop-cambridge-core/content/view/9088D34DA32850AD3EB7DF974F66D38D/S0013091501224419a.pdf/shen-kangshen-crossley-j-n-and-lun-a-w-c-the-nine-chapters-on-the-mathematical-art-companion-and-commentary-oxford-university-press-and-science-press-beijing-1999-xiv596-pp-0-19-853936-3-oup-and-7-03-.pdf>.
- **Guo, Li Jimin, Bai Shangshu and Shen follow-up locations:** Zhu 2010, p. 80 n. 19, points respectively to *古代世界數學泰斗劉徽*, pp. 164–172; *九章算術導讀與譯注*, pp. 676–682; *九章算術今譯*, pp. 393–405; and *九章算術導讀*, pp. 605–613. These are **references seen in Zhu, not pages I read**. Guo *Huijiao* bibliographic search results: <https://ihns.cas.cn/sourcedb/cn/ihnsmonograph/200906/t20090606_497122.html>; <https://search.megbook.com.hk/mall/detail.jsp?proID=2441428>.
- **Guo 2020 publication metadata:** <https://search.megbook.com.tw/mall/detail.jsp?proID=3579634>; <https://www.airitibooks.com/Publication/Details?publicationID=P202502270508>. The latter dates the electronic issue 2021; the transcription's copyright page dates the book November 2020.
- **Other Chemla–Guo bibliographic/review search leads, not evidence for the six claims:** <https://ci.nii.ac.jp/ncid/BA70372160>; <https://sciamvs.org/files/SCIAMVS_07_213-218_Review_Horiuchi_On_Chemla_Guo.pdf>; <https://www.persee.fr/doc/etchi_0755-5857_2005_num_24_1_1366_t1_0342_0000_1>; <https://donwagner.dk/Chemla-rev/Chemla-rev.pdf>.
- **Li Huang attribution/edition leads:** <https://ctext.org/wiki.pl?if=gb&remap=gb&res=884215>; <https://gujiwu.cn/guji/3766.html>; <https://www.shidianguji.com/book/CADAL02094024> (last URL failed). The claim evidence uses the chapter itself.
- **Additional 直除 / computational-history leads, snippets only:** <https://pure.mpg.de/pubman/item/item_3134809_11/component/file_3401737/chemla_2019_different.pdf>; <https://bibnum.publimath.fr/ACF/ACF19057.pdf>; <https://bibnum.publimath.fr/ACF/ACF15001.pdf>; <https://wab.uib.no/agora-alws/article/viewFile/2605/2933>; <https://www.degruyterbrill.com/document/doi/10.1515/asia-2025-0024/pdf>. None established the exact 8.18 costs.
- **Other failed full-text/access attempts:** <https://www.sciengine.com/parse/pdf/2096-4226/AFC55AA8F2D241F3BBEA7BC7F0919C4E.pdf> (403); <https://cdn.sciengine.com/doi/pdf/210DBDD378724631952F70A1103B1F5F> (502); <https://zrkxs.ihns.ac.cn/>; <https://www.rhart.org/>; <https://ctext.org/text.pl?if=gb&node=51678&remap=gb&show=parallel> (403; search snippet has 負 twice). No findings are attributed to unread full texts.
