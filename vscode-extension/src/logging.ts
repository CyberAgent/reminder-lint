import * as vscode from "vscode";

export const channel = vscode.window.createOutputChannel("reminder-lint", {
  log: true,
});
