import * as vscode from "vscode";
import { subscribeToDocumentChanges } from "./diagnostics";
import { channel } from "./logging";

export const activate = async (context: vscode.ExtensionContext) => {
  channel.info("Activate extension");

  const diagnostics =
    vscode.languages.createDiagnosticCollection("reminder-lint");
  context.subscriptions.push(diagnostics);

  await subscribeToDocumentChanges(context, diagnostics);
};
