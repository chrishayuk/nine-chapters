# The Nine Chapters in your editor

Two pieces give an editor an understanding of a page:

- **`jiuzhang lsp`**, a language server. It is the `jiuzhang` program itself, given the argument `lsp`, and it works with any editor that speaks the Language Server Protocol.
- **A Visual Studio Code extension** in [`vscode`](vscode), which bundles `jiuzhang` and adds syntax highlighting and run buttons. Its [README](vscode/README.md) lists the features.

Both are built on the reconstruction itself: [`src/page.rs`](../src/page.rs) reads the page, [`src/board.rs`](../src/board.rs) is the counting board and what is done to it, and [`src/check.rs`](../src/check.rs), the code behind `jiuzhang FILE` and `jiuzhang --check`, compares every figure the source states with what the board holds. So the editor and `jiuzhang` cannot disagree about what is on the page. The notation is in [`docs/notation.md`](../docs/notation.md).

A `.jz` file is a counting board and what is done to it, one operation to a line: a heading (`No. 8.1`), the source's words in double quotes, `Things` naming the positions of a column, the columns set from the right (`Set right 3 2 1 39`), the operations the text describes (`Take right from middle until the head is gone.`), what the source says the board gives (`Gives`, `Answer`, `Rates`, `Count`), and `|` between the readings of a clause that can be read more than one way. `%` starts a note to the end of the line; notes are not in the text.

## The language server

```sh
cargo install --git https://github.com/chrishayuk/nine-chapters jiuzhang
```

or, in a clone of this repository, `cargo build --release`, which writes `target/release/jiuzhang`.

`jiuzhang lsp` talks on standard input and output. It provides diagnostics, hover, semantic tokens, document symbols and completion.

- **Diagnostics.** A figure the source states that the board does not give is a warning, with what the board has; so is an operation that cannot be done. A line that is not in the notation is an error, and a remark on how an answer is written is a hint.
- **Hover** on an operation shows what it means, what it did at each position by the rule of signs, how many 算 it takes under each rule of counting, and the board after it; on a line of what the source states, ✓ or ✗.
- **Semantic tokens** mean an editor with no grammar for the notation still gets colours. The legend is `comment` (notes), `string` (the source's words), `keyword` (operations), `number` (figures), `operator` (the joining words and the `|` between readings), `namespace` (columns), `variable` (things) and `macro` (headings).
- **Document symbols** are the headings, each from its line to the next heading.
- **Completion** of the operations, the columns and the ways of taking.

It also builds for WASI (`cargo build --release --target wasm32-wasip1`, which writes `target/wasm32-wasip1/release/jiuzhang.wasm`). That one file is both the checker and the server, and is how the VS Code extension runs it.

### Neovim (0.11 or later)

```lua
vim.filetype.add({ extension = { jz = "jiuzhang" } })
vim.lsp.config("jiuzhang", {
  cmd = { "/path/to/jiuzhang", "lsp" },
  filetypes = { "jiuzhang" },
})
vim.lsp.enable("jiuzhang")
```

Neovim colours from semantic tokens without a grammar. To have `gcc` comment a line, add `vim.bo.commentstring = "% %s"` in a `jiuzhang` filetype autocommand.

### Helix

In `languages.toml`:

```toml
[language-server.jiuzhang]
command = "/path/to/jiuzhang"
args = ["lsp"]

[[language]]
name = "jiuzhang"
scope = "source.jz"
file-types = ["jz"]
comment-token = "%"
language-servers = ["jiuzhang"]
```

Helix does not colour from semantic tokens, so without a tree-sitter grammar the page is not highlighted; diagnostics, hover, symbols and completion work.

### Emacs (Eglot)

```elisp
(define-derived-mode jiuzhang-mode text-mode "Jiuzhang"
  (setq-local comment-start "% "))
(add-to-list 'auto-mode-alist '("\\.jz\\'" . jiuzhang-mode))
(with-eval-after-load 'eglot
  (add-to-list 'eglot-server-programs '(jiuzhang-mode "/path/to/jiuzhang" "lsp")))
```

### Any other LSP client

Start `jiuzhang lsp` for files that end in `.jz`, with the language id `jiuzhang`. The server wants the whole text on every change (full document sync) and counts positions in UTF-16 code units.

These configurations follow each editor's documented format but have not been run here; the server itself is tested over standard input and output by `vscode/test/lsp.test.mjs`, against both the WebAssembly build and the native program.

## Running a page from the command line

```sh
jiuzhang page.jz
```

prints the page worked a line at a time, with the board after each operation and every figure the source states marked ✓ or ✗.

```sh
jiuzhang --check page.jz
```

prints one line per problem, as `line N: severity: message`, and exits with status 1 if a line is not in the notation, a figure is wrong or an operation cannot be done. The severities are `notation`, `wrong`, `fails` and `note`. `jiuzhang --plain page.jz` prints the page checked, without the boards, and `jiuzhang readings page.jz` runs every combination of the readings a page gives in the alternative.
