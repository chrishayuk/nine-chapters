// Builds the files the playground needs:  node web/build.mjs
//   web/jiuzhang.wasm   jiuzhang, compiled for WASI: the checker and the language server (`jiuzhang lsp`)
//   web/corpus.json     every page of the transcription: the problems in order, then the commentaries
import { execSync } from "node:child_process";
import { copyFileSync, readFileSync, readdirSync, writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

const root = fileURLToPath(new URL("..", import.meta.url));
execSync("cargo build --release --target wasm32-wasip1", { cwd: root, stdio: "inherit" });
copyFileSync(`${root}target/wasm32-wasip1/release/jiuzhang.wasm`, `${root}web/jiuzhang.wasm`);

const order = (name) => {
  const m = /(\d+)/.exec(name);
  const n = m ? parseInt(m[1], 10) : 99;
  return [n, name.startsWith("no-") ? 0 : 1, name];
};
const cmp = (a, b) => {
  const [x, y] = [order(a), order(b)];
  for (let i = 0; i < 3; i++) if (x[i] !== y[i]) return x[i] < y[i] ? -1 : 1;
  return 0;
};
const names = readdirSync(`${root}corpus`).filter((f) => f.endsWith(".jz")).map((f) => f.slice(0, -3)).sort(cmp);
const pages = names.map((name) => {
  const text = readFileSync(`${root}corpus/${name}.jz`, "utf8");
  const title = (/^No\. (.*)$/m.exec(text) || [, name])[1];
  const words = (/^"([^"]*)"/m.exec(text) || [, ""])[1];
  return { name, title, words, text };
});
writeFileSync(`${root}web/corpus.json`, JSON.stringify(pages));
console.log(`web/jiuzhang.wasm, web/corpus.json (${pages.length} pages) written`);
