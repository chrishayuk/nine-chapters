# The Nine Chapters on a counting board, for Visual Studio Code

Editor support for pages of chapter 8 of the Nine Chapters on the Mathematical Art (九章算術, 方程, "rectangular arrays"), written down so that they can be run. A `.jz` file is a counting board and what is done to it, one operation to a line, in the order the text gives them: the columns set from the right, multiplied, taken from one another until the head is gone, and the figures the text and its commentators (Liu Hui, Dai Zhen) say the board gives. `jiuzhang` does each operation on the board and compares every figure the source states with what the board holds. The notation is described in [`docs/notation.md`](../../docs/notation.md).

Nothing else needs installing: `jiuzhang`, which is both the checker and the language server, is built into the extension as WebAssembly.

## What you get

- **Colours** for headings (`No. 8.1`), the source's words in quotes (on a line of their own or at the end of an operation), the operations (`Things`, `Set`, `Multiply`, `Take`, `Add`, `Halve`, `Reduce`, `Divide`, `Drop`, `Gives`, `Solve`, `Answer`, `Rates`, `Count`, `Nothing`), the columns (`right`, `middle`, `left`, `2nd`, `3rd` …, `右行`, `中行`, `左行`, `第二行` …), the names of things (`上禾`, `實`), how a column is taken (`until the head is gone`, `as far as it goes`, `in turn`, `crosswise`), figures (`39`, `-6`, `·`, `9 1/4`, `1 18/52`), the `|` between readings, and `%` notes. The language server colours the page too, from what it has read, and its colours take the place of the grammar's.
- **Problems as you type.** A figure the source states that the board does not give is a warning that says what the board has (`the board has 0 5 1 24`); so is an operation that cannot be done (`there is no 4th column on a board of 3`). A line that is not in the notation is an error, and a remark on how an answer is written is a hint.
- **Hover** on an operation to see what it means, what it did at each position by the rule of signs (同名相除, 異名相益, 無入), how many 算 it takes under each rule of counting, the board after it, the source's words for it, and whether what the source states is right: ✓, or ✗ with what the board has. On `Solve`, the procedure followed and its steps; on a line read more than one way, that the run takes the first.
- **The outline** of a page is its headings.
- **Completion** of the operations, with an example of each, the columns and the ways of taking.
- **Snippets** `no` (a problem page), `things`, `set`, `multiply`, `take`, `gives`, `readings` and `solve`.
- **Run This Page:** the play button at the top of the editor, or Cmd+Enter (Ctrl+Enter on Windows and Linux). The page appears in a terminal worked a line at a time, with the board after each operation, every figure the source states marked ✓ or ✗, and a count of them. **Run This Page Without the Boards** prints the page, checked, without the boards; **Check This Page** prints a line for each problem, as `file:line: severity: message`, each a link to the place; **Run Every Reading of This Page** runs every combination of the readings a page gives in the alternative and lists those that agree with every figure the source states. (For `liu-hui-18-old.jz` that takes a quarter of a minute.)

## Settings

| Setting | What it does |
| --- | --- |
| `jiuzhang.path` | Use a `jiuzhang` program you built yourself, both as the language server (`jiuzhang lsp`) and to run pages. |
| `jiuzhang.trace.server` | Log the messages exchanged with the language server. |

## Building it

From this folder, with Rust (and its `wasm32-wasip1` target) and Node installed:

```sh
npm install
npm run build        # compiles jiuzhang to WebAssembly, bundles the extension
npm test             # grammar, language server and runner tests
npm run test:vscode  # opens a real VS Code and drives the extension
npm run package      # writes jiuzhang-language-0.1.0.vsix
code --install-extension jiuzhang-language-0.1.0.vsix
```

To try it without installing, open this folder in VS Code and press F5.

## How it fits together

| File | What it is |
| --- | --- |
| `syntaxes/jiuzhang.tmLanguage.json` | The highlighting grammar, used until the language server's colours arrive. |
| `language-configuration.json` | The note character and the quotation marks. |
| `snippets/jiuzhang.code-snippets` | A problem page, the operations and readings in the alternative. |
| `src/extension.js` | Starts the language server and provides the run commands. |
| `server/jiuzhang.wasm` | `jiuzhang`, compiled for WASI: one program for running pages and, as `jiuzhang lsp`, the language server. |
| `server/lsp.js` | Starts `jiuzhang lsp`. |
| `server/run.js` | Runs a page: `jiuzhang FILE`, `jiuzhang --plain FILE`, `jiuzhang --check FILE` or `jiuzhang readings FILE`, with the page as the one file it can open. |
| `server/wasi.js` | A small WASI host for Node, with no dependencies. |

## Licence

Apache License 2.0. See [LICENSE](LICENSE).
