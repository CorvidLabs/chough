---
change: fix-249-chough-passes-a-relayed-decision-s-words-place-and-recording-agent-to-specsync-s-review-and-approve
artifact: testing
---

# Testing

With a stub `specsync` that records its arguments: closing a scope gate that drives
approval runs `specsync change approve` with `--words`, `--where` and `--recorded-by`
equal to the relay's values; closing a review gate, and `chough change ship` with the
review gate's relay, run `specsync change review` with the same three. Red at the test
commit, green at the fix. The full local suite at the final head.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-records-008 | `tests/humans.rs`: approve and review closes pass the relay as `--words`, `--where` and `--recorded-by` |
| REQ-cli-059 | `tests/change.rs`: the ship's review carries the relay |
