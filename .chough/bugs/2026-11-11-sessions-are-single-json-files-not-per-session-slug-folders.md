---
kind: bug
id: 2026-11-11-sessions-are-single-json-files-not-per-session-slug-folders
aliases:
  - BUG-sessions-are-single-json-files-not-per-session-slug-folders-214-aeb99dcab249f2058eb08ca2
title: Sessions are single JSON files, not per-session slug folders (#214)
status: verified
classification: product-defect
scope: in-scope
impact: "Sessions and their proof scatter and merge badly (GitHub #214, DRIVE-24)"
criteria: [DRIVE-24]
requirements: [REQ-history-006]
cases: [TC-history-sessions-in-folders]
evidence:
  - OBS-tc-history-sessions-in-folders-116d392ec28a19954f3ac64c
  - OBS-tc-history-sessions-in-folders-a0702b1be02a8ecde903326e
receipts:
  - id: 2026-11-11-tc-history-sessions-in-folders
    aliases: OBS-tc-history-sessions-in-folders-116d392ec28a19954f3ac64c
    claim: "Ran cargo test --locked --test sessions -- each_session_has_its_own_folder_and_timeline sessions_live_in_folders_and_finish_into_one_story for TC-history-sessions-in-folders: failed (exit 101)."
    result: failed
    command: cargo test --locked --test sessions -- each_session_has_its_own_folder_and_timeline sessions_live_in_folders_and_finish_into_one_story
    revision: 370d221d68840f2e263e90b927a200438a7a0173
    environment: local-macos-arm64
    content: 843a083f60a67703f38514e1fcffa147b0e3c500ec307ca879614ca5ee672a4c
    fingerprint: sha256:d87b84fbc113470b4001894d0dc43ea5f8b7ad6527e6672ce97b4c725e8e3863
    case: TC-history-sessions-in-folders
    case_digest: 2c0d9e9e02eb15ab6b3a49a45cba2e06c8f48945ff61d4ed05191c1d3893beff
    observed: 2026-11-11T20:35:56Z
    duration_seconds: 0
  - id: 2026-11-11-tc-history-sessions-in-folders-00dce3
    aliases: OBS-tc-history-sessions-in-folders-a0702b1be02a8ecde903326e
    claim: "Ran cargo test --locked --test sessions -- each_session_has_its_own_folder_and_timeline sessions_live_in_folders_and_finish_into_one_story for TC-history-sessions-in-folders: passed (exit 0)."
    result: passed
    command: cargo test --locked --test sessions -- each_session_has_its_own_folder_and_timeline sessions_live_in_folders_and_finish_into_one_story
    revision: 7b4cd3eb827f70c821255c0efb0326f17d7ca23a
    environment: local-macos-arm64
    content: e287cef970a2e70b5d716af6bc20f127df4d7275ee92871b760314f7393345e9
    fingerprint: sha256:6a4283d29412197b337fbfb57e85b143a6c7745b05a35c01d3d6b4329e37daca
    case: TC-history-sessions-in-folders
    case_digest: 2c0d9e9e02eb15ab6b3a49a45cba2e06c8f48945ff61d4ed05191c1d3893beff
    observed: 2026-11-11T20:38:38Z
    duration_seconds: 5
environment: local-macos-arm64
revision: 67d001e55e59fa271166d92c95ea44fac6d4f304
found_by: "GitHub issue #214; red on 0.5.0 at leif/0.6.0-batch-red 370d221d"
related: []
created: 2026-11-11T20:36:57Z
updated: 2026-11-11T20:39:42Z
---
# Sessions are single JSON files, not per-session slug folders (#214)

**What happened:** chough session new writes .chough/sessions/<key>.json; there is no folder per session for its record, evidence and agent notes
Actual: one JSON file, .chough/sessions/alpha.json

**Steps:** chough session new alpha --objective <outcome>
1. chough session new alpha --objective <outcome>

**Expected:** a folder .chough/sessions/<date>-<slug>/ holding session.md, evidence/ and agent/

**Why it matters:** Sessions and their proof scatter and merge badly (GitHub #214, DRIVE-24)

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-11T20:36:57Z · Reported (GitHub issue #214; red on 0.5.0 at leif/0.6.0-batch-red 370d221d); evidence: OBS-tc-history-sessions-in-folders-116d392ec28a19954f3ac64c
- 2026-11-11T20:39:42Z · status: open -> verified. Fixed by plan step 3 (PR #238): sessions are folders with session.md; the case passed after its 0.5.0 failure.; evidence: OBS-tc-history-sessions-in-folders-a0702b1be02a8ecde903326e
