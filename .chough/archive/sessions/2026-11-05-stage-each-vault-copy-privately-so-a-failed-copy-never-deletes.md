---
kind: session
id: 2026-11-05-stage-each-vault-copy-privately-so-a-failed-copy-never-deletes
aliases:
  - stage-each-vault-copy-privately-so-a-failed-copy-never-deletes-another-writer-bytes-115-review
objective: Stage vault copies privately so a failed copy never deletes another writer's bytes (#115 review P1)
intent_scope: KNOW
modules: [cli]
stage: done
stage_status: active
iteration: 1
callers: []
native_change: stage-each-vault-copy-privately-so-a-failed-copy-never-deletes-another-writer-bytes-115-review
pr: ""
criteria: [KNOW-10, KNOW-19]
requirements: []
cases: []
design: []
files: []
observations: [OBS-tc-cli-failed-copy-keeps-other-writer-ac77298375f085b47ae7a1d3]
bugs: []
blockers: []
waiting_on_human: []
run_consents: []
next: Stage copies privately and link them into place against the failing unit test
closed: 2026-11-05
outcome: done
receipts:
  - id: 2026-11-05-tc-cli-failed-copy-keeps-other-writer
    aliases: OBS-tc-cli-failed-copy-keeps-other-writer-ac77298375f085b47ae7a1d3
    claim: "Ran cargo test --locked --bin chough -- --exact evidence::tests::a_failed_copy_never_deletes_another_writers_bytes for TC-cli-failed-copy-keeps-other-writer: passed (exit 0)."
    result: passed
    command: cargo test --locked --bin chough -- --exact evidence::tests::a_failed_copy_never_deletes_another_writers_bytes
    revision: 8a87a1d468beae0778e2ee34d596ca905d9b5953
    environment: local-linux-aarch64
    content: b87d3fe8f9e7b0763cb6d66671ffef5ec8de037a9b9e1b1253d80a5763597c50
    fingerprint: sha256:c8ac8b69b1e94e64068ad3890d2db7f7d2bb6f6b0e154563559bcbe6774a8eab
    case: TC-cli-failed-copy-keeps-other-writer
    case_digest: 89906c5eb725aafadfd235095153191f8cf1b26b4e7f55c798bda9ad238e225e
    observed: 2026-11-05T17:54:16Z
    duration_seconds: 0
---
# Stage vault copies privately so a failed copy never deletes another writer's bytes (#115 review P1)

## Goal

Stage vault copies privately so a failed copy never deletes another writer's bytes (#115 review P1)

## Where it stands

## Decisions and why

- PR #115 staging fix: Leif (0xLeif) recorded his native review (pass) at e3f6aa44 through the orc, 2026-11-05 10:57 MST: Record and merge.

## What was verified

- OBS-tc-cli-failed-copy-keeps-other-writer-ac77298375f085b47ae7a1d3

## Open threads

## Timeline

- 2026-11-05T17:52:33Z · advance · intent → design · Plan approved natively (0xLeif) ^t-31dc13
  - native_change: stage-each-vault-copy-privately-so-a-failed-copy-never-deletes-another-writer-bytes-115-review
  - revision: a6cfd3298d70b8b3ce6e2de92a25c817c4cbb324
- 2026-11-05T17:52:33Z · advance · design → plan · Plan approved natively (0xLeif) ^t-e2e346
  - native_change: stage-each-vault-copy-privately-so-a-failed-copy-never-deletes-another-writer-bytes-115-review
  - revision: a6cfd3298d70b8b3ce6e2de92a25c817c4cbb324
- 2026-11-05T17:52:33Z · advance · plan → build · Plan approved natively (0xLeif) ^t-452510
  - native_change: stage-each-vault-copy-privately-so-a-failed-copy-never-deletes-another-writer-bytes-115-review
  - gate: native:approved
  - revision: a6cfd3298d70b8b3ce6e2de92a25c817c4cbb324
- 2026-11-05T17:54:12Z · advance · build → test · Native change verified (specsync change check --commit) ^t-7dc367
  - native_change: stage-each-vault-copy-privately-so-a-failed-copy-never-deletes-another-writer-bytes-115-review
  - gate: native:verifying
  - revision: 8a87a1d468beae0778e2ee34d596ca905d9b5953
- 2026-11-05T17:54:17Z · advance · test → accept · Cases for this change passed on Linux (local-linux-aarch64) ^t-7a97b5
  - native_change: stage-each-vault-copy-privately-so-a-failed-copy-never-deletes-another-writer-bytes-115-review
  - gate: native:verifying
  - revision: 8a87a1d468beae0778e2ee34d596ca905d9b5953
- 2026-11-05T17:54:17Z · gate-opened · accept → accept · Native implementation review of this change by Leif (FLOW-10) ^t-67d872
  - native_change: stage-each-vault-copy-privately-so-a-failed-copy-never-deletes-another-writer-bytes-115-review
  - revision: 8a87a1d468beae0778e2ee34d596ca905d9b5953
- 2026-11-05T17:57:41Z · gate-closed · accept → accept · Native implementation review of this change by Leif (FLOW-10) ^t-020cfc
  - native_change: stage-each-vault-copy-privately-so-a-failed-copy-never-deletes-another-writer-bytes-115-review
  - revision: 1fac64bcdf3cc15fea6ea97416932aa36d575a99
- 2026-11-05T17:57:42Z · advance · accept → done · Native change archived; Leif recorded his review ^t-123cde
  - native_change: stage-each-vault-copy-privately-so-a-failed-copy-never-deletes-another-writer-bytes-115-review
  - gate: native:archived
  - revision: 1fac64bcdf3cc15fea6ea97416932aa36d575a99


Closed 2026-11-05: done.
