// The Nine Chapters extension: starts the language server (`jiuzhang lsp`)
// and adds commands that run the page in the editor on a counting board and
// show every figure the source states checked.
"use strict";

const vscode = require("vscode");
const fs = require("fs");
const os = require("os");
const path = require("path");
const { LanguageClient, TransportKind } = require("vscode-languageclient/node");

/** @type {LanguageClient | undefined} */
let client;

function setting(name) {
  const value = vscode.workspace.getConfiguration("jiuzhang").get(name);
  return typeof value === "string" ? value.trim() : value;
}

async function startServer(context) {
  const custom = setting("path");
  // Built in: jiuzhang compiled to WebAssembly, run by the editor's own Node,
  // so nothing has to be installed.
  const builtIn = { module: context.asAbsolutePath(path.join("server", "lsp.js")), transport: TransportKind.stdio };
  const serverOptions = custom ? { command: custom, args: ["lsp"] } : { run: builtIn, debug: builtIn };
  client = new LanguageClient("jiuzhang", "Nine Chapters (jiuzhang)", serverOptions, {
    documentSelector: [{ language: "jiuzhang" }],
  });
  await client.start();
}

async function restartServer(context) {
  if (client) await client.stop().catch(() => {});
  client = undefined;
  await startServer(context);
}

async function runPage(context, options, title) {
  const editor = vscode.window.activeTextEditor;
  if (!editor || editor.document.languageId !== "jiuzhang") {
    vscode.window.showInformationMessage("Open a page of the Nine Chapters (.jz) to run it.");
    return;
  }
  const document = editor.document;
  let file = document.uri.fsPath;
  if (document.isUntitled || document.uri.scheme !== "file") {
    // Not on disk yet: run a copy.
    file = path.join(os.tmpdir(), "untitled.jz");
    fs.writeFileSync(file, document.getText());
  } else if (document.isDirty) {
    await document.save();
  }

  const custom = setting("path");
  const execution = custom
    ? new vscode.ProcessExecution(custom, [...options, file])
    : new vscode.ProcessExecution(process.execPath, [context.asAbsolutePath(path.join("server", "run.js")), ...options, file], {
        env: { ELECTRON_RUN_AS_NODE: "1" },
      });
  const task = new vscode.Task({ type: "jiuzhang" }, vscode.TaskScope.Workspace, `${title} ${path.basename(file)}`, "jiuzhang", execution, []);
  task.presentationOptions = {
    reveal: vscode.TaskRevealKind.Always,
    panel: vscode.TaskPanelKind.Dedicated,
    focus: true,
    clear: true,
    // Without this the terminal first prints the whole command line.
    echo: false,
    showReuseMessage: false,
  };
  await vscode.tasks.executeTask(task);
}

async function activate(context) {
  context.subscriptions.push(
    vscode.commands.registerCommand("jiuzhang.run", () => runPage(context, [], "Run")),
    vscode.commands.registerCommand("jiuzhang.plain", () => runPage(context, ["--plain"], "Page")),
    vscode.commands.registerCommand("jiuzhang.check", () => runPage(context, ["--check"], "Check")),
    vscode.commands.registerCommand("jiuzhang.readings", () => runPage(context, ["readings"], "Readings")),
    vscode.commands.registerCommand("jiuzhang.restartServer", () => restartServer(context)),
    vscode.workspace.onDidChangeConfiguration((event) => {
      if (event.affectsConfiguration("jiuzhang.path")) restartServer(context);
    })
  );
  await startServer(context);
}

function deactivate() {
  return client ? client.stop() : undefined;
}

module.exports = { activate, deactivate };
