// Tokenises pages with the same TextMate engine VS Code uses and checks the
// scopes the grammar assigns.
import assert from "node:assert/strict";
import { readFileSync, readdirSync } from "node:fs";
import { createRequire } from "node:module";
import test from "node:test";
import { fileURLToPath } from "node:url";
import oniguruma from "vscode-oniguruma";
import textmate from "vscode-textmate";

const require = createRequire(import.meta.url);
const grammarPath = fileURLToPath(new URL("../syntaxes/jiuzhang.tmLanguage.json", import.meta.url));
const corpus = fileURLToPath(new URL("../../../corpus/", import.meta.url));

const wasm = readFileSync(require.resolve("vscode-oniguruma/release/onig.wasm"));
await oniguruma.loadWASM(wasm.buffer.slice(wasm.byteOffset, wasm.byteOffset + wasm.byteLength));
const registry = new textmate.Registry({
  onigLib: Promise.resolve({
    createOnigScanner: (patterns) => new oniguruma.OnigScanner(patterns),
    createOnigString: (text) => new oniguruma.OnigString(text),
  }),
  loadGrammar: async () => textmate.parseRawGrammar(readFileSync(grammarPath, "utf8"), grammarPath),
});
const grammar = await registry.loadGrammar("source.jz");

/** Each non-blank piece of text with its most specific scope. */
function scopes(source) {
  const out = [];
  let state = textmate.INITIAL;
  for (const line of source.split("\n")) {
    const result = grammar.tokenizeLine(line, state);
    for (const token of result.tokens) {
      const text = line.slice(token.startIndex, token.endIndex);
      if (text.trim()) out.push([text.trim(), token.scopes.at(-1)]);
    }
    state = result.ruleStack;
  }
  return out;
}

/** The scope of the piece of text `text` on its own. */
const scopeOf = (line, text) => {
  const found = scopes(line).find(([t]) => t === text);
  assert.ok(found, `no token ${text} in ${line}`);
  return found[1];
};

test("headings and notes", () => {
  assert.deepEqual(scopes("No. 8.1"), [["No. 8.1", "entity.name.section.problem.jiuzhang"]]);
  assert.deepEqual(scopes("No. 8.18 (Liu Hui: the old method made short, 77 算)"), [
    ["No. 8.18 (Liu Hui: the old method made short, 77 算)", "entity.name.section.problem.jiuzhang"],
  ]);
  assert.deepEqual(scopes("% in 斗"), [["% in 斗", "comment.line.percentage.jiuzhang"]]);
  assert.deepEqual(scopes("Take 3rd from 4th   % corrupt?").at(-1), ["% corrupt?", "comment.line.percentage.jiuzhang"]);
});

test("the source's words: a line of its own, and at the end of an operation", () => {
  assert.deepEqual(scopes('"今有上禾三秉，中禾二秉"'), [
    ['"', "punctuation.definition.string.begin.jiuzhang"],
    ["今有上禾三秉，中禾二秉", "string.quoted.double.jiuzhang"],
    ['"', "punctuation.definition.string.end.jiuzhang"],
  ]);
  assert.deepEqual(scopes('Set right   3 2 1 39          "置上禾三秉，於右方"').slice(-3), [
    ['"', "punctuation.definition.string.begin.jiuzhang"],
    ["置上禾三秉，於右方", "string.quoted.double.jiuzhang"],
    ['"', "punctuation.definition.string.end.jiuzhang"],
  ]);
  // numbers and column names in the quotation are not the operation's
  assert.equal(scopeOf('Halve 2nd.   "餘可半 right 2"', "餘可半 right 2"), "string.quoted.double.jiuzhang");
  assert.equal(scopeOf("Halve 2nd.   「餘可半」", "餘可半"), "string.quoted.other.corner.jiuzhang");
});

test("the operations", () => {
  const words = ["Things", "Set", "Multiply", "Take", "Add", "Halve", "Reduce", "Divide", "Drop", "Gives", "Solve", "Answer", "Rates", "Count", "Nothing"];
  for (const w of words) assert.equal(scopes(w)[0][1], "keyword.control.operation.jiuzhang", w);
  assert.deepEqual(scopes("Set right   3 2 1 39"), [
    ["Set", "keyword.control.operation.jiuzhang"],
    ["right", "entity.name.type.column.jiuzhang"],
    ["3", "constant.numeric.whole.jiuzhang"],
    ["2", "constant.numeric.whole.jiuzhang"],
    ["1", "constant.numeric.whole.jiuzhang"],
    ["39", "constant.numeric.whole.jiuzhang"],
  ]);
  assert.deepEqual(scopes("Things 上禾 中禾 下禾 實"), [
    ["Things", "keyword.control.operation.jiuzhang"],
    ["上禾", "variable.other.thing.jiuzhang"],
    ["中禾", "variable.other.thing.jiuzhang"],
    ["下禾", "variable.other.thing.jiuzhang"],
    ["實", "variable.other.thing.jiuzhang"],
  ]);
  assert.deepEqual(scopes("Multiply left by the 中禾 of middle."), [
    ["Multiply", "keyword.control.operation.jiuzhang"],
    ["left", "entity.name.type.column.jiuzhang"],
    ["by", "keyword.other.jiuzhang"],
    ["the", "keyword.other.jiuzhang"],
    ["中禾", "variable.other.thing.jiuzhang"],
    ["of", "keyword.other.jiuzhang"],
    ["middle", "entity.name.type.column.jiuzhang"],
    [".", "punctuation.terminator.statement.jiuzhang"],
  ]);
  assert.deepEqual(scopes("Divide middle by 3"), [
    ["Divide", "keyword.control.operation.jiuzhang"],
    ["middle", "entity.name.type.column.jiuzhang"],
    ["by", "keyword.other.jiuzhang"],
    ["3", "constant.numeric.whole.jiuzhang"],
  ]);
  assert.deepEqual(scopes("Solve by reducing."), [
    ["Solve", "keyword.control.operation.jiuzhang"],
    ["by", "keyword.other.jiuzhang"],
    ["reducing", "support.constant.method.jiuzhang"],
    [".", "punctuation.terminator.statement.jiuzhang"],
  ]);
});

test("how a column is taken from another", () => {
  assert.deepEqual(scopes("Take right from middle until the head is gone."), [
    ["Take", "keyword.control.operation.jiuzhang"],
    ["right", "entity.name.type.column.jiuzhang"],
    ["from", "keyword.other.jiuzhang"],
    ["middle", "entity.name.type.column.jiuzhang"],
    ["until the head is gone", "keyword.other.how.jiuzhang"],
    [".", "punctuation.terminator.statement.jiuzhang"],
  ]);
  assert.equal(scopeOf("Take right from middle as far as it goes", "as far as it goes"), "keyword.other.how.jiuzhang");
  assert.equal(scopeOf("Take right from middle crosswise", "crosswise"), "keyword.other.how.jiuzhang");
  assert.equal(scopeOf("Take 2nd from 3rd in turn", "in turn"), "keyword.other.how.jiuzhang");
  assert.deepEqual(scopes("Take right from middle as far as the 菽 goes").slice(4), [
    ["as", "keyword.other.jiuzhang"],
    ["far", "keyword.other.jiuzhang"],
    ["as", "keyword.other.jiuzhang"],
    ["the", "keyword.other.jiuzhang"],
    ["菽", "variable.other.thing.jiuzhang"],
    ["goes", "keyword.other.jiuzhang"],
  ]);
});

test("the columns, in English and in the text's own names", () => {
  for (const c of ["right", "middle", "left", "2nd", "3rd", "4th", "11th", "右行", "中行", "左行", "第二行", "第九行"]) {
    assert.equal(scopeOf(`Drop ${c}`, c), "entity.name.type.column.jiuzhang", c);
  }
  assert.deepEqual(scopes("Take 第三行 from 左行"), [
    ["Take", "keyword.control.operation.jiuzhang"],
    ["第三行", "entity.name.type.column.jiuzhang"],
    ["from", "keyword.other.jiuzhang"],
    ["左行", "entity.name.type.column.jiuzhang"],
  ]);
});

test("figures: negative, empty, and answers with fractions", () => {
  assert.deepEqual(scopes("Gives 4th 0 0 0 0 -1 −6 ·").slice(2), [
    ["0", "constant.numeric.whole.jiuzhang"],
    ["0", "constant.numeric.whole.jiuzhang"],
    ["0", "constant.numeric.whole.jiuzhang"],
    ["0", "constant.numeric.whole.jiuzhang"],
    ["-1", "constant.numeric.whole.jiuzhang"],
    ["−6", "constant.numeric.whole.jiuzhang"],
    ["·", "constant.numeric.empty.jiuzhang"],
  ]);
  assert.deepEqual(scopes("Answer 9 1/4, 1 18/52, 1/2, 3"), [
    ["Answer", "keyword.control.operation.jiuzhang"],
    ["9 1/4", "constant.numeric.fraction.jiuzhang"],
    [",", "punctuation.separator.comma.jiuzhang"],
    ["1 18/52", "constant.numeric.fraction.jiuzhang"],
    [",", "punctuation.separator.comma.jiuzhang"],
    ["1/2", "constant.numeric.fraction.jiuzhang"],
    [",", "punctuation.separator.comma.jiuzhang"],
    ["3", "constant.numeric.whole.jiuzhang"],
  ]);
  assert.equal(scopeOf("Count 77", "77"), "constant.numeric.whole.jiuzhang");
  assert.equal(scopeOf("Rates 721 265 191", "721"), "constant.numeric.whole.jiuzhang");
});

test("readings in the alternative", () => {
  assert.deepEqual(scopes("Take 3rd from 4th | Nothing | Take 4th from 3rd crosswise"), [
    ["Take", "keyword.control.operation.jiuzhang"],
    ["3rd", "entity.name.type.column.jiuzhang"],
    ["from", "keyword.other.jiuzhang"],
    ["4th", "entity.name.type.column.jiuzhang"],
    ["|", "keyword.operator.reading.jiuzhang"],
    ["Nothing", "keyword.control.operation.jiuzhang"],
    ["|", "keyword.operator.reading.jiuzhang"],
    ["Take", "keyword.control.operation.jiuzhang"],
    ["4th", "entity.name.type.column.jiuzhang"],
    ["from", "keyword.other.jiuzhang"],
    ["3rd", "entity.name.type.column.jiuzhang"],
    ["crosswise", "keyword.other.how.jiuzhang"],
  ]);
});

test("a word that is not an operation", () => {
  assert.deepEqual(scopes("Frobnicate 3")[0], ["Frobnicate", "invalid.illegal.jiuzhang"]);
  assert.deepEqual(scopes("Take 3rd from 4th | Frob")[5], ["Frob", "invalid.illegal.jiuzhang"]);
  assert.deepEqual(scopes("Setright 3")[0], ["Setright", "invalid.illegal.jiuzhang"]);
});

test("every page of the corpus tokenises with nothing marked invalid", () => {
  const files = readdirSync(corpus).filter((f) => f.endsWith(".jz"));
  assert.ok(files.length >= 26);
  for (const file of files) {
    const bad = scopes(readFileSync(corpus + file, "utf8")).filter(([, k]) => k === "invalid.illegal.jiuzhang");
    assert.deepEqual(bad, [], file);
  }
});
