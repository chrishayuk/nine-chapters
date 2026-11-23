// Runs inside VS Code. Exercises the extension the way a person would: open
// a page of the corpus, hover, see its problems, outline and colours, and
// run it.
"use strict";

const assert = require("node:assert/strict");
const path = require("node:path");
const vscode = require("vscode");

const sleep = (ms) => new Promise((resolve) => setTimeout(resolve, ms));

async function until(what, probe, seconds = 20) {
  for (let i = 0; i < seconds * 10; i++) {
    const value = await probe();
    if (value) return value;
    await sleep(100);
  }
  throw new Error(`timed out waiting for ${what}`);
}

const hoverText = (hovers) =>
  hovers.flatMap((h) => h.contents).map((c) => (typeof c === "string" ? c : c.value)).join("\n");

/** Runs a command that starts a jiuzhang task and returns the task's exit code. */
async function taskExit(command) {
  const finished = new Promise((resolve) => {
    const listener = vscode.tasks.onDidEndTaskProcess((event) => {
      if (event.execution.task.definition.type === "jiuzhang") {
        listener.dispose();
        resolve(event.exitCode);
      }
    });
  });
  await vscode.commands.executeCommand(command);
  return Promise.race([finished, sleep(20000).then(() => "timed out")]);
}

exports.run = async function run() {
  const folder = vscode.workspace.workspaceFolders[0].uri.fsPath;
  const document = await vscode.workspace.openTextDocument(path.join(folder, "no-01.jz"));
  await vscode.window.showTextDocument(document);
  assert.equal(document.languageId, "jiuzhang", "the .jz extension selects the language");

  const line = (text) => {
    for (let i = 0; i < document.lineCount; i++) if (document.lineAt(i).text.startsWith(text)) return i;
    throw new Error(`no line ${text}`);
  };

  // The server is ready when it answers a hover.
  const take = new vscode.Position(line("Take right from middle"), 1);
  const hovers = await until("a hover from the language server", async () => {
    const found = await vscode.commands.executeCommand("vscode.executeHoverProvider", document.uri, take);
    return found.length ? found : undefined;
  });
  assert.match(hoverText(hovers), /同名相除/);
  const answer = await vscode.commands.executeCommand("vscode.executeHoverProvider", document.uri, new vscode.Position(line("Answer"), 1));
  assert.match(hoverText(answer), /✓ as the source says/);
  console.log("hover ok");

  const legend = await vscode.commands.executeCommand("vscode.provideDocumentSemanticTokensLegend", document.uri);
  assert.deepEqual(legend.tokenTypes, ["comment", "string", "keyword", "number", "operator", "namespace", "variable", "macro"]);
  const tokens = await vscode.commands.executeCommand("vscode.provideDocumentSemanticTokens", document.uri);
  assert.ok(tokens.data.length > 0);
  const symbols = await vscode.commands.executeCommand("vscode.executeDocumentSymbolProvider", document.uri);
  assert.deepEqual(symbols.map((s) => s.name), ["No. 8.1"]);
  console.log("symbols and semantic tokens ok");

  // A page with a figure the board does not give and a line that is not in the notation.
  const broken = await vscode.workspace.openTextDocument({
    language: "jiuzhang",
    content: "No. 1\nThings a b 實\nSet right 1 2 3\nSet left 2 3 4\nTake right from left\nGives left 1 1 2\nFrobnicate 3\n",
  });
  await vscode.window.showTextDocument(broken);
  const problems = await until("diagnostics", () => {
    const found = vscode.languages.getDiagnostics(broken.uri);
    return found.length >= 2 ? found : undefined;
  });
  const wrong = problems.find((p) => p.severity === vscode.DiagnosticSeverity.Warning);
  assert.match(wrong.message, /the board has 1 1 1/);
  assert.equal(wrong.range.start.line, 5);
  const error = problems.find((p) => p.severity === vscode.DiagnosticSeverity.Error);
  assert.equal(error.source, "jiuzhang");
  assert.equal(error.range.start.line, 6);
  console.log("diagnostics ok");

  const completion = await vscode.commands.executeCommand("vscode.executeCompletionItemProvider", broken.uri, new vscode.Position(7, 0));
  const labels = completion.items.map((i) => (typeof i.label === "string" ? i.label : i.label.label));
  assert.ok(labels.includes("until the head is gone"));
  console.log("completion ok");

  // Run This Page and Check This Page: tasks that end with exit code 0.
  await vscode.window.showTextDocument(document);
  assert.equal(await taskExit("jiuzhang.run"), 0);
  console.log("run ok");
  assert.equal(await taskExit("jiuzhang.check"), 0);
  console.log("check ok");
  assert.equal(await taskExit("jiuzhang.plain"), 0);
  console.log("plain ok");
};
