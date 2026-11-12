---
kind: session
id: 2026-11-05-spread-the-specsync-archive-history-walk-across-the-ci-shards
aliases:
  - spread-the-specsync-archive-history-walk-across-the-ci-shards-so-a-long-pr-fits-the-ten-minute-limit-105
objective: Spread the SpecSync Archive history walk across the four CI shards so a long PR fits the unchanged ten-minute limit (#105)
intent_scope: DRIVE
modules: [cli]
stage: done
stage_status: active
iteration: 1
callers: []
native_change: spread-the-specsync-archive-history-walk-across-the-ci-shards-so-a-long-pr-fits-the-ten-minute-limit-105
pr: ""
criteria: [DRIVE-10]
requirements: []
cases: []
design: []
files: []
observations: []
bugs: []
blockers: []
waiting_on_human: []
run_consents: []
next: Split the history walk with shard_revisions against the failing fixtures
closed: 2026-11-05
outcome: done
receipts: []
---
# Spread the SpecSync Archive history walk across the four CI shards so a long PR fits the unchanged ten-minute limit (#105)

## Goal

Spread the SpecSync Archive history walk across the four CI shards so a long PR fits the unchanged ten-minute limit (#105)

## Where it stands

## Decisions and why

- PR #118 (#105) accepted by Leif (0xLeif), 2026-11-05 15:28 MST through the orc: Record and merge; his native review (pass) recorded at 188f96b2. He also chose Yes, draft PR for proof for a hosted run on the long #99 candidate.

## What was verified

## Open threads

## Timeline

- 2026-11-05T22:13:27Z · advance · intent → design · Plan approved natively (0xLeif) ^t-d14bfd
  - native_change: spread-the-specsync-archive-history-walk-across-the-ci-shards-so-a-long-pr-fits-the-ten-minute-limit-105
  - revision: a688800ee100a77c50119e2ce17530c64a1e0fe8
- 2026-11-05T22:13:27Z · advance · design → plan · Plan approved natively (0xLeif) ^t-cf2b5c
  - native_change: spread-the-specsync-archive-history-walk-across-the-ci-shards-so-a-long-pr-fits-the-ten-minute-limit-105
  - revision: a688800ee100a77c50119e2ce17530c64a1e0fe8
- 2026-11-05T22:13:27Z · advance · plan → build · Plan approved natively (0xLeif) ^t-ca87ee
  - native_change: spread-the-specsync-archive-history-walk-across-the-ci-shards-so-a-long-pr-fits-the-ten-minute-limit-105
  - gate: native:approved
  - revision: a688800ee100a77c50119e2ce17530c64a1e0fe8
- 2026-11-05T22:20:39Z · advance · build → test · Native change verified (specsync change check --commit) ^t-ce4a33
  - native_change: spread-the-specsync-archive-history-walk-across-the-ci-shards-so-a-long-pr-fits-the-ten-minute-limit-105
  - gate: native:verifying
  - revision: 15ed6390f2f001cbce2b11d7a95da24e0bcf0702
- 2026-11-05T22:20:40Z · advance · test → accept · Archive fixtures pass on Linux (local-linux-aarch64): Ran 25 tests, OK, including the 3 new #105 tests that failed first; long #99 candidate measured: slowest shard 88.8 s against 268.6 s ^t-b2d62b
  - native_change: spread-the-specsync-archive-history-walk-across-the-ci-shards-so-a-long-pr-fits-the-ten-minute-limit-105
  - gate: native:verifying
  - revision: 15ed6390f2f001cbce2b11d7a95da24e0bcf0702
- 2026-11-05T22:20:40Z · gate-opened · accept → accept · Native implementation review of this change by Leif (FLOW-10) ^t-415d92
  - native_change: spread-the-specsync-archive-history-walk-across-the-ci-shards-so-a-long-pr-fits-the-ten-minute-limit-105
  - revision: 15ed6390f2f001cbce2b11d7a95da24e0bcf0702
- 2026-11-05T22:29:21Z · gate-closed · accept → accept · Native implementation review of this change by Leif (FLOW-10) ^t-815b3c
  - native_change: spread-the-specsync-archive-history-walk-across-the-ci-shards-so-a-long-pr-fits-the-ten-minute-limit-105
  - revision: 722c2dca39c6ae30051c3b1bdf63035edb613caa
- 2026-11-05T22:29:25Z · advance · accept → done · Native change archived; Leif recorded his review ^t-b55522
  - native_change: spread-the-specsync-archive-history-walk-across-the-ci-shards-so-a-long-pr-fits-the-ten-minute-limit-105
  - gate: native:archived
  - revision: 722c2dca39c6ae30051c3b1bdf63035edb613caa


Closed 2026-11-05: done.
