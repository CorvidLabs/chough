---
change: 0-6-1-the-intent-gate-summary-gives-each-criterion-s-sentence-no-diff-line-without-a-base-and-adds-or-modifies-said-as
artifact: testing
---

# Testing

`tests/gates.rs`: an intent gate with no native change lists each referenced criterion's
ID and sentence and no diff line; a delta that only adds says adds, and one that adds
and modifies names each; the 24-line and 4,096-byte bounds hold and the digest is
stable. Red at the test commit, green at the fix.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-cli-061 | `tests/gates.rs`: the intent gate summary gives each criterion's sentence and no diff line; a delta's requirements are said as added or modified |
