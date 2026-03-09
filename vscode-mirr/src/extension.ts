import * as path from 'path';
import { workspace, ExtensionContext } from 'vscode';
import {
    LanguageClient,
    LanguageClientOptions,
    ServerOptions,
} from 'vscode-languageclient/node';

let client: LanguageClient | undefined;

export function activate(context: ExtensionContext): void {
    // Resolve the mirr-lsp binary. Users can override via mirr.lspPath setting.
    const config = workspace.getConfiguration('mirr');
    const lspPath = config.get<string>('lspPath', 'mirr-lsp');

    const serverOptions: ServerOptions = {
        command: lspPath,
        args: [],
    };

    const clientOptions: LanguageClientOptions = {
        documentSelector: [{ scheme: 'file', language: 'mirr' }],
    };

    client = new LanguageClient(
        'mirr-lsp',
        'MIRR Language Server',
        serverOptions,
        clientOptions,
    );

    client.start();
}

export function deactivate(): Thenable<void> | undefined {
    if (!client) {
        return undefined;
    }
    return client.stop();
}
