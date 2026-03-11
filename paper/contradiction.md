# Contradiction — A Specification for the Living Research Artifact

**Version:** 0.1.0
**License:** GPL-3.0 (same as the artifact it describes)
**Status:** Active

---

## 1. The Contradiction

An academic paper is a PDF. It describes software but cannot run it. It makes claims
but cannot verify them. It freezes knowledge at submission time and watches it rot.

A Living Research Artifact is different. The paper, the code, the proofs, and the demos
are one GPL-licensed Git repository, citable by commit hash.

This specification defines what a Living Research Artifact must be.

---

## 2. Core Properties

### 2.1 Unity

The artifact is one repository. The paper does not *describe* the compiler — it *is*
the compiler. Source code, formal proofs, documentation, and the interactive paper
coexist in the same version-controlled tree.

### 2.2 Executability

Every claim in the paper has a corresponding executable demo. The reader does not trust
screenshots — they run the compiler in their browser via WebAssembly.

### 2.3 Citability

The artifact is citable by commit hash. A `CITATION.cff` file provides machine-readable
metadata. The version is the compiler version. The DOI (if obtained) points to a
specific commit.

### 2.4 Openness

The artifact is GPL-3.0. Anyone can fork it, reproduce results, and extend the work.
The license applies to all components: code, proofs, paper, and demos.

### 2.5 Liveness

The artifact evolves. Each commit is a new citable version. The paper stays current
with the implementation because they are the same thing. Stale claims are impossible
when the claims are executable.

---

## 3. Architecture

```
repository/
├── src/           # Compiler source (the artifact itself)
├── tests/         # Test suite (executable specification)
├── proofs/        # Formal proofs (Rocq/Coq)
├── paper/         # Interactive paper (HTML + CSS + JS)
│   ├── index.html # The paper
│   ├── paper.css  # Typographic styles
│   └── paper.js   # Demo controller (zero dependencies)
├── crates/
│   └── mirr-wasm/ # WASM bindings (separate crate)
├── demos/         # CI-built WASM artifacts (gitignored)
├── CITATION.cff   # Machine-readable citation
└── docs/          # User-facing documentation
```

---

## 4. Constraints

### 4.1 Zero External Dependencies (Paper)

The interactive paper (`paper/index.html`) loads no external CSS, JS, or fonts.
System font stacks only. The WASM module is the only non-inline dependency, loaded
from the same repository's build artifacts.

### 4.2 Bounded Input

The WASM API enforces `MAX_SOURCE_BYTES = 65536` to prevent browser tab freezes.
This is a NASA Power-of-10 constraint applied to the web layer.

### 4.3 JSON Protocol

All WASM functions return JSON: `{"ok": "..."}` or `{"err": "..."}`. No exceptions
are thrown across the WASM boundary. All errors are values.

### 4.4 Separate Crate

The WASM crate is a workspace member, not an inline module. The main compiler crate
is never modified for WASM concerns. The WASM crate is a consumer, not a modifier.

### 4.5 CI-Built Artifacts

WASM binaries are built in CI and deployed to GitHub Pages. They are gitignored in
the main branch. The repository is reproducible: anyone with `wasm-pack` can rebuild.

---

## 5. Claims and Evidence

Each claim in the paper follows this structure:

```
Claim N: [Statement]
  ↓
Demo N: [Executable evidence]
  ↓
Back-reference: [Link from demo back to claim]
```

Every claim must have a demo. Every demo must link back to its claim. Claims without
executable evidence are not claims — they are aspirations.

---

## 6. What This Is Not

- **Not a framework.** This is a specification for one project (MIRR). Others may
  adopt the pattern, but this document does not prescribe how.
- **Not a build system.** The specification is agnostic to `wasm-pack` vs `wasm-bindgen-cli`
  vs any other toolchain.
- **Not a hosting requirement.** GitHub Pages is convenient but not required. The paper
  works from `file://` with pre-built WASM.

---

## 7. Success Criteria

The Living Research Artifact succeeds when:

1. A reviewer can open the paper in a browser and compile MIRR source without
   installing anything.
2. Every claim in the paper has a working demo.
3. The repository compiles, tests pass, proofs check, and the paper renders —
   all from one `git clone`.
4. The artifact is citable by commit hash via `CITATION.cff`.
5. The GPL-3.0 license protects both the code and the paper.
