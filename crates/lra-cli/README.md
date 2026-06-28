# lra — Living Research Artifact CLI

Validate, scaffold, and serve interactive research papers that conform to the
[LRA-1.0 standard](https://github.com/brandonfromph/mirr-project/blob/main/template/spec/LRA-1.0.md).

## Install

```bash
cargo install lra-cli
```

## Commands

```
lra init my-paper        # Scaffold a new LRA project
lra validate             # Check LRA-1.0 compliance (Bronze/Silver/Gold)
lra serve                # Dev server with live reload
lra badge                # Print shields.io badge URL for detected tier
lra build paper.md       # Compile Markdown to LRA-compliant HTML
```

## Validation tiers

| Tier | Requirements |
|------|-------------|
| Bronze | HTML structure, metadata, LICENSE, CITATION.cff (14 checks) |
| Silver | + demo section, noscript fallback, paper.js (3 checks) |
| Gold | + WASM module, evidence links, no external fetch, aria-live (4 checks) |

## License

GPL-3.0-or-later — see [LICENSE](../../LICENSE) for terms.
