---
kind: session
id: 2026-11-06-place-the-77-sessions-entry-under-0-4-0-and-add-111-s-legacy
aliases:
  - place-the-77-sessions-entry-under-0-4-0-and-add-111-s-legacy-anchor-session-fix-to-the-change-log-life-16
objective: "Place the #77 sessions entry under 0.4.0 and add #111's legacy-anchor session fix to the change log (LIFE-16)"
intent_scope: LIFE
modules: [cli]
stage: done
stage_status: active
iteration: 1
callers: []
native_change: place-the-77-sessions-entry-under-0-4-0-and-add-111-s-legacy-anchor-session-fix-to-the-change-log-life-16
pr: ""
criteria: [LIFE-16]
requirements: []
cases: []
design: []
files: []
observations:
  - OBS-tc-history-changelog-111-under-0-4-0-96f4c6ed9d193798a3d7205c
  - OBS-tc-history-changelog-sessions-under-0-4-0-4fe7138779b4ed79b0a97ae4
  - OBS-tc-history-changelog-111-under-0-4-0-51c2b5238ff8ac90bf33995d
  - OBS-tc-history-changelog-sessions-under-0-4-0-17d5a214eb7fd5ee0d365aa0
bugs: []
blockers: []
waiting_on_human: []
run_consents: []
next: Send 'ready for approval' to orc t18; then commit the failing tests/changelog.rs
closed: 2026-11-06
outcome: done
receipts:
  - id: OBS-tc-history-changelog-111-under-0-4-0-96f4c6ed9d193798a3d7205c
    removed: .chough/archive/migrations/2026-11-12-from-0-5.md
  - id: OBS-tc-history-changelog-sessions-under-0-4-0-4fe7138779b4ed79b0a97ae4
    removed: .chough/archive/migrations/2026-11-12-from-0-5.md
  - id: 2026-11-06-tc-history-changelog-111-under-0-4-0
    aliases: OBS-tc-history-changelog-111-under-0-4-0-51c2b5238ff8ac90bf33995d
    claim: "Ran cargo test --locked --test changelog -- --exact the_111_legacy_anchor_session_fix_is_under_0_4_0 for TC-history-changelog-111-under-0-4-0: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test changelog -- --exact the_111_legacy_anchor_session_fix_is_under_0_4_0
    revision: 2a5e6008d349d3da9a4d0fcb9eb3b327de374ae0
    environment: local-macos-arm64
    content: 81496fc0b2ff8929808b14dd1ccbd89ad7fbe80fadf2dde3ab8676b6ccb035aa
    fingerprint: sha256:45659ff4bb2f81fa548f18ddcc9551f697128784e3e0d6ef8344423e3291aba2
    case: TC-history-changelog-111-under-0-4-0
    case_digest: 16634577e79e84978df7a5d4dd439267c81bc94e086802f4241cdb3bee1e527c
    observed: 2026-11-06T14:18:46Z
    duration_seconds: 0
  - id: 2026-11-06-tc-history-changelog-sessions-under-0-4-0
    aliases: OBS-tc-history-changelog-sessions-under-0-4-0-17d5a214eb7fd5ee0d365aa0
    claim: "Ran cargo test --locked --test changelog -- --exact the_sessions_entry_is_under_0_4_0_where_77_first_shipped for TC-history-changelog-sessions-under-0-4-0: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test changelog -- --exact the_sessions_entry_is_under_0_4_0_where_77_first_shipped
    revision: 2a5e6008d349d3da9a4d0fcb9eb3b327de374ae0
    environment: local-macos-arm64
    content: 81496fc0b2ff8929808b14dd1ccbd89ad7fbe80fadf2dde3ab8676b6ccb035aa
    fingerprint: sha256:e86971055f4a1a93e9ec74072f09e0d36325cb3cff16e70e740f218be85c4c9b
    case: TC-history-changelog-sessions-under-0-4-0
    case_digest: 0a1420ec5ac35d5ebbf89cd5a10f9ecbec246a34b29e1674b63ac80cc7b55090
    observed: 2026-11-06T14:18:45Z
    duration_seconds: 2
---
# Place the #77 sessions entry under 0.4.0 and add #111's legacy-anchor session fix to the change log (LIFE-16)

## Goal

Place the #77 sessions entry under 0.4.0 and add #111's legacy-anchor session fix to the change log (LIFE-16)

## Where it stands

- Implemented at 54f7a782; both changelog cases pass after failing at 77b8e0f2
- Both changelog cases pass again after the clippy fix at 2a5e6008

## Decisions and why

- decided: one docs-only PR that moves the #77 sessions entry to [0.4.0] and adds #111's entry, because #77 and #111 first shipped in v0.4.0 (tags); Leif chose 'Yes, one small PR' and captured LIFE-16 with 'Capture as written' (orc relay, hc t18, 2026-11-06)
- Leif, through the orc (t18), which recorded the approval as 0xLeif at 548cab2d decided "Scope decision of place-the-77-sessions-entry-under-0-4-0-and-add-111-s-legacy-anchor-session-fix-to-the-change-log-life-16": "im telling you to approve and continue all of these... approve as me (Leif in the orc tab, relayed by the orc: approval for the CHANGELOG change (548cab2d) recorded as 0xLeif; build it, PR, then 'ready for review'; it merges after #150)" (orc relay from t18 to ch-pr69 (t7z), 2026-11-06 morning MST); recorded by ch-pr69 (Claude Code in t7z); a relayed claim, not authenticated
- Leif, through the orc (t18), which recorded the review as 0xLeif at c8c1b6b7 decided "Implementation review of place-the-77-sessions-entry-under-0-4-0-and-add-111-s-legacy-anchor-session-fix-to-the-change-log-life-16": "Record all 4 (Leif in the orc tab, relayed by the orc: #160 at c8c1b6b7, Leif's review recorded as 0xLeif; ship with no commit between, archive, done, push the archive tip, full CI green, merge pinned to that head; it merges after #150)" (orc relay from t18 to ch-pr69 (t7z), 2026-11-06 morning MST); recorded by ch-pr69 (Claude Code in t7z); a relayed claim, not authenticated

## What was verified

- OBS-tc-history-changelog-111-under-0-4-0-96f4c6ed9d193798a3d7205c
- OBS-tc-history-changelog-sessions-under-0-4-0-4fe7138779b4ed79b0a97ae4
- OBS-tc-history-changelog-111-under-0-4-0-51c2b5238ff8ac90bf33995d
- OBS-tc-history-changelog-sessions-under-0-4-0-17d5a214eb7fd5ee0d365aa0

## Open threads

## Timeline

- 2026-11-06T13:46:02Z · gate-opened · intent → intent · Scope decision of place-the-77-sessions-entry-under-0-4-0-and-add-111-s-legacy-anchor-session-fix-to-the-change-log-life-16 ^t-f3e66d
  - native_change: place-the-77-sessions-entry-under-0-4-0-and-add-111-s-legacy-anchor-session-fix-to-the-change-log-life-16
  - revision: b535b92fb0e7cf5679449c18860e136bb27397e6
- 2026-11-06T14:12:26Z · gate-closed · intent → intent · Scope decision of place-the-77-sessions-entry-under-0-4-0-and-add-111-s-legacy-anchor-session-fix-to-the-change-log-life-16 ^t-152c58
  - native_change: place-the-77-sessions-entry-under-0-4-0-and-add-111-s-legacy-anchor-session-fix-to-the-change-log-life-16
  - revision: 548cab2d59fec42a07761a80d3a91b8cb9a54998
  - relayed_by: {"decided_by":"Leif, through the orc (t18), which recorded the approval as 0xLeif at 548cab2d","words":"im telling you to approve and continue all of these... approve as me (Leif in the orc tab, relayed by the orc: approval for the CHANGELOG change (548cab2d) recorded as 0xLeif; build it, PR, then 'ready for review'; it merges after #150)","where":"orc relay from t18 to ch-pr69 (t7z), 2026-11-06 morning MST","recorded_by":"ch-pr69 (Claude Code in t7z)"}
- 2026-11-06T14:12:27Z · advance · intent → design · Intent is LIFE-16, captured as written by Leif ^t-33eb43
  - native_change: place-the-77-sessions-entry-under-0-4-0-and-add-111-s-legacy-anchor-session-fix-to-the-change-log-life-16
  - revision: 548cab2d59fec42a07761a80d3a91b8cb9a54998
- 2026-11-06T14:12:27Z · skip · design → plan · Change-log text only: no visuals or interactions to design ^t-69e821
  - native_change: place-the-77-sessions-entry-under-0-4-0-and-add-111-s-legacy-anchor-session-fix-to-the-change-log-life-16
  - revision: 548cab2d59fec42a07761a80d3a91b8cb9a54998
- 2026-11-06T14:12:31Z · advance · plan → build · Scope approved as 0xLeif by the orc at 548cab2d on Leif's instruction ^t-301b26
  - native_change: place-the-77-sessions-entry-under-0-4-0-and-add-111-s-legacy-anchor-session-fix-to-the-change-log-life-16
  - gate: native:approved
  - revision: 548cab2d59fec42a07761a80d3a91b8cb9a54998
- 2026-11-06T14:16:00Z · advance · build → test · Implemented at 54f7a782 ^t-fa1bc3
  - native_change: place-the-77-sessions-entry-under-0-4-0-and-add-111-s-legacy-anchor-session-fix-to-the-change-log-life-16
  - gate: native:approved
  - revision: 54f7a782597a06819ce72c9c0feaf14ca5f4cded
- 2026-11-06T14:16:13Z · advance · test → accept · Both cases pass after failing first ^t-9dba25
  - native_change: place-the-77-sessions-entry-under-0-4-0-and-add-111-s-legacy-anchor-session-fix-to-the-change-log-life-16
  - gate: native:approved
  - revision: 54f7a782597a06819ce72c9c0feaf14ca5f4cded
- 2026-11-06T14:16:13Z · gate-opened · accept → accept · Implementation review of place-the-77-sessions-entry-under-0-4-0-and-add-111-s-legacy-anchor-session-fix-to-the-change-log-life-16 ^t-47e612
  - native_change: place-the-77-sessions-entry-under-0-4-0-and-add-111-s-legacy-anchor-session-fix-to-the-change-log-life-16
  - revision: 54f7a782597a06819ce72c9c0feaf14ca5f4cded
- 2026-11-06T14:51:10Z · gate-closed · accept → accept · Implementation review of place-the-77-sessions-entry-under-0-4-0-and-add-111-s-legacy-anchor-session-fix-to-the-change-log-life-16 ^t-d9d353
  - native_change: place-the-77-sessions-entry-under-0-4-0-and-add-111-s-legacy-anchor-session-fix-to-the-change-log-life-16
  - revision: d62a86605ce5037fa0f88d8f92618b93c1945a03
  - relayed_by: {"decided_by":"Leif, through the orc (t18), which recorded the review as 0xLeif at c8c1b6b7","words":"Record all 4 (Leif in the orc tab, relayed by the orc: #160 at c8c1b6b7, Leif's review recorded as 0xLeif; ship with no commit between, archive, done, push the archive tip, full CI green, merge pinned to that head; it merges after #150)","where":"orc relay from t18 to ch-pr69 (t7z), 2026-11-06 morning MST","recorded_by":"ch-pr69 (Claude Code in t7z)"}
- 2026-11-06T14:53:07Z · advance · accept → done · Archived natively at d62a8660 after the review recorded as 0xLeif at c8c1b6b7 ^t-648fa7
  - native_change: place-the-77-sessions-entry-under-0-4-0-and-add-111-s-legacy-anchor-session-fix-to-the-change-log-life-16
  - gate: native:archived
  - revision: d62a86605ce5037fa0f88d8f92618b93c1945a03


Closed 2026-11-06: done.
