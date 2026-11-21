---
change: release-0-6-2
artifact: testing
---

# Testing

No behavior changes, so no new test. The verify lane runs on the release branch.
`cargo package --list --locked` and `cargo publish --dry-run --locked` are recorded as
dry runs, and `fledge run release-check` as the index check.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| none changed | the verify lane and the recorded dry runs |
