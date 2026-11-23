// Runs a page:  node run.js [options] page.jz
// The page is read here and given to jiuzhang as the one file it can open,
// under its own name, so jiuzhang needs no access to the disk. Options are
// those of `jiuzhang` before a file (--plain, --check, readings).
"use strict";

const fs = require("fs");
const path = require("path");
const { run } = require("./wasi.js");

const args = process.argv.slice(2);
const file = args.pop();
if (!file || file.startsWith("--")) {
  console.error("usage: run.js [--plain | --check | readings] page.jz");
  process.exit(2);
}
let page;
try {
  page = fs.readFileSync(file);
} catch (error) {
  console.error(`jiuzhang: cannot read ${file}: ${error.message}`);
  process.exit(2);
}

const wasm = path.join(__dirname, "jiuzhang.wasm");
const base = path.basename(file);
// A name that starts with a dash would be read as an option.
const name = base.startsWith("-") ? `./${base}` : base;
const files = { [base]: page };

if (args.includes("--check")) {
  // jiuzhang says `line N:`: put the file's name in front, so each line reads
  // path:line: and the terminal links it to the place.
  const out = [];
  const code = run(wasm, {
    args: [...args, name],
    files,
    keyboard: false,
    output: (fd, bytes) => (fd === 1 ? out.push(Buffer.from(bytes)) : process.stderr.write(bytes)),
  });
  process.stdout.write(Buffer.concat(out).toString().replace(/^line (\d+): /gm, `${file}:$1: `));
  process.exitCode = code;
} else {
  process.exitCode = run(wasm, { args: [...args, name], files, keyboard: false });
}
