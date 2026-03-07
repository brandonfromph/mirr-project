# Migration Guide: MIRR 0.1.0 to 0.2.0

This guide covers breaking and notable changes between MIRR compiler versions
0.1.0 and 0.2.0. It is organized by audience.

---

## For Rust API Consumers

### New enum variants on `PropertyFormula`

`PropertyFormula` now has **6 variants** (was 3). If your code matches on this
enum without a wildcard arm, you must handle the new variants:

| New Variant | MIRR Syntax |
|-------------|-------------|
| `NeverImplies { antecedent, consequent }` | `never (P -> Q)` |
| `EventuallyWithin { expr, cycles }` | `eventually within N (P)` |
| `AlwaysFollowedBy { trigger, response, delay_cycles }` | `always (P followed_by N Q)` |

The `exprs()` and `exprs_mut()` methods on `PropertyFormula` handle all 6
variants, so code using those methods needs no changes.

### New `directive` field on `PropertyDecl`

`PropertyDecl` has a new field:

```rust
pub directive: PropertyDirective,  // Assert (default), Cover, Assume
```

It is annotated with `#[serde(default)]`, so JSON deserialization of old data
still works (defaults to `Assert`).

### New fields on `Module`

Three new fields were added to `Module`:

| Field | Type | Default |
|-------|------|---------|
| `properties` | `Vec<PropertyDecl>` | `[]` (serde default) |
| `pattern_calls` | `Vec<PatternCall>` | `[]` (serde default) |
| `pattern_origins` | `Vec<PatternOrigin>` | `[]` (serde default) |

All three have `#[serde(default)]`, so deserialization of old JSON is
backward-compatible.

### New public types

| Type | Module |
|------|--------|
| `PropertyDirective` | `crate::ast::property` |
| `PropertyFormula` (3 new variants) | `crate::ast::property` |
| `PatternDef` | `crate::ast::pattern` |
| `PatternOrigin` | `crate::ast::pattern` |
| `PatternParam` | `crate::ast::pattern` |

### New public functions

| Function | Purpose |
|----------|---------|
| `expand_patterns()` | Expand `def`/`reflect` pattern calls |
| `emit_firrtl()` | Emit FIRRTL output |
| `emit_sva_only()` | Emit standalone SVA (no Verilog wrapper) |

---

## For JSON Netlist Consumers

### New top-level field: `schema_version`

The JSON netlist now includes a `schema_version` field as the first key:

```json
{
  "schema_version": "0.2.0",
  "ir_version": "1.0",
  ...
}
```

`ir_version` remains `"1.0"` and tracks the IR contract version.
`schema_version` tracks the JSON output schema and will be bumped when
fields are added, removed, or renamed.

**Action required:** If your JSON parser rejects unknown fields, allow
`schema_version` (type: string).

### `properties` array is always present

Previously, the `properties` key was absent when no properties existed.
Now it is always present as an array (may be empty `[]`).

### Property objects have new fields and values

Each object in the `properties` array now has:

| Field | New? | Values |
|-------|------|--------|
| `name` | No | Property name string |
| `directive` | **Yes** | `"assert"`, `"cover"`, or `"assume"` |
| `kind` | No (3 new values) | `"always"`, `"never"`, `"always_implies"`, `"never_implies"`, `"eventually_within"`, `"always_followed_by"` |
| `formula_text` | No | Human-readable formula string |

### `pattern_origins` array (conditional)

When a module uses pattern calls, a `pattern_origins` array appears:

```json
"pattern_origins": [
  { "pattern_name": "threshold_guard", "args_summary": "temperature, 100, 5, temp_alarm" }
]
```

This array is omitted (not present) when empty.

---

## For MIRR Syntax / Tool Authors

### New keywords in `property` blocks

Five new keywords are recognized inside `property` blocks:

| Keyword | Context |
|---------|---------|
| `cover` | Directive prefix: `cover eventually within N (P)` |
| `assume` | Directive prefix: `assume always (P)` |
| `eventually` | Formula keyword: `eventually within N (P)` |
| `within` | Used with `eventually`: `eventually within N (P)` |
| `followed_by` | Used in implication: `always (P followed_by N Q)` |

These keywords only have special meaning inside `property {}` blocks.
They do **not** affect parsing of `signal`, `guard`, or `reflex` blocks.

### New top-level keywords: `def` and `reflect`

The pattern system introduces two new top-level constructs:

```
def pattern_name(param1: type, ...) {
    reflect {
        guard ... { ... }
        reflex ... { ... }
        property ... { ... }
    }
}
```

Pattern calls appear at module scope: `pattern_name(arg1, arg2, ...);`

---

## For Error Message Parsers

All error messages now start with a structured error code prefix:

| Prefix | Category | Example |
|--------|----------|---------|
| `[E100]` | Parse errors | `[E100] Parse error: Unbalanced parentheses in expression.` |
| `[E200]` | Semantic errors | `[E200] Semantic error: Duplicate signal name: 'x'.` |
| `[E300]` | Temporal errors | `[E300] Temporal compilation error: ...` |
| `[E400]` | Pattern errors | `[E400] Pattern error: ...` |

If you parse error output, you can now match on the `[Ennn]` prefix to
categorize errors programmatically.
