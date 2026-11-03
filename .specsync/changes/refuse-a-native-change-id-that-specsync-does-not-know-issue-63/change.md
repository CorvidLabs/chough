---
id: refuse-a-native-change-id-that-specsync-does-not-know-issue-63
state: implementing
type: bug_fix
base_commit: 4daf861509b70c14282ac16b5d0547d701e385b2
---

# Refuse a native change ID that SpecSync does not know (issue 63)

## Intent

Refuse a native change ID that SpecSync does not know (issue 63)

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- With SpecSync configured chough checkpoint --native-change for an ID that specsync change status does not report exits non-zero with refused:native-change-unknown naming the ID and the status command and .chough/session.json stays byte-identical even when other updates are in the same call, A failed or timed-out SpecSync read also refuses with nothing saved, An ID that SpecSync reports is saved as before, Without a SpecSync configuration only the existing ID format check applies, docs/cli.md states the refusal and the new test fails on main 4daf861 and fledge lanes run verify passes

## No-spec Rationale

Checkpoint native-change validation within REQ-cli traceability as written: the helper already must validate references against native records before recording; no requirement or criterion text changes; docs/cli.md states the refusal
