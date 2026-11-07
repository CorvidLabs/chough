---
id: flag-direct-specsync-lifecycle-writes-in-history-183-drive-21
state: approved
type: feature
base_commit: 0a20d934a08a5c92a14ca6b0016dfb520daa0815
---

# Flag direct SpecSync lifecycle writes in history (183, DRIVE-21)

## Intent

Flag direct SpecSync lifecycle writes in history (183, DRIVE-21)

## Affected Canonical Specs

- `cli`
- `history`

## Acceptance Criteria

- REQ-cli-060 and REQ-history-013 (new) and REQ-cli-057 (modified): a lifecycle record of the active session's change written outside Chough is recorded once as a direct-native-write history event, with its files and states, at the next successful stage move; Chough's own change steps are never flagged because they update an untracked baseline; first sight flags nothing; next and status show pending writes and change nothing; nothing is recorded in the check-to-finalize window; a finding never refuses. The change log names the oldest Chough that reads the event. Seven tests in tests/direct_writes.rs and the modified change-steps test fail first, each mutant in testing.md fails one, and fledge run linux-verify passes.

## No-spec Rationale

Not applicable
