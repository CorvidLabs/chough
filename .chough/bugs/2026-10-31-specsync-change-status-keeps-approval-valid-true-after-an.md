---
kind: bug
id: 2026-10-31-specsync-change-status-keeps-approval-valid-true-after-an
aliases: [BUG-0002]
title: SpecSync change status keeps approval_valid true after an approved delta is edited
status: open
classification: environment
scope: out-of-scope
impact: An agent trusting status could believe an edited contract is still approved; Chough relies on change check and records the lesson
criteria: [LIFE-7]
requirements: [REQ-cli-008]
cases: []
evidence: []
receipts: []
environment: specsync-6.1.1-macos-arm64
revision: 12c8d4009995e13a42a354e4bc59f641ad9d0dfb
found_by: SpecSync rehearsal in a disposable clone, 2026-10-31
related: [BUG-0006]
created: 2026-10-31T18:59:24Z
updated: 2026-11-01T16:55:54Z
---
# SpecSync change status keeps approval_valid true after an approved delta is edited

**What happened:** After approval, editing deltas/<module>.md left change status reporting approval_valid true, while change check refused the edited delta
Actual: approval_valid stays true; only change check reports the changed delta

**Steps:** Edit an approved change's semantic delta, then run specsync change status <id> --json
1. Approve a change in a disposable clone
2. Append a REQUIREMENT block to its delta
3. Compare change status --json with change check

**Expected:** Status reports the approval as stale, matching change check

**Why it matters:** An agent trusting status could believe an edited contract is still approved; Chough relies on change check and records the lesson

**Fix:** Not fixed yet.

## Log

- 2026-10-31T18:59:24Z · Reported (SpecSync rehearsal in a disposable clone, 2026-10-31)
- 2026-10-31T20:45:16Z · 2026-10-31 independent reproduction (specsync 6.1.1, disposable examples/shipping): after a synthetic approval, editing the delta (uncommitted or committed) or plan.md left status approved with approval_valid true while change check refused the delta. Cause in spec-sync f240d37: status computes approval_valid from ensure_definition_approval_valid (scope digest), and approved_delta_digests are compared only by ensure_approved_delta_bodies_unchanged during check. Status does recompute the scope digest on each call: changing affected_paths made approval_valid false. Distinct from BUG-0006, where Chough ignored an explicit false.
- 2026-10-31T21:32:30Z · Fix proposed upstream in spec-sync PR #823 (draft, awaiting scope approval): status reports approval_valid false and names changed_approved_deltas when an approved delta body changes.
- 2026-11-01T16:55:54Z · 2026-11-01: Chough now guards this upstream defect: moves to build, test and accept compare SpecSync 6.1.1's approved-delta digests with the current deltas and refuse as refused:native-delta-changed or refused:native-delta-unverifiable (REQ-cli-022). Rehearsed with real SpecSync 6.1.1 in a disposable examples/shipping copy: status kept approval_valid true after the delta edit and chough stage advance refused. Stays open until CI pins a SpecSync release that reports an edited approved delta (spec-sync #823).
