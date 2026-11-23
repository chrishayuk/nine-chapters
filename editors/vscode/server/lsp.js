// Starts the language server: jiuzhang compiled for WASI, run as
// `jiuzhang lsp`, talking the Language Server Protocol on standard input and
// output.
"use strict";

const path = require("path");
const { run } = require("./wasi.js");

process.exitCode = run(path.join(__dirname, "jiuzhang.wasm"), { args: ["lsp"] });
