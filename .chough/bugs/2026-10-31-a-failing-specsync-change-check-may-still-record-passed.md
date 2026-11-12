---
kind: bug
id: 2026-10-31-a-failing-specsync-change-check-may-still-record-passed
aliases: [BUG-0003]
title: A failing SpecSync change check may still record passed verification
status: fixed-upstream
classification: evidence-gap
scope: out-of-scope
impact: Possible false verification evidence upstream; seen once and not reproduced
criteria: [LIFE-7]
requirements: [REQ-cli-008]
cases: []
evidence: []
receipts: []
environment: specsync-6.1.1-macos-arm64
revision: 12c8d4009995e13a42a354e4bc59f641ad9d0dfb
found_by: SpecSync rehearsal in a disposable clone, 2026-10-31
related: [BUG-0002]
created: 2026-10-31T18:59:24Z
updated: 2026-11-12T14:36:24Z
---
# A failing SpecSync change check may still record passed verification

**What happened:** One change check run exited 1 with an undocumented-export error, yet verification.json recorded passed true and status moved to verifying
Actual: verification.json passed: true while the command failed

**Steps:** First change check after adding a pub(crate) item in a new module
1. Add a pub(crate) fn in src/stage.rs listed in the spec files
2. Run specsync change check <id> --json
3. Read verification.json

**Expected:** A failing check records failed verification or none

**Why it matters:** Possible false verification evidence upstream; seen once and not reproduced

**Fix:** Not fixed yet.

## Log

- 2026-10-31T18:59:24Z · Reported (SpecSync rehearsal in a disposable clone, 2026-10-31)
- 2026-10-31T20:45:16Z · 2026-10-31 independent reproduction (specsync 6.1.1, enforcement = strict): change check recorded passed true for specsync check --spec shipping (non-strict) with undocumented exports; check --commit then exited 1 on the effective-contract export errors without recording an attempt, and status kept next_action review. Cause in spec-sync f240d37: change_requires_strict_validation uses only built-in SpecSync paths and modules or --strict, never the project's enforcement setting, and the commit path validates more strictly than the recorded verification.
- 2026-10-31T21:32:30Z · 2026-10-31 correction to the earlier note: configured enforcement = "strict" does not fail warnings (specsync check exits 0 under it; only the --strict flag fails), so the cause stated above is wrong. Per spec-sync PR #824's investigation, the effective-contract gate skips the change being verified on its first check, so that check records passed; later check --commit, finalize and audit refuse without recording an attempt, leaving the stale pass current. Fix proposed upstream in spec-sync PR #824 (draft, awaiting scope approval).
- 2026-11-12T14:36:24Z · status: open -> fixed-upstream. Fixed upstream in SpecSync (CorvidLabs/spec-sync#824): re-checked by t37b on spec-sync main 46f55652 (SpecSync 6.2.0); this repository now pins the released SpecSync 6.3.0, which carries that fix. · by claude-code (hc t382)
