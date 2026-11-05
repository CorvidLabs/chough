---
change: prepare-the-0-4-0-release-set-the-package-version-to-0-4-0-and-date-the-change-log-section-for-0-4-0
artifact: testing
---

# Testing

- `chough --version` reports 0.4.0.
- `cargo package --list --locked` lists only `src/`, `skills/chough-workflow/`, `Cargo.toml`, `Cargo.lock`, `README.md`, `LICENSE` and the files cargo generates.
- `cargo publish --dry-run --locked` succeeds.
- `fledge lanes run verify` passes on Linux.

These are recorded as observations.
