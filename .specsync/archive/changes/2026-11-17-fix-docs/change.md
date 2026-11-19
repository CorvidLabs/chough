---
id: fix-docs
state: archived
type: bug_fix
base_commit: 0fd9fd229744da039e9d43a5ce1df6dfe5ddc8d7
---

# Fix docs

## Intent

fix docs

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- The drifted sentences match current main, the fast path keeps its frozen steps inside 32 lines, and each changed host template's newest digest is that file's SHA-256.

## No-spec Rationale

FLOW-18 already requires the docs, skill and diagrams to say what current main does. This change corrects drifted sentences and does not change requirement text.
