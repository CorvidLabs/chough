---
kind: session
id: 2026-11-05-mark-the-0-4-0-release-published-in-the-roadmap-with-its
aliases: [mark-the-0-4-0-release-published-in-the-roadmap-with-its-install-observation]
objective: Mark Chough 0.4.0 published in the roadmap with its clean install observation
intent_scope: LIFE
modules: [cli]
stage: done
stage_status: active
iteration: 1
callers: []
native_change: mark-the-0-4-0-release-published-in-the-roadmap-with-its-install-observation
pr: ""
criteria: [LIFE-1]
requirements: []
cases: []
design: []
files: []
observations: [OBS-installation-observation-eed34b5fe7378f8d4bf93fa4]
bugs: []
blockers: []
waiting_on_human: []
run_consents: []
next: Record the install observation, verify, and open the docs PR
closed: 2026-11-05
outcome: done
receipts:
  - id: 2026-11-05-clean-cargo-install-chough-version-0-4-0-locked-into-an-empty
    aliases: OBS-installation-observation-eed34b5fe7378f8d4bf93fa4
    claim: Clean cargo install chough --version 0.4.0 --locked into an empty root on macOS 26.5.2 arm64 with cargo 1.95.0 exited 0; the installed chough --version printed chough 0.4.0.
    result: passed
    revision: 2082dddff4204a93c7539e39d3c4ede385178b02
    environment: macos-arm64-cargo-1.95.0
    content: 82186ba9f5ca9e57ef1247e951d6bc5869d410126907e329b4c2864912084227
    fingerprint: sha256:d57071857d5f71e74d1bccd2f91540f1ac869ec1fea72768839e79b554d37293
    observed: 2026-11-05T19:09:59Z
---
# Mark Chough 0.4.0 published in the roadmap with its clean install observation

## Goal

Mark Chough 0.4.0 published in the roadmap with its clean install observation

## Where it stands

## Decisions and why

- ROADMAP 0.4.0 change (#117) accepted by Leif (0xLeif): Record and merge, 2026-11-05 via the orc; his native review (pass) recorded at 844f0266.

## What was verified

- OBS-installation-observation-eed34b5fe7378f8d4bf93fa4

## Open threads

## Timeline

- 2026-11-05T19:09:51Z · advance · intent → design · Plan approved natively (0xLeif) ^t-75dd83
  - native_change: mark-the-0-4-0-release-published-in-the-roadmap-with-its-install-observation
  - revision: 69950c7a9253230aeeee3bac0d0d6b580e291aad
- 2026-11-05T19:09:52Z · advance · design → plan · Plan approved natively (0xLeif) ^t-4513e6
  - native_change: mark-the-0-4-0-release-published-in-the-roadmap-with-its-install-observation
  - revision: 69950c7a9253230aeeee3bac0d0d6b580e291aad
- 2026-11-05T19:09:52Z · advance · plan → build · Plan approved natively (0xLeif) ^t-d0400e
  - native_change: mark-the-0-4-0-release-published-in-the-roadmap-with-its-install-observation
  - gate: native:approved
  - revision: 69950c7a9253230aeeee3bac0d0d6b580e291aad
- 2026-11-05T19:09:59Z · advance · build → test · Native change verified (specsync change check --commit) ^t-454889
  - native_change: mark-the-0-4-0-release-published-in-the-roadmap-with-its-install-observation
  - gate: native:verifying
  - revision: 2082dddff4204a93c7539e39d3c4ede385178b02
- 2026-11-05T19:10:04Z · advance · test → accept · Clean install observed on macOS arm64 ^t-8c244b
  - native_change: mark-the-0-4-0-release-published-in-the-roadmap-with-its-install-observation
  - gate: native:verifying
  - revision: 2082dddff4204a93c7539e39d3c4ede385178b02
- 2026-11-05T19:10:04Z · gate-opened · accept → accept · Native implementation review of this change by Leif (FLOW-10) ^t-085566
  - native_change: mark-the-0-4-0-release-published-in-the-roadmap-with-its-install-observation
  - revision: 2082dddff4204a93c7539e39d3c4ede385178b02
- 2026-11-05T21:41:49Z · gate-closed · accept → accept · Native implementation review of this change by Leif (FLOW-10) ^t-1ce66d
  - native_change: mark-the-0-4-0-release-published-in-the-roadmap-with-its-install-observation
  - revision: 25ec6a34b77e2e9c2e131762c7afeed039eba83a
- 2026-11-05T21:41:52Z · advance · accept → done · Native change archived; Leif recorded his review ^t-2be1d1
  - native_change: mark-the-0-4-0-release-published-in-the-roadmap-with-its-install-observation
  - gate: native:archived
  - revision: 25ec6a34b77e2e9c2e131762c7afeed039eba83a


Closed 2026-11-05: done.
