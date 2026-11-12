---
kind: bug
id: 2026-11-12-after-two-roots-merge-a-bug-s-bare-evidence-link-turns
aliases:
  - BUG-after-two-roots-merge-a-bug-s-bare-evidence-link-turns-ambiguous-and-st-e0c0f5e9cb4891137251f9d1
title: After two roots merge, a bug's bare evidence link turns ambiguous and status, next and bug show refuse
status: verified
classification: product-defect
scope: in-scope
impact: Parallel branches that record Chough work break status and next on merge, against DRIVE-12
criteria: [DRIVE-12]
requirements: [REQ-cli-023]
cases: [TC-cli-independently-added-records-merge-cleanly]
evidence:
  - OBS-tc-cli-independently-added-records-merge-cleanly-a9f84d2d7eb56edd2a095987
  - OBS-tc-cli-independently-added-records-merge-cleanly-c10279b664da101b072cacbf
receipts:
  - id: 2026-11-12-tc-cli-independently-added-records-merge-cleanly
    aliases: OBS-tc-cli-independently-added-records-merge-cleanly-a9f84d2d7eb56edd2a095987
    claim: "Ran cargo test --locked --test cases -- --exact independently_added_records_merge_without_conflict_or_replacement for TC-cli-independently-added-records-merge-cleanly: failed (exit 101, tests run 1)."
    result: failed
    command: cargo test --locked --test cases -- --exact independently_added_records_merge_without_conflict_or_replacement
    revision: 18ae842ddc52c45009aabd6bb9b07036a4fb03b2
    environment: local-macos-arm64
    content: 5559f34fc8b0d774a933d33b5d5655a0cdc97a04e30c96e82887a358fb9181aa
    fingerprint: sha256:708ece7e0a6a21c9d7ae42b33f876ebbabe8984c8f7207d73d7f0f5ac4da51d2
    case: TC-cli-independently-added-records-merge-cleanly
    case_digest: 0a559e4647d3fb410bd47718d55643b8120cedcb1d8043acd6c986ed8613f06d
    observed: 2026-11-12T00:48:40Z
    duration_seconds: 2
  - id: 2026-11-12-tc-cli-independently-added-records-merge-cleanly-153e1a
    aliases: OBS-tc-cli-independently-added-records-merge-cleanly-c10279b664da101b072cacbf
    claim: "Ran cargo test --locked --test cases -- --exact independently_added_records_merge_without_conflict_or_replacement for TC-cli-independently-added-records-merge-cleanly: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test cases -- --exact independently_added_records_merge_without_conflict_or_replacement
    revision: 68f28b604db36829dea6089ea916937ee87b9169
    environment: local-macos-arm64
    content: a6a7a612101c63db7ce039a393d6190030ba04ac62321b71482b8915deadbc99
    fingerprint: sha256:6fd799ad8f307abc34f8f2f47a96d7495b810307176844122315da618db7fcaf
    case: TC-cli-independently-added-records-merge-cleanly
    case_digest: 0a559e4647d3fb410bd47718d55643b8120cedcb1d8043acd6c986ed8613f06d
    observed: 2026-11-12T00:56:51Z
    duration_seconds: 3
environment: local-macos-arm64
revision: cf09ccef8881a57b79ff0ce122701ff2623bad5a
found_by: t38b's review (F1) of t37j's item 8; red at 18ae842d (leif/0.6.0-step4-evidence-fixes)
related: []
created: 2026-11-12T00:49:14Z
updated: 2026-11-12T00:57:08Z
---
# After two roots merge, a bug's bare evidence link turns ambiguous and status, next and bug show refuse

**What happened:** Each root's bug links its own case run by the bare ID cases run printed; after the branches merge, both entries share that name, and status, next and bug show refuse the whole report as ambiguous
Actual: refused: evidence <id> is ambiguous ...; link one as <session-id>/<evidence-id>

**Steps:** In two roots on separate branches, each in its own session, run the same case and add a bug linking the run's printed ID, then merge
1. Run the merge test with each root's bug linking its run by the bare ID

**Expected:** Both roots' records read after the merge with no hand edit: a bug's link resolves to its own root's run (REQ-cli-023)

**Why it matters:** Parallel branches that record Chough work break status and next on merge, against DRIVE-12

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-12T00:49:14Z · Reported (t38b's review (F1) of t37j's item 8; red at 18ae842d (leif/0.6.0-step4-evidence-fixes)); evidence: OBS-tc-cli-independently-added-records-merge-cleanly-a9f84d2d7eb56edd2a095987
- 2026-11-12T00:57:08Z · status: open -> verified. Green after t37j's 7667e63e (bug links stored as <session>/<id>) and 8ff04aeb (a link that stays ambiguous reads 'ambiguous', naming its places): after the merge, each root's bug resolves to its own root's run and status and next read.; evidence: OBS-tc-cli-independently-added-records-merge-cleanly-c10279b664da101b072cacbf
