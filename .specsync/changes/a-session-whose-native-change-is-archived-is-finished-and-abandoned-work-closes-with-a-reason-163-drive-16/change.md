---
id: a-session-whose-native-change-is-archived-is-finished-and-abandoned-work-closes-with-a-reason-163-drive-16
state: implementing
type: feature
base_commit: dab18909368bb3c2d1a4521b4f82efca4ec1e58a
---

# A session whose native change is archived is finished, and abandoned work closes with a reason (163, DRIVE-16)

## Intent

A session whose native change is archived is finished, and abandoned work closes with a reason (163, DRIVE-16)

## Affected Canonical Specs

- `history`
- `cli`

## Acceptance Criteria

- DRIVE-16 (Leif, relayed by orc 2026-11-06, A1 + B for chough#163), in REQ-history-006 and REQ-cli-037: session new and session use archive, bytes unchanged, every other session whose native change is archived on HEAD (finalization.json, no native read) at any stage, keeping one with unfolded inbox entries open; with no pointer such a finished session does not count toward several open sessions, so the only unfinished one is active; chough session close <key> --reason <text> archives an open session for abandoned work with the reason recorded as a decision and no native read, clears the pointer when it named it, and refuses a blank reason, an archived or missing session and unfolded inbox entries with nothing changed. The four new tests in tests/sessions.rs fail first; docs, the skill reference and CHANGELOG say so; fledge lanes run verify passes on Linux.

## No-spec Rationale

Not applicable
