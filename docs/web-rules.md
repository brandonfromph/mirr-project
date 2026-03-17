# NASA Power-of-10 — Web Edition (W1–W10)

Ten rules for safety-critical web development. Derived from audit of `paper/` and `docs/` sites.

| Rule | Summary |
|------|---------|
| W1 | No JS-gated visibility. CSS content readable without JS. JS enhances via class toggle. |
| W2 | No static imports for optional deps. Use dynamic `import()` in `try/catch`. |
| W3 | Print must override every dark-theme color. WCAG AA (4.5:1) against white. |
| W4 | Negative margins must match parent padding at exact breakpoint. |
| W5 | Every `overflow: auto` must declare scrollbar policy (`scrollbar-width` + `::-webkit-scrollbar`). |
| W6 | Every template variable escaped via `escape_html()` at interpolation site. |
| W7 | Every `@media` breakpoint boundary-tested (±1px). Flex: `flex-wrap`. Grid: `auto-fill/minmax()`. |
| W8 | No dead selectors, no orphaned JS. Zero-debt applies to web code. |
| W9 | Content readable without JS. `<noscript>` for JS-required widgets. |
| W10 | Every `z-index` scoped inside explicit stacking context (`isolation: isolate`). |

## Compliance Record (2026-03-14)

All 10 rules: **PASS**. See `CLAUDE.md` history for per-rule evidence.
