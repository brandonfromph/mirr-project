# Contributing to the LRA Standard

Thank you for your interest in the Living Research Artifact standard.

## How to Contribute

### Reporting Issues

- Open an issue for bugs in the template (HTML/CSS/JS)
- Open an issue for unclear or missing spec language in `spec/LRA-1.0.md`
- Tag issues with `template` or `spec` as appropriate

### Proposing Changes

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Open a Pull Request with a clear description

### Proposing Spec Changes

Changes to `spec/LRA-1.0.md` require:

- A clear rationale (why the current spec is insufficient)
- Backward compatibility analysis (will existing LRAs break?)
- At least one example demonstrating the need

Minor clarifications can go directly to a PR. Substantive changes
(new tiers, new required sections, license changes) should be
discussed in an issue first.

## Style Guide

- HTML: semantic elements, no `<div>` soup
- CSS: use CSS custom properties (variables), no frameworks
- JS: vanilla ES modules, no npm dependencies, no CDN
- Markdown: ATX headers, line length under 80 chars

## Code of Conduct

Be respectful. Be constructive. Focus on the work.
