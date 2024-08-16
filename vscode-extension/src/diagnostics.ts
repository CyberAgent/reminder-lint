import * as vscode from "vscode";
import path from "node:path";
import { channel } from "./logging";
import { execFile } from "./util";
import type { ListCommandOutput, Reminder } from "./types";

const reminderListByFile = new Map<string, Reminder[]>();

// TODO: doc.fileNameを引数に渡して、対象のファイルのみを検査する
const collectReminders = async (
  context: vscode.ExtensionContext,
  dir: string,
) => {
  const cliPath = path.join(
    context.extensionPath,
    "../target/debug",
    "reminder-lint",
  );
  channel.info(`Run: ${cliPath} list --json (cwd: ${dir})`);

  const rawResult = await execFile(cliPath, ["list", "--json"], { cwd: dir });
  const result = JSON.parse(rawResult.stdout) as ListCommandOutput;
  channel.info("Reminder List:", result);

  for (const reminder of [...result.expired, ...result.upcoming]) {
    const key = path.join(dir, reminder.position.file);

    const reminders = reminderListByFile.get(key) ?? [];
    reminders.push(reminder);

    reminderListByFile.set(key, reminders);
  }
};

const refreshDiagnostics = async (
  doc: vscode.TextDocument,
  collection: vscode.DiagnosticCollection,
): Promise<void> => {
  channel.info("Refresh diagnostics", doc.fileName);
  collection.delete(doc.uri);

  const reminders = reminderListByFile.get(doc.fileName) ?? [];
  const diagnostics: vscode.Diagnostic[] = reminders
    .map((reminder) => {
      const lineOfText = doc.lineAt(reminder.position.line - 1);

      if (reminder.datetime < Date.now() / 1000) {
        return createDiagnostic(lineOfText);
      }

      return null;
    })
    .filter(
      (diagnostic): diagnostic is vscode.Diagnostic => diagnostic != null,
    );

  collection.set(doc.uri, diagnostics);
};

const createDiagnostic = (lineOfText: vscode.TextLine): vscode.Diagnostic => {
  const diagnostic = new vscode.Diagnostic(
    lineOfText.range,
    "This reminder is expired.",
    vscode.DiagnosticSeverity.Error,
  );

  return diagnostic;
};

export const subscribeToDocumentChanges = async (
  context: vscode.ExtensionContext,
  collection: vscode.DiagnosticCollection,
): Promise<void> => {
  channel.info("subscribeToDocumentChanges");
  await collectReminders(
    context,
    vscode.workspace.workspaceFolders?.[0].uri.fsPath ?? "",
  );

  if (vscode.window.activeTextEditor != null) {
    await refreshDiagnostics(
      vscode.window.activeTextEditor.document,
      collection,
    );
  }

  context.subscriptions.push(
    vscode.window.onDidChangeActiveTextEditor((editor) => {
      channel.info("onDidChangeActiveTextEditor");
      if (editor != null) {
        refreshDiagnostics(editor.document, collection);
      }
    }),
  );

  // TODO: 標準入力に対して検査できるようになったら、onDidChangeTextDocumentも使いたいかも
  context.subscriptions.push(
    vscode.workspace.onDidSaveTextDocument(async (doc) => {
      channel.info("onDidSaveTextDocument", doc.uri.toString());
      await collectReminders(
        context,
        vscode.workspace.workspaceFolders?.[0].uri.fsPath ?? "",
      );
      await refreshDiagnostics(doc, collection);
    }),
  );

  context.subscriptions.push(
    vscode.workspace.onDidCloseTextDocument((doc) =>
      collection.delete(doc.uri),
    ),
  );
};
