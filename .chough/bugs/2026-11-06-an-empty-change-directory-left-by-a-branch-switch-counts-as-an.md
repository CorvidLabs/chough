---
kind: bug
id: 2026-11-06-an-empty-change-directory-left-by-a-branch-switch-counts-as-an
aliases:
  - BUG-an-empty-change-directory-left-by-a-branch-switch-counts-as-an-open-nat-505311f29b37d46cccc197e6
title: An empty change directory left by a branch switch counts as an open native change
status: verified
classification: product-defect
scope: out-of-scope
impact: A branch-switching agent commits an index that CI's knowledge check rejects; library archive --apply may refuse with a false open change
criteria: [KNOW-1]
requirements: [REQ-cli-043]
cases: []
evidence:
  - OBS-tc-cli-empty-change-dir-not-open-6bb0ab6972683f13461a9239
  - 2026-11-12-release-0-6-0/2026-11-12-tc-cli-empty-change-dir-not-open
  - 2026-11-12-release-0-6-0/2026-11-12-tc-cli-empty-change-dir-not-open-483046
receipts:
  - id: 2026-11-12-tc-cli-empty-change-dir-not-open
    aliases: OBS-tc-cli-empty-change-dir-not-open-6bb0ab6972683f13461a9239
    claim: "Ran cargo test --locked --test evidence -- --exact an_empty_change_directory_left_by_a_branch_switch_is_not_an_open_change for TC-cli-empty-change-dir-not-open: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test evidence -- --exact an_empty_change_directory_left_by_a_branch_switch_is_not_an_open_change
    revision: bc8020474b181bbdbfc6671513016ecb1fd0bfb2
    environment: local-macos-arm64
    content: ccde5070f89f807682fe59e75ed34633c9b8d0d2c5b3cdd8a21533d45f2e20bc
    fingerprint: sha256:33fcb863563d381012be1951b1ab1fdf67afcdbb7d8e0340e058a1e1b67f7ca0
    case: TC-cli-empty-change-dir-not-open
    case_digest: c778d32397bac0d8944cad3f34670c3290a4189e4b11c13eeb69aea13ad5d934
    observed: 2026-11-12T00:19:21Z
    duration_seconds: 1
environment: local-macos-arm64
revision: 2daa96b204c6c42b1f9c87dad056a063eb6164b2
found_by: ch-c456 agent, C5 (#137) closeout, 2026-11-05
related: []
created: 2026-11-06T05:36:40Z
updated: 2026-11-12T15:02:16Z
---
# An empty change directory left by a branch switch counts as an open native change

**What happened:** knowledge/INDEX.md lists a change as active although its branch is not checked out, so library index --check fails in a fresh clone of the same commit
Actual: chough library index lists the empty directory under Active, and the committed index differs from what a fresh clone generates (knowledge step of the verify lane failed on Linux at 2daa96b2)

**Steps:** Switch from a branch with an open SpecSync change to one without it: Git removes the tracked files but leaves the empty .specsync/changes/<id>/deltas/ directory; then run chough library index
1. On claude/test/c6-nine-criteria-assertions (open change give-real-...-138), run git checkout claude/test/c5-eight-validity-readers
2. ls .specsync/changes/: give-real-...-138/deltas/ remains, empty
3. chough library index: knowledge/INDEX.md Active lists give-real-...-138; a fresh clone of the same commit generates no such line

**Expected:** A directory under .specsync/changes/ with no change files is not an open change: the index, the open-change refusal of library archive --apply and every other reader skip it

**Why it matters:** A branch-switching agent commits an index that CI's knowledge check rejects; library archive --apply may refuse with a false open change

**Fix:** Not fixed yet.

## Log

- 2026-11-06T05:36:40Z · Reported (ch-c456 agent, C5 (#137) closeout, 2026-11-05)
- 2026-11-06T06:50:55Z · Filed as GitHub issue CorvidLabs/chough#148 (2026-11-05, at the orc's request deciding for Leif overnight).
- 2026-11-12T00:20:54Z · Fresh re-check on 0.5.0 (t38b, bc802047): TC-cli-empty-change-dir-not-open passes there, next included, since #157 fixed the index before 0.5.0. Disposition 2: closed with the #157 link at plan step 14; 0.6.0 keeps the rule for every reader (its test is ported in t37j's item 8).; evidence: OBS-tc-cli-empty-change-dir-not-open-6bb0ab6972683f13461a9239
- 2026-11-12T15:01:59Z · evidence: 2026-11-12-release-0-6-0/2026-11-12-tc-cli-empty-change-dir-not-open. Red recorded through 0.6.0: #157's test fails against the code before #157 (8f620c39). The fix is #157, merged 2026-11-06; GitHub #148 is closed. · by claude-code (hc t382)
- 2026-11-12T15:02:16Z · status: open -> verified; evidence: 2026-11-12-release-0-6-0/2026-11-12-tc-cli-empty-change-dir-not-open-483046. Green at a57c3016: TC-cli-empty-change-dir-not-open passes on this tree after the red against the code before #157. Fixed by #157 (merged 2026-11-06); GitHub #148 is closed. · by claude-code (hc t382)
