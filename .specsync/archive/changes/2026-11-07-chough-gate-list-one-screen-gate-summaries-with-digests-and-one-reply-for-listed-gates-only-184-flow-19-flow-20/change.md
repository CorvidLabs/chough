---
id: chough-gate-list-one-screen-gate-summaries-with-digests-and-one-reply-for-listed-gates-only-184-flow-19-flow-20
state: archived
type: feature
base_commit: 0a20d934a08a5c92a14ca6b0016dfb520daa0815
---

# Chough gate list: one-screen gate summaries with digests, and one reply for listed gates only (184, FLOW-19, FLOW-20)

## Intent

chough gate list: one-screen gate summaries with digests, and one reply for listed gates only (184, FLOW-19, FLOW-20)

## Affected Canonical Specs

- `cli`
- `history`

## Acceptance Criteria

- REQ-cli-061 and REQ-history-014 (new), REQ-cli-059 (modified): chough gate list numbers every open human gate with a one-screen summary (what changed since the change's base, the tests, the risks, a diff link) and its SHA-256 digest, saved untracked, with no native read; chough gate record closes only listed gates with one relay, each keeping its own summary digest and the list, all or none, refusing an unknown list, an unlisted number, a closed gate, a changed summary, or a verifying change (each naming a recovery a test runs); chough change ship takes --list and --gate for the review gate. The change log names the oldest Chough that reads summary digests. Six tests in tests/gates.rs fail first, each mutant in testing.md fails one, and fledge run linux-verify passes.

## No-spec Rationale

Not applicable
