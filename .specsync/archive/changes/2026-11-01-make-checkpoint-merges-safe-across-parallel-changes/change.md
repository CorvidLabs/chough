---
id: make-checkpoint-merges-safe-across-parallel-changes
state: archived
type: feature
base_commit: b35acaf0804f84a2c56fcee4b3ca1504cf74c95e
---

# Make checkpoint merges safe across parallel changes

## Intent

Make checkpoint merges safe across parallel changes

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- `chough checkpoint --merge <base> <ours> <theirs>` performs a bounded three-way merge of schema 2 checkpoints. Exact-string history and reference collections preserve base order plus ours-only and theirs-only additions, identical additions appear once, and one-side removals are not resurrected.
- Objective, intent scope, modules, stage, native change, next action, run consents, blockers and awaiting-human gates merge as one current-cycle group: an unchanged or identical side is selected, while conflicting changes refuse with a named reason. Every refusal leaves all inputs byte-identical and creates no transition, gate, approval, review or consent record.
- Schema 1, malformed input, unknown fields, mixed versions and output beyond existing bounds fail closed. The canonical checkpoint remains .chough/session.json with no second writable store, runtime, service or duplicate lifecycle state.
- A .gitattributes declaration and documented per-clone Git driver invoke the same command without changing Git configuration during init. A real Git fixture proves compatible merges and unresolved current-cycle conflicts. The explicit command remains the fallback when no driver is available.
- Failure-first evidence is retained for the current conflicting checkpoint behavior and linked to a local bug before implementation, followed by a later pass. Existing schema 1 read and migration behavior, one-owner live writes, native SpecSync authority and all non-checkpoint ID schemes remain unchanged.
- Focused and full Rust tests, real Git merge tests, formatting, Clippy with warnings denied, fledge verify and pre-push pass with the current CI tool versions. No new case number is allocated; evidence uses explicit readable IDs. No release or deployment is introduced.

## No-spec Rationale

Not applicable
