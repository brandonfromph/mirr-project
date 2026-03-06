# MIRR Logo for `.mirr` Files

> **Status:** Draft  
> **Version:** 0.1  
> **Last updated:** 2026-03-01

---

## 1) Canonical logo asset

The canonical `.mirr` visual mark is a **three-snake ouroboros**.

- SVG source (authoritative):
  - `assets/branding/mirr_ouroboros_3_snakes.svg`

Use this file as the source for future PNG/ICO exports.

---

## 2) Design intent

- **Three snakes** represent MIRR’s reflexive cycle and tri-layer flow:
  - source intent
  - temporal semantics
  - compiled artifact
- Circular form signals deterministic, closed-loop behavior.

---

## 3) Recommended use for `.mirr` files

### In docs and web pages

Embed the SVG directly:

```html
<img src="../assets/branding/mirr_ouroboros_3_snakes.svg" alt="MIRR logo" width="64" height="64" />
```

### In VS Code (workspace icon association)

VS Code does not natively map file extensions to custom icons without an icon theme extension.

Two common options:

1. Use an icon theme extension that supports custom associations (recommended).
2. Fork/create a custom icon theme and map `*.mirr` to an icon generated from this SVG.

---

## 4) Suggested export targets

If you need raster/icon variants, export from the SVG at:

- `16x16` (small list views)
- `32x32` (standard)
- `64x64` (high-DPI)
- `256x256` (master PNG)
- `.ico` bundle for Windows shell integrations

---

## 5) Governance

- Any logo redesign should update this document and keep the old version archived.
- Keep SVG source in version control as the single source of truth.
