---
change: fix-307
artifact: testing
---

# Testing

`tests/ready.rs::a_path_whose_owning_module_is_not_declared_is_named` fails on
ea159cf, where the report says "Nothing is missing.":

- A draft whose affected paths are two files in the cli spec's `files:` and one
  path no spec lists, with no affected spec, exits 1 and names cli with its two
  paths and `chough change answer <id> affected_specs "cli"`. The unlisted path
  and the records module are not named.
- The same change once approved names `specsync change send-back`.
- With cli declared, the report has no finding and exits 0.
- Each run leaves tracked files and the hash cache unchanged.

`.chough/cases/cli.ready-owning-module.cases.toml` binds the test to REQ-cli-067.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-cli-067 | the test above and its case |
