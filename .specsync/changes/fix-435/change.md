---
id: fix-435
state: implementing
type: feature
base_commit: 4a46668220663eb00524cb6990a325b0713eb099
---

# Fix 435

## Intent

fix 435

## Affected Canonical Specs

- `records`
- `history`
- `cli`

## Acceptance Criteria

- On a volume without hard links (exFAT), chough inbox add writes its entry whole or not at all and succeeds, as init, bug add and journal add do since #260 and #424; the ignored test tests/no_hard_links.rs covers it.

## No-spec Rationale

publish_entry switches to the existing whole-or-not-at-all writer write_new_bytes (REQ-records-001, #313); no contract text changes
