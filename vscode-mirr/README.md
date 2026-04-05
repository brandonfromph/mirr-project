# MIRR Language Support for VS Code

Syntax highlighting and LSP integration for `.mirr` files — the MIRR hardware-safety DSL.

## Features

- **Keyword highlighting**: `module`, `guard`, `reflex`, `property`, `def`, `reflect`, `when`, `for`, `on`, `always`, `never`, `cycles`, `and`, `eventually`, `within`, `followed_by`, `cover`, `assume`, `assert`, `prev`
- **Type highlighting**: `bool`, unsigned integers (`u1`, `u8`, `u16`, `u32`, `u64`, ...), signed integers (`i8`, `i16`, `i32`, `i64`, ...), and arbitrary bit-widths
- **Signal direction keywords**: `signal`, `in`, `out`, `internal`
- **Template parameter highlighting**: `${param}`
- **Numeric literals**
- **Line comments**: `//`
- **Bracket matching and auto-closing**
- **Smart indentation**: automatic indent/outdent on braces

## LSP Integration

The extension includes a Language Server Protocol (LSP) client that connects to the `mirr-lsp` binary. When installed and available on your `PATH`, the language server provides diagnostics and other editor features.

This package does not bundle `mirr-lsp`; it only launches an external binary when one is available.

### Requirements

- The `mirr-lsp` binary must be installed and accessible. Build it from the MIRR compiler workspace:

```bash
cargo build --release
```

The binary will be at `target/release/mirr-lsp` (or `mirr-lsp.exe` on Windows).

## Configuration

| Setting        | Type   | Default      | Description                                      |
|----------------|--------|--------------|--------------------------------------------------|
| `mirr.lspPath` | string | `"mirr-lsp"` | Path to the `mirr-lsp` language server executable |

To override the default, add this to your VS Code `settings.json`:

```json
{
    "mirr.lspPath": "/path/to/mirr-lsp"
}
```

## Installation

### Option A: Copy to extensions directory

```bash
# Linux / macOS
cp -r vscode-mirr ~/.vscode/extensions/mirr-lang

# Windows
xcopy /E vscode-mirr %USERPROFILE%\.vscode\extensions\mirr-lang\
```

Restart VS Code.

### Option B: Development mode

1. Open the `vscode-mirr/` folder in VS Code
2. Run `npm install` to fetch dependencies
3. Run `npm run compile` to build the TypeScript extension
4. Press **F5** to launch an Extension Development Host
5. Open any `.mirr` file to see syntax highlighting and LSP features
