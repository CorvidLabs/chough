---
kind: session
id: 2026-11-12-release-0-6-0
aliases: [release-0-6-0]
objective: "Prepare the Chough 0.6.0 release as the dogfood cycle, driven with the 0.6.0 build only: Cargo 0.6.0 and the dated CHANGELOG [0.6.0] with the dogfood results; fix what the dogfood found (D1, the pointer naming a migrated key); clear the CodeQL false positive; close the KNOW gaps (#216) and #148 through Chough; settle the four flaky-test bugs"
intent_scope: ""
modules: []
stage: plan
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
observations: []
bugs: []
blockers: []
waiting_on_human:
  - gate: Approve the scope of the 0.6.0 release preparation
    holds: plan
    drives: approve
    opened: 2026-11-12T14:42:58Z
run_consents: []
next: "Wait for Leif's scope decision at the gate; then build: D1 red then green first"
---
# Prepare the Chough 0.6.0 release as the dogfood cycle, driven with the 0.6.0 build only: Cargo 0.6.0 and the dated CHANGELOG [0.6.0] with the dogfood results; fix what the dogfood found (D1, the pointer naming a migrated key); clear the CodeQL false positive; close the KNOW gaps (#216) and #148 through Chough; settle the four flaky-test bugs

## Goal

Prepare the Chough 0.6.0 release as the dogfood cycle, driven with the 0.6.0 build only: Cargo 0.6.0 and the dated CHANGELOG [0.6.0] with the dogfood results; fix what the dogfood found (D1, the pointer naming a migrated key); clear the CodeQL false positive; close the KNOW gaps (#216) and #148 through Chough; settle the four flaky-test bugs

## Where it stands

## Decisions and why

- decided: the 0.6.0 release preparation is the dogfood cycle, one small native change driven with the 0.6.0 build only, scoped by t370 (relaying Leif's plan): Cargo 0.6.0 and the dated CHANGELOG with dogfood results, D1, the CodeQL helper rename, the KNOW case links and runs, #148 red then green, and settling the four flaky-test bugs; no tag, release or merge without Leif

## What was verified

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

