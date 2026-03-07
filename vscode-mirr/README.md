# MIRR Language Support for VS Code

Syntax highlighting for `.mirr` files — the MIRR hardware-safety DSL.

## Features

- Keyword highlighting (`module`, `guard`, `reflex`, `property`, `def`, `reflect`, etc.)
- Type highlighting (`bool`, `u8`, `u16`, `u32`, `u64`)
- Signal direction keywords (`signal`, `in`, `out`, `internal`)
- Template parameter highlighting (`${param}`)
- Numeric literals
- Line comments (`//`)
- Bracket matching and auto-closing

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
2. Press **F5** to launch an Extension Development Host
3. Open any `.mirr` file to see syntax highlighting
