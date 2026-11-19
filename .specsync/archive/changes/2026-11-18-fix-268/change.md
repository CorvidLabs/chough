---
id: fix-268
state: archived
type: bug_fix
base_commit: be6c9f126e448bd33945a52261d55db2a9677d3d
---

# Fix 268

## Intent

fix 268

## Affected Canonical Specs

- `records`
- `cli`

## Acceptance Criteria

- A token form in a chough checkpoint --file or --design value, such as src/ghp_<token>.txt, refuses the write as refused:leak naming the field (file or design) and the kind token, never echoing the value, with every file unchanged; with --redact the stored path holds <redacted:token> and the command says it redacted. A file or design path that holds no leak is saved as before.

## No-spec Rationale

REQ-records-006 already requires every value Chough writes, front matter included, to pass the leak check; only the checkpoint guard and its tests change.
