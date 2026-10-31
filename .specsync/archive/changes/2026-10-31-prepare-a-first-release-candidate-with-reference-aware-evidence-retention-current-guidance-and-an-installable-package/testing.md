---
change: prepare-a-first-release-candidate-with-reference-aware-evidence-retention-current-guidance-and-an-installable-package
artifact: testing
---

# Testing

## Requirement evidence

| Requirement | Test |
|---|---|
| REQ-cli-018 | `tests/evidence.rs` (new file in tests/): preview changes nothing; every protection reason; age and count defaults; apply needs consent or `retention auto`; ledger line; current, historical, pruned and unavailable labels; native read failure keeps sets protected |
| REQ-cli-019 | `tests/cli.rs`: `cargo package --list` contents; `chough init` in an empty project installs no development records; known-outdated skill copies |

Exercises: a disposable copy with repeated validation runs (retention preview, apply
under synthetic consent, protected sets kept, resumed agent sees `pruned`, never
`passed`); a fresh agent context resuming from files; `cargo install --path` into a
temporary root and `chough init` in a new project. Platforms: hosted CI on Ubuntu,
macOS and Windows; local macOS arm64.
