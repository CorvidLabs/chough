---
change: the-knowledge-index-and-feed-are-rebuilt-and-checked-at-release-not-at-each-move-into-done-or-in-each-pull-request-289
artifact: testing
---

# Testing

| Requirement | Evidence |
|---|---|
| REQ-cli-043 | tests/journal.rs: the move into done reports `deferred` and leaves the index and feed byte-identical |
| REQ-cli-047 | tests/vault.rs: a hand-edited linked bullet fails `--check`; a test that the verify lane runs no `--check` and `release-check` does |
| REQ-cli-052 | the existing one-read-into-done test |
| REQ-records-011 | tests/journal.rs: the feed lists the post after `library index` |
| REQ-history-006 | the existing archive-at-done tests |
