# Security Policy

## Reporting a Vulnerability

If you discover a security vulnerability in the MIRR compiler, the WASM bindings,
or the interactive paper infrastructure, please report it responsibly.

### How to Report

1. **Do NOT open a public GitHub issue** for security vulnerabilities.
2. Email: brandonfromph@users.noreply.github.com
3. Include: description, reproduction steps, affected version, severity assessment.

### What Qualifies

- Compiler bugs that produce incorrect hardware (wrong RTL output)
- WASM sandbox escapes
- Service Worker cache poisoning
- XSS or injection via the interactive paper
- Dependencies with known CVEs

### What Does Not Qualify

- Bugs in example `.mirr` files
- Cosmetic issues in documentation
- Feature requests

### Response Timeline

- Acknowledgment: within 48 hours
- Initial assessment: within 7 days
- Fix or mitigation: best effort, disclosed after fix lands

### Supported Versions

| Version | Supported |
|---------|-----------|
| 0.3.x   | Yes       |
| < 0.3   | No        |

## Disclosure Policy

We follow coordinated disclosure. Security fixes are committed to `main`
with a CVE identifier when applicable. No bounty program exists at this time.
