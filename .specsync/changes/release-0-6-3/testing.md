---
change: release-0-6-3
artifact: testing
---

# Testing

No behavior changes, so no new test. `cargo package --list --locked` and
`cargo publish --dry-run --locked` are recorded as dry runs, and
`fledge run release-check` as the index check.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| none changed | the recorded dry runs and the release check |
