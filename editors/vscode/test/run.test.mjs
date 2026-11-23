// Runs every page of the corpus through the runner the "Run This Page"
// command uses, and compares what it prints, and its exit status, with what
// the native jiuzhang does, when that has been built.
import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { existsSync, mkdtempSync, readdirSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

const runner = fileURLToPath(new URL("../server/run.js", import.meta.url));
const corpus = fileURLToPath(new URL("../../../corpus/", import.meta.url));
const native = fileURLToPath(new URL("../../../target/release/jiuzhang", import.meta.url));

function run(text, options = [], name = "page.jz") {
  const file = join(mkdtempSync(join(tmpdir(), "jiuzhang-")), name);
  writeFileSync(file, text);
  return { file, ...spawnSync(process.execPath, [runner, ...options, file]) };
}

const page = `No. 8.1
Things 上 中 下 實
Set right 3 2 1 39
Set middle 2 3 1 34
Set left 1 2 3 26
Multiply middle by the head of right.
Take right from middle until the head is gone.
Gives middle 0 5 1 25
`;

test("the built-in jiuzhang runs every page of the corpus", () => {
  const names = readdirSync(corpus).filter((f) => f.endsWith(".jz"));
  assert.ok(names.length >= 26);
  for (const file of names) {
    const result = spawnSync(process.execPath, [runner, corpus + file]);
    // 1 is a page with a figure the board does not give (No. 8.6 as the text has it)
    assert.ok(result.status === 0 || result.status === 1, `${file}: ${result.status} ${result.stderr}`);
    const out = result.stdout.toString();
    assert.match(out, /^No\. /, file);
    assert.match(out, /\d+ figures? checked, \d+ right, \d+ wrong\.( \d+ not reached\.)?\n/, file);
    if (existsSync(native)) {
      const want = spawnSync(native, [corpus + file]);
      assert.equal(out, want.stdout.toString(), file);
      assert.equal(result.status, want.status, file);
    }
  }
});

test("No. 8.1 is worked on the board and its answer checked", () => {
  const result = spawnSync(process.execPath, [runner, corpus + "no-01.jz"]);
  assert.equal(result.status, 0, result.stderr.toString());
  const out = result.stdout.toString();
  assert.match(out, /^ +left +middle +right$/m);
  assert.match(out, /^ +實 +99 +24 +39$/m);
  assert.match(out, /^Answer 9 1\/4, 4 1\/4, 2 3\/4 +✓/m);
  assert.match(out, /1 figure checked, 1 right, 0 wrong\./);
});

test("a page is run, and a figure the board does not give is marked with what it has", () => {
  const result = run(page);
  assert.equal(result.status, 1);
  const out = result.stdout.toString();
  assert.match(out, /^Take right from middle until the head is gone\.\n +left +middle +right\n +上 +1 +· +3$/m);
  assert.match(out, /^Gives middle 0 5 1 25 +✗ the board has 0 5 1 24$/m);
  assert.match(out, /1 figure checked, 0 right, 1 wrong\./);
});

test("--plain prints the page, checked, without the boards", () => {
  const result = run(page, ["--plain"]);
  assert.equal(result.status, 1);
  const out = result.stdout.toString();
  assert.doesNotMatch(out, /left +middle +right/);
  assert.match(out, /^Gives middle 0 5 1 25 +✗ the board has 0 5 1 24$/m);
});

test("--check names the file and the line", () => {
  const result = run(page + "Frobnicate 3\n", ["--check"]);
  assert.equal(result.status, 1);
  const lines = result.stdout.toString().trimEnd().split("\n");
  assert.equal(lines[0], `${result.file}:8: wrong: the board has 0 5 1 24`);
  assert.equal(lines[1], `${result.file}:9: notation: \`Frobnicate\` is not an operation: Things, Set, Multiply, Take, Add, Halve, Reduce, Divide, Drop, Gives, Solve, Answer, Rates, Count, Nothing`);
  assert.equal(run(page.replace("25", "24"), ["--check"]).stdout.toString(), "");
});

test("readings: every combination of the readings a page gives", () => {
  const result = run("Things a b 實\nSet right 1 2 3\nSet left 2 3 4\nTake right from left | Take left from right\nGives left 1 1 1 | Gives left 0 -1 -1\n", ["readings"]);
  assert.equal(result.status, 0, result.stderr.toString());
  const out = result.stdout.toString();
  assert.match(out, /^4 readings/);
  assert.match(out, /1 of them agree with every figure the source states/);
  assert.match(out, /line +4: Take right from left\n +line +5: Gives left 1 1 1/);
  if (existsSync(native)) {
    // (not liu-hui-18-old.jz: its readings take a quarter of a minute to run)
    const file = corpus + "liu-hui-18-new.jz";
    const mine = spawnSync(process.execPath, [runner, "readings", file]).stdout.toString();
    assert.equal(mine, spawnSync(native, ["readings", file]).stdout.toString());
  }
});

test("the page is the only file jiuzhang can open, whatever its name", () => {
  // a name that starts with a dash is not taken for an option
  const result = run(page, [], "-odd name.jz");
  assert.equal(result.status, 1, result.stderr.toString());
  assert.match(result.stdout.toString(), /the board has 0 5 1 24/);
  const missing = spawnSync(process.execPath, [runner, join(tmpdir(), "no-such-page.jz")]);
  assert.equal(missing.status, 2);
  assert.match(missing.stderr.toString(), /cannot read/);
});
