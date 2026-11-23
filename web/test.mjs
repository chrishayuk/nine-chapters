// Runs every page through the WebAssembly build and compares with the
// native build:  node web/test.mjs   (after node web/build.mjs and cargo build --release)
import { execFileSync } from "node:child_process";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { jiuzhang } from "./worker.js";
import { answer } from "./lsp-worker.js";
import { rodsOf, widthsOf } from "./board3d.js";

const root = fileURLToPath(new URL("..", import.meta.url));
const module = await WebAssembly.compile(readFileSync(`${root}web/jiuzhang.wasm`));
const native = `${root}target/release/jiuzhang`;
let failed = 0;
const fail = (m) => (failed++, console.log("FAIL " + m));
const pages = JSON.parse(readFileSync(`${root}web/corpus.json`, "utf8"));
for (const page of pages) {
  const wasm = jiuzhang(module, ["json", "-"], page.text);
  const want = execFileSync(native, ["json", "-"], { input: page.text }).toString();
  if (wasm.out !== want) fail(`${page.name}: the wasm build and the native build differ`);
  const run = JSON.parse(wasm.out);
  const boards = run.lines.flatMap((l) => l.steps.map((s) => s.board));
  if (boards.length && widthsOf(boards).length === 0) fail(`${page.name}: no widths`);
}
// the language server answers in the page as it does in an editor
const p1 = pages.find((p) => p.name === "no-01").text;
const lines = p1.split("\n");
const take = lines.findIndex((l) => l.startsWith("Take right from middle"));
const lsp = answer(module, p1, [
  { method: "textDocument/semanticTokens/full" },
  { method: "textDocument/hover", params: { position: { line: take, character: 1 } } },
]);
if (!lsp.results[0]?.data?.length) fail("lsp: no semantic tokens");
if (!/同名相除/.test(lsp.results[1]?.contents?.value ?? "")) fail("lsp: hover does not give the rule of signs");
const p6 = pages.find((p) => p.name === "no-06").text;
if (answer(module, p6, []).diagnostics.filter((d) => d.severity <= 2).length !== 1) fail("lsp: no-06 should have one problem");
// rods: 8 is a five and three; 721 is 7 upright, 2 lying, 1 upright
if (rodsOf(8).length !== 4) fail("rods of 8");
if (rodsOf(721).length !== 3 + 2 + 1) fail("rods of 721");
if (rodsOf(20).filter((r) => r.along === "x").length !== 2) fail("tens lie flat");
const s = jiuzhang(module, ["solve", "3", "2", "1", "39", "/", "2", "3", "1", "34", "/", "1", "2", "3", "26"]);
if (!/法 36/.test(s.out)) fail("solve");
console.log(`${pages.length} pages through the wasm build, ${failed ? failed + " failed" : "all agree with the native build"}`);
process.exit(failed ? 1 : 0);
