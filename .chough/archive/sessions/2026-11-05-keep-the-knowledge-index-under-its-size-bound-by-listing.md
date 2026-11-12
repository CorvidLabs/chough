---
kind: session
id: 2026-11-05-keep-the-knowledge-index-under-its-size-bound-by-listing
aliases:
  - keep-the-knowledge-index-under-its-size-bound-by-listing-recent-archives-and-moving-the-full-list-to-a-monthly-timeline
objective: "Keep the knowledge index under its size bound: recent archives in INDEX.md, the full list on a monthly timeline (KNOW-28)"
intent_scope: KNOW
modules: [cli]
stage: done
stage_status: active
iteration: 1
callers: []
native_change: keep-the-knowledge-index-under-its-size-bound-by-listing-recent-archives-and-moving-the-full-list-to-a-monthly-timeline
pr: ""
criteria: [KNOW-28, KNOW-14]
requirements: []
cases: []
design: []
files: []
observations: [OBS-tc-cli-knowledge-index-bounded-timeline-7e7a8fe1e56dc73d605521c6]
bugs: []
blockers: []
waiting_on_human: []
run_consents: []
next: corvid-agent reviews the PR; Leif records the native implementation review, then finalize and archive
closed: 2026-11-05
outcome: done
receipts:
  - id: 2026-11-05-tc-cli-knowledge-index-bounded-timeline
    aliases: OBS-tc-cli-knowledge-index-bounded-timeline-7e7a8fe1e56dc73d605521c6
    claim: "Ran cargo test --locked --test vault -- --exact the_index_stays_bounded_and_a_monthly_timeline_lists_every_archive for TC-cli-knowledge-index-bounded-timeline: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test vault -- --exact the_index_stays_bounded_and_a_monthly_timeline_lists_every_archive
    revision: 6149a4c692abc9ed2d6077b3c1fc52c33a7a7e03
    environment: local-linux-aarch64
    content: ce534d71c17f68084f63fe4805e5f78a0b3ecf323af84f9fc738c08adb4b4d4a
    fingerprint: sha256:77b11a7dfe0444dfdb1feae6126cd7e3de8e70d707ac27e9bc16f2bbc7a3d690
    case: TC-cli-knowledge-index-bounded-timeline
    case_digest: 5cd755e6a7403659ad912b03397ccee08c725eb12add948d2cd216dc24c5cf8e
    observed: 2026-11-05T16:26:15Z
    duration_seconds: 1
---
# Keep the knowledge index under its size bound: recent archives in INDEX.md, the full list on a monthly timeline (KNOW-28)

## Goal

Keep the knowledge index under its size bound: recent archives in INDEX.md, the full list on a monthly timeline (KNOW-28)

## Where it stands

## Decisions and why

- PR #112 index-bound fix: Leif (0xLeif) recorded his native review (pass) at d4cdacd1 from the orc tab, 2026-11-05, under Widen #112 with the fix and Yes, record and merge.

## What was verified

- OBS-tc-cli-knowledge-index-bounded-timeline-7e7a8fe1e56dc73d605521c6

## Open threads

## Timeline

- 2026-11-05T16:23:49Z · advance · intent → design · Plan approved natively (0xLeif) ^t-5c904f
  - native_change: keep-the-knowledge-index-under-its-size-bound-by-listing-recent-archives-and-moving-the-full-list-to-a-monthly-timeline
  - revision: 7934800355522c0c5f31f1898c4720c0dae7f786
- 2026-11-05T16:23:49Z · advance · design → plan · Plan approved natively (0xLeif) ^t-81b914
  - native_change: keep-the-knowledge-index-under-its-size-bound-by-listing-recent-archives-and-moving-the-full-list-to-a-monthly-timeline
  - revision: 7934800355522c0c5f31f1898c4720c0dae7f786
- 2026-11-05T16:23:50Z · advance · plan → build · Plan approved natively (0xLeif) ^t-295c31
  - native_change: keep-the-knowledge-index-under-its-size-bound-by-listing-recent-archives-and-moving-the-full-list-to-a-monthly-timeline
  - gate: native:approved
  - revision: 7934800355522c0c5f31f1898c4720c0dae7f786
- 2026-11-05T16:26:11Z · advance · build → test · Native change verified (specsync change check --commit) ^t-2788e3
  - native_change: keep-the-knowledge-index-under-its-size-bound-by-listing-recent-archives-and-moving-the-full-list-to-a-monthly-timeline
  - gate: native:verifying
  - revision: 6149a4c692abc9ed2d6077b3c1fc52c33a7a7e03
- 2026-11-05T16:26:16Z · advance · test → accept · Cases for this change passed on Linux (local-linux-aarch64) ^t-d8a98c
  - native_change: keep-the-knowledge-index-under-its-size-bound-by-listing-recent-archives-and-moving-the-full-list-to-a-monthly-timeline
  - gate: native:verifying
  - revision: 6149a4c692abc9ed2d6077b3c1fc52c33a7a7e03
- 2026-11-05T16:26:16Z · gate-opened · accept → accept · Native implementation review of this change by Leif (FLOW-10) ^t-6e23fe
  - native_change: keep-the-knowledge-index-under-its-size-bound-by-listing-recent-archives-and-moving-the-full-list-to-a-monthly-timeline
  - revision: 6149a4c692abc9ed2d6077b3c1fc52c33a7a7e03
- 2026-11-05T16:33:33Z · gate-closed · accept → accept · Native implementation review of this change by Leif (FLOW-10) ^t-3cb630
  - native_change: keep-the-knowledge-index-under-its-size-bound-by-listing-recent-archives-and-moving-the-full-list-to-a-monthly-timeline
  - revision: d9beb9fd80545283c3c47c40d56c087d4931bbe8
- 2026-11-05T16:34:07Z · advance · accept → done · Native change archived; Leif recorded his review ^t-2e3d17
  - native_change: keep-the-knowledge-index-under-its-size-bound-by-listing-recent-archives-and-moving-the-full-list-to-a-monthly-timeline
  - gate: native:archived
  - revision: d9beb9fd80545283c3c47c40d56c087d4931bbe8


Closed 2026-11-05: done.
