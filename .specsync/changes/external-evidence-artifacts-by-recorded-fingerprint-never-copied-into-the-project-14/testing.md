---
change: external-evidence-artifacts-by-recorded-fingerprint-never-copied-into-the-project-14
artifact: testing
---

# Testing

| Requirement | Coverage |
|---|---|
| REQ-cli-006 | tests/evidence.rs `an_external_artifact_is_fingerprinted_in_place`: absolute path fingerprinted, nothing copied into the project, missing external path refused by name |

RED observed with the src change stashed (canonical-spelling refusal); GREEN with it.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-cli-006 (modified) | tests/evidence.rs (`an_external_artifact_is_fingerprinted_in_place`) |
