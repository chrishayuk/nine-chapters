// Talks to the language server the way an editor does: over standard input
// and output. It tests the WebAssembly build the extension ships and, when
// it has been built, the native program other editors would use.
import assert from "node:assert/strict";
import { spawn } from "node:child_process";
import { existsSync } from "node:fs";
import test from "node:test";
import { fileURLToPath } from "node:url";

const here = fileURLToPath(new URL("..", import.meta.url));
const native = fileURLToPath(new URL("../../../target/release/jiuzhang", import.meta.url));
const servers = [["WebAssembly", process.execPath, [`${here}server/lsp.js`, "--stdio"]]];
if (existsSync(native)) servers.push(["native", native, ["lsp"]]);

function connect(command, args) {
  const child = spawn(command, args, { stdio: ["pipe", "pipe", "inherit"] });
  let buffer = Buffer.alloc(0);
  const waiting = [];
  const arrived = [];
  child.stdout.on("data", (chunk) => {
    buffer = Buffer.concat([buffer, chunk]);
    for (;;) {
      const header = buffer.indexOf("\r\n\r\n");
      if (header === -1) return;
      const length = Number(/Content-Length: (\d+)/i.exec(buffer.subarray(0, header).toString())[1]);
      if (buffer.length < header + 4 + length) return;
      const message = JSON.parse(buffer.subarray(header + 4, header + 4 + length).toString());
      buffer = buffer.subarray(header + 4 + length);
      const next = waiting.shift();
      if (next) next(message);
      else arrived.push(message);
    }
  });
  const send = (message) => {
    const body = JSON.stringify({ jsonrpc: "2.0", ...message });
    child.stdin.write(`Content-Length: ${Buffer.byteLength(body)}\r\n\r\n${body}`);
  };
  const receive = () => new Promise((resolve) => (arrived.length ? resolve(arrived.shift()) : waiting.push(resolve)));
  let id = 0;
  const request = async (method, params) => {
    send({ id: ++id, method, params });
    const reply = await receive();
    assert.equal(reply.id, id);
    return reply.result;
  };
  const exited = new Promise((resolve) => child.on("exit", resolve));
  return { send, receive, request, exited, kill: () => child.kill() };
}

const uri = "file:///no-01.jz";
// Problem 8.1 (corpus/no-01.jz, shortened), with a Gives line whose 實 is
// 25 where the board has 24.
const page = `% Problem 8.1, with a figure the board does not give.
No. 8.1
"今有上禾三秉，中禾二秉，下禾一秉，實三十九斗"
Things 上禾 中禾 下禾 實
Set right   3 2 1 39          "置上禾三秉，中禾二秉，下禾一秉，實三十九斗，於右方"
Set middle  2 3 1 34          "中左禾列如右方"
Set left    1 2 3 26
Multiply middle by the head of right.            "以右行上禾徧乘中行"
Take right from middle until the head is gone.   "而以直除"
Gives middle 0 5 1 25
Multiply left by the head of right. | Multiply left by 3   % two readings, the same here
Take right from left until the head is gone.     "亦以直除"
Multiply left by the 中禾 of middle.             "然以中行中禾不盡者，徧乘左行"
Take middle from left until the head is gone.    "而以直除"
Solve.
Answer 9 1/4, 4 1/4, 2 3/4
`;

/** Semantic tokens as [line, column, length, type name]. */
function decode(data, legend) {
  const out = [];
  let line = 0;
  let column = 0;
  for (let i = 0; i < data.length; i += 5) {
    line += data[i];
    column = data[i] ? data[i + 1] : column + data[i + 1];
    out.push([line, column, data[i + 2], legend[data[i + 3]]]);
  }
  return out;
}

for (const [name, command, args] of servers) {
  test(`${name} server: a session from initialize to exit`, async (t) => {
    const server = connect(command, args);
    // A failed assertion must not leave the server running, or the test never ends.
    t.after(() => server.kill());
    const initialised = await server.request("initialize", { processId: null, rootUri: null, capabilities: {} });
    assert.equal(initialised.serverInfo.name, "jiuzhang");
    const capabilities = initialised.capabilities;
    assert.equal(capabilities.textDocumentSync, 1);
    assert.equal(capabilities.hoverProvider, true);
    assert.equal(capabilities.documentSymbolProvider, true);
    assert.ok(capabilities.completionProvider);
    const legend = capabilities.semanticTokensProvider.legend.tokenTypes;
    assert.deepEqual(legend, ["comment", "string", "keyword", "number", "operator", "namespace", "variable", "macro"]);
    server.send({ method: "initialized", params: {} });

    server.send({ method: "textDocument/didOpen", params: { textDocument: { uri, languageId: "jiuzhang", version: 1, text: page } } });
    const opened = await server.receive();
    assert.equal(opened.method, "textDocument/publishDiagnostics");
    assert.equal(opened.params.uri, uri);
    // the 25 is a figure the board does not give: a warning, on its line, with what the board has
    const wrong = opened.params.diagnostics.filter((d) => d.severity === 2);
    assert.equal(wrong.length, 1);
    assert.equal(wrong[0].range.start.line, 9);
    assert.match(wrong[0].message, /the board has 0 5 1 24/);
    assert.equal(wrong[0].source, "jiuzhang");
    assert.deepEqual(opened.params.diagnostics.filter((d) => d.severity === 1), []);

    const at = (line, character) => ({ textDocument: { uri }, position: { line, character } });
    const hover = async (line, character) => (await server.request("textDocument/hover", at(line, character)))?.contents.value;
    assert.match(await hover(3, 1), /names the positions of a column/);
    assert.match(await hover(4, 1), /sets a column \(置\)/);
    assert.match(await hover(4, 1), /> 置上禾三秉/);
    // on Take: what it means, what it did at each position by the rule of signs, and the board after
    const take = await hover(8, 1);
    assert.match(take, /直除/);
    assert.match(take, /take right from middle, 2 times\*\* \(右行 → 中行\)/);
    assert.match(take, /實: 102 → 24 — 同名相除/);
    assert.match(take, /算: plain \d+/);
    assert.match(take, /實 +26 +24 +39/);
    assert.match(await hover(9, 1), /✗ the board has 0 5 1 24/);
    assert.match(await hover(10, 1), /Read in 2 ways; this run takes the first/);
    assert.match(await hover(14, 1), /\d+ steps; 算:/);
    assert.match(await hover(15, 1), /✓ as the source says/);

    const { data } = await server.request("textDocument/semanticTokens/full", { textDocument: { uri } });
    const tokens = decode(data, legend);
    const kinds = (line) => tokens.filter((k) => k[0] === line).map((k) => k[3]);
    assert.deepEqual(kinds(0), ["comment"]);
    assert.deepEqual(kinds(1), ["macro"]);
    assert.deepEqual(kinds(2), ["string"]);
    assert.deepEqual(kinds(3), ["keyword", "variable", "variable", "variable", "variable"]);
    assert.deepEqual(kinds(4), ["keyword", "namespace", "number", "number", "number", "number", "string"]);
    assert.deepEqual(kinds(8), ["keyword", "namespace", "operator", "namespace", "operator", "operator", "operator", "operator", "operator", "string"]);
    // two readings, the | between them, and a note
    assert.deepEqual(kinds(10), ["keyword", "namespace", "operator", "operator", "operator", "operator", "namespace", "operator", "keyword", "namespace", "operator", "number", "comment"]);
    assert.deepEqual(kinds(12), ["keyword", "namespace", "operator", "operator", "variable", "operator", "namespace", "string"]);
    assert.deepEqual(kinds(15), ["keyword", "number", "number", "number"]);
    // the answer's figures: `9 1/4` is one token
    assert.deepEqual(tokens.filter((k) => k[0] === 15 && k[3] === "number").map((k) => k.slice(1, 3)), [[7, 5], [14, 5], [21, 5]]);
    // a quotation after a name of a thing in Chinese characters
    const twelfth = page.split("\n")[12];
    assert.deepEqual(tokens.find((k) => k[0] === 12 && k[3] === "string").slice(1, 3), [twelfth.indexOf('"'), 16]);

    const symbols = await server.request("textDocument/documentSymbol", { textDocument: { uri } });
    assert.deepEqual(symbols.map((s) => s.name), ["No. 8.1"]);
    assert.equal(symbols[0].range.start.line, 1);
    const completion = await server.request("textDocument/completion", at(16, 0));
    const labels = completion.map((c) => c.label);
    for (const label of ["Take", "Solve", "Answer", "right", "右行", "until the head is gone", "crosswise"]) {
      assert.ok(labels.includes(label), label);
    }

    server.send({ method: "textDocument/didChange", params: { textDocument: { uri, version: 2 }, contentChanges: [{ text: "No. 1\nFrobnicate 3.\n" }] } });
    const broken = await server.receive();
    assert.equal(broken.params.diagnostics[0].severity, 1);
    assert.equal(broken.params.diagnostics[0].range.start.line, 1);
    assert.match(broken.params.diagnostics[0].message, /`Frobnicate` is not an operation/);

    server.send({ method: "textDocument/didClose", params: { textDocument: { uri } } });
    const closed = await server.receive();
    assert.deepEqual(closed.params.diagnostics, []);

    assert.equal(await server.request("shutdown", null), null);
    server.send({ method: "exit" });
    assert.equal(await server.exited, 0);
  });
}
