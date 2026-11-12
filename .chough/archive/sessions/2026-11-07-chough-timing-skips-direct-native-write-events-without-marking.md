---
kind: session
id: 2026-11-07-chough-timing-skips-direct-native-write-events-without-marking
aliases:
  - chough-timing-skips-direct-native-write-events-without-marking-its-report-partial-189-drive-21
objective: chough timing skips direct-native-write events without marking its report partial (#189, DRIVE-21)
intent_scope: DRIVE
modules: [history]
stage: done
stage_status: active
iteration: 1
callers: []
native_change: chough-timing-skips-direct-native-write-events-without-marking-its-report-partial-189-drive-21
pr: ""
criteria: [DRIVE-21, DRIVE-20]
requirements: [REQ-history-013, REQ-cli-057]
cases: [TC-history-timing-skips-direct-writes, TC-cli-change-new-no-spec-change]
design: []
files: []
observations:
  - OBS-tc-history-timing-skips-direct-writes-0a97e1153f349a07b54d9027
  - OBS-tc-cli-change-new-no-spec-change-3fde0095f41dd209e86af817
bugs: []
blockers: []
waiting_on_human: []
run_consents: []
next: After the orc records Leif's review at the final verified tip, run chough change ship with the recorded pass verdict; no commit between review and ship
closed: 2026-11-07
outcome: done
receipts:
  - id: 2026-11-07-tc-history-timing-skips-direct-writes
    aliases: OBS-tc-history-timing-skips-direct-writes-0a97e1153f349a07b54d9027
    claim: "Ran cargo test --locked --test direct_writes -- --exact timing_skips_direct_write_events_without_marking_the_report_partial for TC-history-timing-skips-direct-writes: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test direct_writes -- --exact timing_skips_direct_write_events_without_marking_the_report_partial
    revision: 7e5e942677d2b295ab57b0c4c7d962bb144a16cf
    environment: local-macos-arm64
    content: ea394bd7648a7d3c7df36ec12bce980f901a73d91f768a0bf6159c24dbeaab76
    fingerprint: sha256:53d34a7f09e478ecf5e824ac662afc0bd467845bb934f5abf0a5d537f0cd5b29
    case: TC-history-timing-skips-direct-writes
    case_digest: 2e91fdcbb43294670574df198751dfd1148852ca4007e9c80d594a8e04d249b3
    observed: 2026-11-07T03:37:17Z
    duration_seconds: 3
  - id: 2026-11-07-tc-cli-change-new-no-spec-change
    aliases: OBS-tc-cli-change-new-no-spec-change-3fde0095f41dd209e86af817
    claim: "Ran cargo test --locked --test change -- --exact change_new_passes_no_spec_change_with_its_rationale for TC-cli-change-new-no-spec-change: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test change -- --exact change_new_passes_no_spec_change_with_its_rationale
    revision: 7e5e942677d2b295ab57b0c4c7d962bb144a16cf
    environment: local-macos-arm64
    content: ea394bd7648a7d3c7df36ec12bce980f901a73d91f768a0bf6159c24dbeaab76
    fingerprint: sha256:8da0e2dec63bf941ec8dfe2d7521d868640b27d47adc9dfa33c24dc5b988851e
    case: TC-cli-change-new-no-spec-change
    case_digest: 7228e9c7ab3485be81f47d8ca5c46319320b36a4749c451e407b516d28be001d
    observed: 2026-11-07T03:37:35Z
    duration_seconds: 2
---
# chough timing skips direct-native-write events without marking its report partial (#189, DRIVE-21)

## Goal

chough timing skips direct-native-write events without marking its report partial (#189, DRIVE-21)

## Where it stands

- Refreshed both #189 cases on the merged PR head; both passed

## Decisions and why

- Regression from #183 (#186): timing counted the valid direct-native-write kind as unrecognised and marked its report partial; fix in DRIVE-21's scope by treating it as a known non-segment kind; REQ-history-013 modified to say so
- Folded in at the orc's request: chough change new gains --no-spec-change and --rationale (found by ch-docs-accuracy); REQ-cli-057 modified; tests in tests/change.rs fail first
- Leif (0xLeif) decided "Definition approval of chough-timing-skips-direct-native-write-events-without-marking-its-report-partial-189-drive-21": "approved #189" (orc tab, 2026-11-07, relayed by orc t18; native approval recorded as 0xLeif by the orc at 0635c835 (after the reporting module was added)); recorded by ch-c456 (Claude Opus 5.5); a relayed claim, not authenticated
- Leif decided "Implementation review of chough-timing-skips-direct-native-write-events-without-marking-its-report-partial-189-drive-21": "Yes to all 3 (Recommended)" (Moshi question from orc t18, 2026-11-06); recorded by glm-finish; a relayed claim, not authenticated

## What was verified

- OBS-tc-history-timing-skips-direct-writes-0a97e1153f349a07b54d9027
- OBS-tc-cli-change-new-no-spec-change-3fde0095f41dd209e86af817

## Open threads

## Timeline

- 2026-11-07T02:30:34Z · advance · intent → design ·  ^t-3fb8f5
  - native_change: chough-timing-skips-direct-native-write-events-without-marking-its-report-partial-189-drive-21
  - revision: f41e7ab4c6380698783c1e98a4c6cf2fa96c1ff8
- 2026-11-07T02:30:34Z · advance · design → plan ·  ^t-ffeb47
  - native_change: chough-timing-skips-direct-native-write-events-without-marking-its-report-partial-189-drive-21
  - revision: f41e7ab4c6380698783c1e98a4c6cf2fa96c1ff8
- 2026-11-07T02:30:34Z · gate-opened · plan → plan · Definition approval of chough-timing-skips-direct-native-write-events-without-marking-its-report-partial-189-drive-21 ^t-869d1d
  - native_change: chough-timing-skips-direct-native-write-events-without-marking-its-report-partial-189-drive-21
  - revision: f41e7ab4c6380698783c1e98a4c6cf2fa96c1ff8
- 2026-11-07T02:42:00Z · gate-closed · plan → plan · Definition approval of chough-timing-skips-direct-native-write-events-without-marking-its-report-partial-189-drive-21 ^t-6e560d
  - native_change: chough-timing-skips-direct-native-write-events-without-marking-its-report-partial-189-drive-21
  - revision: 0635c8355f9847fdfd469032b5fe5cd27aca2efb
  - relayed_by: {"decided_by":"Leif (0xLeif)","words":"approved #189","where":"orc tab, 2026-11-07, relayed by orc t18; native approval recorded as 0xLeif by the orc at 0635c835 (after the reporting module was added)","recorded_by":"ch-c456 (Claude Opus 5.5)"}
- 2026-11-07T02:42:03Z · direct-native-write · plan → plan ·  ^t-8a9532
  - native_change: chough-timing-skips-direct-native-write-events-without-marking-its-report-partial-189-drive-21
  - revision: 0635c8355f9847fdfd469032b5fe5cd27aca2efb
  - direct_write: {"change":"chough-timing-skips-direct-native-write-events-without-marking-its-report-partial-189-drive-21","files":[{"path":"state.json","change":"changed"},{"path":"approvals.json","change":"changed"}],"before_state":"draft","after_state":"approved"}
- 2026-11-07T02:42:03Z · advance · plan → build · native:approved ^t-0c6c0d
  - native_change: chough-timing-skips-direct-native-write-events-without-marking-its-report-partial-189-drive-21
  - gate: native:approved
  - revision: 0635c8355f9847fdfd469032b5fe5cd27aca2efb
- 2026-11-07T02:54:16Z · direct-native-write · build → build ·  ^t-cb2fa9
  - native_change: chough-timing-skips-direct-native-write-events-without-marking-its-report-partial-189-drive-21
  - revision: 4f940c4d89d7fa49db38fd29ff06b1f80d200c35
  - direct_write: {"change":"chough-timing-skips-direct-native-write-events-without-marking-its-report-partial-189-drive-21","files":[{"path":"state.json","change":"changed"},{"path":"verification.json","change":"added"},{"path":"verification-attempts.json","change":"added"}],"before_state":"approved","after_state":"verifying"}
- 2026-11-07T02:54:16Z · advance · build → test · native:verifying ^t-03b2e0
  - native_change: chough-timing-skips-direct-native-write-events-without-marking-its-report-partial-189-drive-21
  - gate: native:verifying
  - revision: 4f940c4d89d7fa49db38fd29ff06b1f80d200c35
- 2026-11-07T03:38:24Z · direct-native-write · test → test ·  ^t-4b3ddb
  - native_change: chough-timing-skips-direct-native-write-events-without-marking-its-report-partial-189-drive-21
  - revision: 7e5e942677d2b295ab57b0c4c7d962bb144a16cf
  - direct_write: {"change":"chough-timing-skips-direct-native-write-events-without-marking-its-report-partial-189-drive-21","files":[{"path":"review.json","change":"added"},{"path":"review-attempts.json","change":"added"}],"before_state":"verifying","after_state":"verifying"}
- 2026-11-07T03:38:24Z · advance · test → accept · native:verifying ^t-7ab0f9
  - native_change: chough-timing-skips-direct-native-write-events-without-marking-its-report-partial-189-drive-21
  - gate: native:verifying
  - revision: 7e5e942677d2b295ab57b0c4c7d962bb144a16cf
- 2026-11-07T03:38:25Z · gate-opened · accept → accept · Implementation review of chough-timing-skips-direct-native-write-events-without-marking-its-report-partial-189-drive-21 ^t-71fe60
  - native_change: chough-timing-skips-direct-native-write-events-without-marking-its-report-partial-189-drive-21
  - revision: 7e5e942677d2b295ab57b0c4c7d962bb144a16cf
- 2026-11-07T04:10:28Z · gate-closed · accept → accept · Implementation review of chough-timing-skips-direct-native-write-events-without-marking-its-report-partial-189-drive-21 ^t-f6f467
  - native_change: chough-timing-skips-direct-native-write-events-without-marking-its-report-partial-189-drive-21
  - revision: 5bf2229c670fd0e4e78653a2da425198da3dea59
  - relayed_by: {"decided_by":"Leif","words":"Yes to all 3 (Recommended)","where":"Moshi question from orc t18, 2026-11-06","recorded_by":"glm-finish"}
- 2026-11-07T04:10:52Z · direct-native-write · accept → accept ·  ^t-b461e3
  - native_change: chough-timing-skips-direct-native-write-events-without-marking-its-report-partial-189-drive-21
  - revision: 5bf2229c670fd0e4e78653a2da425198da3dea59
  - direct_write: {"change":"chough-timing-skips-direct-native-write-events-without-marking-its-report-partial-189-drive-21","files":[{"path":"review.json","change":"changed"},{"path":"review-attempts.json","change":"changed"}],"before_state":"verifying","after_state":"verifying"}
- 2026-11-07T04:10:52Z · advance · accept → done · native:archived ^t-91f987
  - native_change: chough-timing-skips-direct-native-write-events-without-marking-its-report-partial-189-drive-21
  - gate: native:archived
  - revision: 5bf2229c670fd0e4e78653a2da425198da3dea59


Closed 2026-11-07: done.
