---
change: prepare-the-0-2-0-release-set-the-package-version-to-0-2-0-and-date-the-change-log-section-for-0-2-0
artifact: testing
---

# Testing

- `cargo build --locked` then `./target/debug/chough --version` prints `chough 0.2.0`.
- `fledge lanes run verify` passes (format, lint, the full test suite, HI and SpecSync
  checks), so no test depends on the old version string.
- `cargo package --list --locked` lists only `src/`, `skills/chough-workflow/`,
  `Cargo.toml`, `Cargo.lock`, `README.md` and `LICENSE`, plus cargo's generated files.
- Hosted CI on the pull request: Product on Ubuntu, macOS and Windows, MSRV 1.88.0,
  Fledge/HI/SpecSync and the archive gate.
