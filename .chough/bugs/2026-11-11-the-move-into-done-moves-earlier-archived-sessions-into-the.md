---
kind: bug
id: 2026-11-11-the-move-into-done-moves-earlier-archived-sessions-into-the
aliases:
  - BUG-the-move-into-done-moves-earlier-archived-sessions-into-the-vault-a-sil-1be5350fa57a5efb1f3a2e13
title: The move into done moves earlier archived sessions into the vault, a silent loss where knowledge is not committed (#165)
status: verified
classification: product-defect
scope: blocking
impact: Silent loss of committed archived session records in adopter projects; blocks 0.6.0's keep-all archive (KNOW-34).
criteria: [KNOW-34, KNOW-30]
requirements: [REQ-history-006]
cases: [TC-history-165-earlier-archives-stay]
evidence:
  - OBS-tc-history-165-earlier-archives-stay-5d690dc9b4d700b32265baf0
  - OBS-tc-history-165-earlier-archives-stay-e355ee5b96eaa89fb762f797
receipts:
  - id: 2026-11-11-tc-history-165-earlier-archives-stay
    aliases: OBS-tc-history-165-earlier-archives-stay-5d690dc9b4d700b32265baf0
    claim: "Ran cargo test --locked --test sessions -- --exact done_leaves_earlier_archived_stories_in_place --include-ignored for TC-history-165-earlier-archives-stay: failed (exit 101)."
    result: failed
    command: cargo test --locked --test sessions -- --exact done_leaves_earlier_archived_stories_in_place --include-ignored
    revision: 24545ca7ee6e23da00aa7f7fb1414f532062995a
    environment: local-macos-arm64
    content: fb2ed25ce7966d09b6c02c352f49ba0549662d7694d44c99c52967513ab980ed
    fingerprint: sha256:0990c9f9074928bfe9b5febeb5919d724be4376ea6b95ec980e3611e960b0c03
    case: TC-history-165-earlier-archives-stay
    case_digest: 6ac30da4167a469abb2cd4cdb8a95b341a26122aa2c7c0ad02dde852585075bb
    observed: 2026-11-11T15:51:34Z
    duration_seconds: 1
fixture: tests/sessions.rs done_leaves_earlier_archived_stories_in_place
environment: local-macos-arm64
revision: 24545ca7ee6e23da00aa7f7fb1414f532062995a
found_by: "GitHub issue #165; algochat repro 2026-11-10; red regression run in plan step 1"
related: []
created: 2026-11-11T15:51:53Z
updated: 2026-11-11T18:17:22Z
---
# The move into done moves earlier archived sessions into the vault, a silent loss where knowledge is not committed (#165)

**What happened:** Advancing a session from accept to done removed an earlier archived session file from .chough/sessions/archive/ with no ledger entry.
Actual: The KNOW-29 roll-up keeps the five newest archived sessions and moves the rest into knowledge/attachments/<key>/sessions/; where knowledge/ is not committed, Git sees the earlier archived sessions as deleted (algochat release-0-7-0, 2026-11-10; GitHub #165).

**Steps:** chough stage advance from accept to done with more than five archived sessions and the librarian on, in a project whose knowledge/ directory is not committed.
1. Create a session and write seven earlier archived sessions under .chough/sessions/archive/, leaving knowledge/ uncommitted.
2. Advance the session from intent to accept, then once more to done.
3. Check .chough/sessions/archive/: earlier-0.json and earlier-1.json are gone, moved into knowledge/attachments/.

**Expected:** Every earlier archived session file stays in place byte for byte; done adds only the advancing session's own archive (KNOW-34, REQ-history-006).

**Why it matters:** Silent loss of committed archived session records in adopter projects; blocks 0.6.0's keep-all archive (KNOW-34).

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-11T15:51:53Z · Reported (GitHub issue #165; algochat repro 2026-11-10; red regression run in plan step 1); evidence: OBS-tc-history-165-earlier-archives-stay-5d690dc9b4d700b32265baf0
- 2026-11-11T18:17:22Z · status: open -> verified. Fixed by plan step 3 (PR #238): sessions finish into one story under .chough/archive/sessions/ and there is no roll-up, so the move into done leaves every earlier story in place. TC-history-165-earlier-archives-stay passed on local-macos-arm64 with the test unignored.; evidence: OBS-tc-history-165-earlier-archives-stay-e355ee5b96eaa89fb762f797
