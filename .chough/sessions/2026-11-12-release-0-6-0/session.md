---
kind: session
id: 2026-11-12-release-0-6-0
aliases: [release-0-6-0]
objective: "Prepare the Chough 0.6.0 release as the dogfood cycle, driven with the 0.6.0 build only: Cargo 0.6.0 and the dated CHANGELOG [0.6.0] with the dogfood results; fix what the dogfood found (D1, the pointer naming a migrated key); clear the CodeQL false positive; close the KNOW gaps (#216) and #148 through Chough; settle the four flaky-test bugs"
intent_scope: ""
modules: []
stage: build
stage_status: active
iteration: 1
callers: []
native_change: prepare-the-chough-0-6-0-release-cargo-0-6-0-and-the-dated-changelog-with-the-dogfood-results-and-the-fixes-the-dogfood
pr: ""
criteria: [KNOW-3, KNOW-4, KNOW-6, KNOW-15, KNOW-24, DRIVE-27]
requirements: []
cases: []
design: []
files: []
observations:
  - 2026-11-12-tc-records-a-0-5-pointer-still-names-its-session
  - 2026-11-12-tc-records-a-0-5-pointer-still-names-its-session-b81047
bugs: []
blockers: []
waiting_on_human: []
run_consents: []
next: "Wait for Leif's scope decision at the gate; then build: D1 red then green first"
---
# Prepare the Chough 0.6.0 release as the dogfood cycle, driven with the 0.6.0 build only: Cargo 0.6.0 and the dated CHANGELOG [0.6.0] with the dogfood results; fix what the dogfood found (D1, the pointer naming a migrated key); clear the CodeQL false positive; close the KNOW gaps (#216) and #148 through Chough; settle the four flaky-test bugs

## Goal

Prepare the Chough 0.6.0 release as the dogfood cycle, driven with the 0.6.0 build only: Cargo 0.6.0 and the dated CHANGELOG [0.6.0] with the dogfood results; fix what the dogfood found (D1, the pointer naming a migrated key); clear the CodeQL false positive; close the KNOW gaps (#216) and #148 through Chough; settle the four flaky-test bugs

## Where it stands

## Decisions and why

- decided: the 0.6.0 release preparation is the dogfood cycle, one small native change driven with the 0.6.0 build only, scoped by t370 (relaying Leif's plan): Cargo 0.6.0 and the dated CHANGELOG with dogfood results, D1, the CodeQL helper rename, the KNOW case links and runs, #148 red then green, and settling the four flaky-test bugs; no tag, release or merge without Leif
- Leif decided "Approve the scope of the 0.6.0 release preparation": "Yes I approve" (coordinator chat (Claude Code, hc t370), 2026-11-12T14:44Z, replying to the coordinator's summary of the 0.6.0 release-prep scope; D2 deferred by Leif's choice 'Fix after 0.6.0' at 2026-11-12T14:46Z; relayed by claude-code-t370); recorded by claude-code-t382; a relayed claim, not authenticated; ran specsync change approve prepare-the-chough-0-6-0-release-cargo-0-6-0-and-the-dated-changelog-with-the-dogfood-results-and-the-fixes-the-dogfood as 0xLeif

## What was verified

- [Ran cargo test --locked --test migrate -- --exact a_pointer_naming_a_migrated_key_still_names_its_session for TC-records-a-0-5-pointer-still-names-its-session: failed (exit 101, tests run 1).](evidence/2026-11-12-tc-records-a-0-5-pointer-still-names-its-session.md)
- [Ran cargo test --locked --test migrate -- --exact a_pointer_naming_a_migrated_key_still_names_its_session for TC-records-a-0-5-pointer-still-names-its-session: passed (exit 0, tests run 1).](evidence/2026-11-12-tc-records-a-0-5-pointer-still-names-its-session-b81047.md)

## Open threads

## Timeline

- 2026-11-12T14:40:28Z · advance · intent → design · Intent: the release preparation and the dogfood findings, scoped by t370 ^t-6b111f
  - revision: 166d1ea1d37971e4c95ee8ba02249a1a46b2a0d2
  - by: claude-code (hc t382)
- 2026-11-12T14:40:28Z · skip · design → plan · Release preparation and CLI fixes; nothing visual ^t-692d4f
  - revision: 166d1ea1d37971e4c95ee8ba02249a1a46b2a0d2
  - by: claude-code (hc t382)
- 2026-11-12T14:42:58Z · gate-opened · plan → plan · Approve the scope of the 0.6.0 release preparation ^t-676797
  - native_change: prepare-the-chough-0-6-0-release-cargo-0-6-0-and-the-dated-changelog-with-the-dogfood-results-and-the-fixes-the-dogfood
  - revision: 166d1ea1d37971e4c95ee8ba02249a1a46b2a0d2
  - holds: plan
  - drives: approve
  - by: claude-code (hc t382)
- 2026-11-12T14:53:15Z · gate-closed · plan → plan · Approve the scope of the 0.6.0 release preparation ^t-a4e2d6
  - native_change: prepare-the-chough-0-6-0-release-cargo-0-6-0-and-the-dated-changelog-with-the-dogfood-results-and-the-fixes-the-dogfood
  - revision: b940bd6c1d1ee3c58522479e412f62841f01c4e5
  - relayed_by: {"decided_by":"Leif","words":"Yes I approve","where":"coordinator chat (Claude Code, hc t370), 2026-11-12T14:44Z, replying to the coordinator's summary of the 0.6.0 release-prep scope; D2 deferred by Leif's choice 'Fix after 0.6.0' at 2026-11-12T14:46Z; relayed by claude-code-t370","recorded_by":"claude-code-t382","native":"specsync change approve prepare-the-chough-0-6-0-release-cargo-0-6-0-and-the-dated-changelog-with-the-dogfood-results-and-the-fixes-the-dogfood","actor":"0xLeif"}
  - by: claude-code (hc t382)
- 2026-11-12T14:53:38Z · direct-native-write · plan → plan ·  ^t-a5ea23
  - native_change: prepare-the-chough-0-6-0-release-cargo-0-6-0-and-the-dated-changelog-with-the-dogfood-results-and-the-fixes-the-dogfood
  - revision: 92ae99a6c8ea2d7679a76f95ac9846dfbca5c0fc
  - direct_write: {"change":"prepare-the-chough-0-6-0-release-cargo-0-6-0-and-the-dated-changelog-with-the-dogfood-results-and-the-fixes-the-dogfood","files":[{"path":"state.json","change":"changed"}],"before_state":"draft","after_state":"draft"}
  - by: claude-code (hc t382)
- 2026-11-12T14:53:38Z · advance · plan → build · Scope approved by Leif at the gate (relayed, run as 0xLeif) ^t-aaf8e7
  - native_change: prepare-the-chough-0-6-0-release-cargo-0-6-0-and-the-dated-changelog-with-the-dogfood-results-and-the-fixes-the-dogfood
  - gate: native:approved
  - revision: 92ae99a6c8ea2d7679a76f95ac9846dfbca5c0fc
  - by: claude-code (hc t382)

