---
kind: bug
id: 2026-11-12-session-list-refuses-with-no-sessions-after-a-migration-whose
aliases:
  - BUG-session-list-refuses-with-no-sessions-after-a-migration-whose-sessions-d6211bbc318217c4ba7a4617
title: session list refuses with no sessions after a migration whose sessions were all archived
status: verified
classification: product-defect
scope: in-scope
impact: A migrated project cannot list its history until a new session is made
criteria: []
requirements: [REQ-records-009, REQ-history-006]
cases: []
evidence:
  - OBS-tc-history-session-list-reads-stories-alone-3d4883a720c40b8a7c61e5e2
  - OBS-tc-history-session-list-reads-stories-alone-895d86007e4f198ef9af9e46
receipts:
  - id: 2026-11-12-tc-history-session-list-reads-stories-alone
    aliases: OBS-tc-history-session-list-reads-stories-alone-3d4883a720c40b8a7c61e5e2
    claim: "Ran cargo test --locked --test sessions -- --exact session_list_lists_archived_stories_when_no_session_folder_is_left for TC-history-session-list-reads-stories-alone: failed (exit 101, tests run 1)."
    result: failed
    command: cargo test --locked --test sessions -- --exact session_list_lists_archived_stories_when_no_session_folder_is_left
    revision: 0ee296aaab0e7d9cc584cc5be2d446701940fcf3
    environment: local-macos-arm64
    content: 2cafae9b427ce78cfa15d45b1fa80e7f728962a43bca0270971aa2af6e280d2b
    fingerprint: sha256:209ec5b32b5f8d702f950764a29e65c2dae1aa445c39723a81341f1e112fb9f6
    case: TC-history-session-list-reads-stories-alone
    case_digest: c1a8723b648ed3d524f81a53e5ecb798873ddf4e26d0d11f4b0d9aa477070c60
    observed: 2026-11-12T02:04:01Z
    duration_seconds: 1
  - id: 2026-11-12-tc-history-session-list-reads-stories-alone-7cea59
    aliases: OBS-tc-history-session-list-reads-stories-alone-895d86007e4f198ef9af9e46
    claim: "Ran cargo test --locked --test sessions -- --exact session_list_lists_archived_stories_when_no_session_folder_is_left for TC-history-session-list-reads-stories-alone: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact session_list_lists_archived_stories_when_no_session_folder_is_left
    revision: 0ee26ac0d72f466129525f27afaa5526cd55322e
    environment: local-macos-arm64
    content: cc056522bd209ed839bdb5d0a0b03f05210757d46be1edf71d7246a66228c3a2
    fingerprint: sha256:46a2e8d7e0da1e643f2fcd47e7b9aa71d45134adb722293c53d8e5521ec1ab64
    case: TC-history-session-list-reads-stories-alone
    case_digest: c1a8723b648ed3d524f81a53e5ecb798873ddf4e26d0d11f4b0d9aa477070c60
    observed: 2026-11-12T02:17:02Z
    duration_seconds: 1
environment: local-macos-arm64, tester clone at 367dc86b, debug build, SpecSync 6.2.0
revision: ebb812d8adfecacba8bacee838f42f996bd2c056
found_by: t384, test stage, area upgrade
related: []
created: 2026-11-12T02:02:10Z
updated: 2026-11-12T02:18:06Z
---
# session list refuses with no sessions after a migration whose sessions were all archived

**What happened:** After migrating a 0.5.0 project whose sessions are all finished, chough session list exits 1 'no sessions; run chough session new <key>', though .chough/archive/sessions/ holds the migrated stories
Actual: session list requires .chough/sessions/ to exist

**Steps:** Migrate a 0.5.x project with no open session, then chough session list

**Expected:** Every record is readable by session list after the migration (REQ-records-009); archived stories are listed

**Why it matters:** A migrated project cannot list its history until a new session is made

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-12T02:02:10Z · Reported (t384, test stage, area upgrade)
- 2026-11-12T02:04:19Z · status: open -> investigating. Red at 0ee296aa (leif/0.6.0-test1-red: d854ce1a plus the regression tests): TC-history-session-list-reads-stories-alone failed for this bug's reason; fix in 4936b629.; evidence: OBS-tc-history-session-list-reads-stories-alone-3d4883a720c40b8a7c61e5e2
- 2026-11-12T02:18:06Z · status: investigating -> verified. Green at 0ee26ac0: TC-history-session-list-reads-stories-alone passed after the fix; full suite 626 passed, clippy and fmt clean at 7b645d60.; evidence: OBS-tc-history-session-list-reads-stories-alone-895d86007e4f198ef9af9e46
