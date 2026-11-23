# Sources

The page images and the downloaded texts are not kept in the repository
(see `.gitignore`); this is where they came from, so they can be fetched
again.

## The text

- **Sibu congkan** (四部叢刊) photographic reprint of the **Weiboxie**
  (微波榭) edition of the *Nine Chapters* (九章算術), with Liu Hui's
  commentary (263), Li Chunfeng's notes (656) and Dai Zhen's collation notes
  (訂訛, 1770s). Chapter 8 is in `Sibu Congkan0392-劉徽-九章算術-3-3.djvu` on
  Wikimedia Commons, djvu pages 2-53. Public domain.
  - `sibu-congkan/pages/pNN.jpg`: those pages at 1600 px, from Commons.
    This is the witness: every figure in `corpus/` is read from these.
  - `sibu-congkan/*.txt`, `*.html`: Wikisource's transcription of the same
    pages (`九章算術 (四部叢刊本)/卷第八`), marked unproofread; used to find
    places, never for figures. `卷第八.paged.txt` has page markers `【p.NN】`.
- **Wikisource 九章算術**: a punctuated text of the main text, without the
  commentary, from another edition. `wikisource/jiuzhang.wiki`. A third
  witness for the main text.

## Fetching them again

The scripts used are in `../scripts/`: `fetch-text.py` (the Wikisource text,
through the MediaWiki API) and `fetch-scans.py` (the page images, through the
Commons API). Both send a plain user agent and wait between requests, as
Wikimedia's robot policy asks.

## Secondary literature

Not read by this project, and so not relied on: K. Chemla and Guo Shuchun,
*Les neuf chapitres* (2004); Shen Kangshen, J. N. Crossley and A. W.-C. Lun,
*The Nine Chapters on the Mathematical Art: Companion and Commentary* (1999);
Guo Shuchun's critical editions; R. Hart, *The Chinese Roots of Linear
Algebra* (2011). Yiwen Zhu, "The interplay between textual procedures and
material operations from the viewpoint of Chinese mathematical texts",
*Science in Context* (2025), was read only in its abstract.
