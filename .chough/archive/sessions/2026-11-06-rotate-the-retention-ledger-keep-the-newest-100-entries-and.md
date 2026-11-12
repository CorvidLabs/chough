---
kind: session
id: 2026-11-06-rotate-the-retention-ledger-keep-the-newest-100-entries-and
aliases:
  - rotate-the-retention-ledger-keep-the-newest-100-entries-and-move-older-summarized-ones-into-the-vault-with-their
objective: "Rotate the retention ledger: keep the newest 100 entries, move older summarized ones into the vault with their archive (KNOW-23)"
intent_scope: KNOW
modules: [cli]
stage: done
stage_status: active
iteration: 1
callers: []
native_change: rotate-the-retention-ledger-keep-the-newest-100-entries-and-move-older-summarized-ones-into-the-vault-with-their
pr: ""
criteria: [KNOW-23]
requirements: []
cases: []
design: []
files: []
observations:
  - OBS-tc-cli-ledger-keeps-newest-and-moves-the-rest-5d279876a8ea8bfd6438b66f
  - OBS-tc-cli-moved-ledger-entries-are-read-47d1a2d0928e050a74bd876f
  - OBS-tc-cli-rotation-leftover-read-once-075a83421952b2deb06c02de
bugs: []
blockers: []
waiting_on_human: []
run_consents: []
next: Wait for Leif to approve the definition and N, then implement against the failing tests
closed: 2026-11-06
outcome: done
receipts:
  - id: 2026-11-06-tc-cli-ledger-keeps-newest-and-moves-the-rest
    aliases: OBS-tc-cli-ledger-keeps-newest-and-moves-the-rest-5d279876a8ea8bfd6438b66f
    claim: "Ran cargo test --locked --test reduce -- --exact done_keeps_the_newest_ledger_entries_and_moves_the_rest_into_the_vault for TC-cli-ledger-keeps-newest-and-moves-the-rest: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test reduce -- --exact done_keeps_the_newest_ledger_entries_and_moves_the_rest_into_the_vault
    revision: 3b55b5d0381f2063f46f4739a6fda0b068c99d40
    environment: local-linux-aarch64
    content: 22990fa72080f363cc9b746fc377814d591f7489dcd055b1f2f6352bb6cafe75
    fingerprint: sha256:1539e9931db7db53761f218f0a14661d3a12545c9e71d2659b1f9bc5d8a332ba
    case: TC-cli-ledger-keeps-newest-and-moves-the-rest
    case_digest: b49f23b1a6e3173419d729133e3a3ba8f6fc2c470cd4d1f7d8b781850507090c
    observed: 2026-11-06T01:41:08Z
    duration_seconds: 0
  - id: 2026-11-06-tc-cli-moved-ledger-entries-are-read
    aliases: OBS-tc-cli-moved-ledger-entries-are-read-47d1a2d0928e050a74bd876f
    claim: "Ran cargo test --locked --test reduce -- --exact moved_ledger_entries_are_still_read for TC-cli-moved-ledger-entries-are-read: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test reduce -- --exact moved_ledger_entries_are_still_read
    revision: 3b55b5d0381f2063f46f4739a6fda0b068c99d40
    environment: local-linux-aarch64
    content: 22990fa72080f363cc9b746fc377814d591f7489dcd055b1f2f6352bb6cafe75
    fingerprint: sha256:9697ff80ee31b1f55dd1333dd192bb0a5caba122ea6eb0fedc3c53d8f225dd1c
    case: TC-cli-moved-ledger-entries-are-read
    case_digest: 48c7e5d0b1cfdb9268d1a6a9f36aea8053cb7243e2233fab9371d15e3f410076
    observed: 2026-11-06T01:41:12Z
    duration_seconds: 0
  - id: 2026-11-06-tc-cli-rotation-leftover-read-once
    aliases: OBS-tc-cli-rotation-leftover-read-once-075a83421952b2deb06c02de
    claim: "Ran cargo test --locked --test reduce -- --exact an_interrupted_rotation_leftover_is_read_once for TC-cli-rotation-leftover-read-once: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test reduce -- --exact an_interrupted_rotation_leftover_is_read_once
    revision: 3b55b5d0381f2063f46f4739a6fda0b068c99d40
    environment: local-linux-aarch64
    content: 22990fa72080f363cc9b746fc377814d591f7489dcd055b1f2f6352bb6cafe75
    fingerprint: sha256:67e40bbf3a857ca78ed98eedbf02a28d26f1a6d7ebd4e8e7450deba39839a931
    case: TC-cli-rotation-leftover-read-once
    case_digest: dd8dda2e3f452cf9539e4d592eef608a31b89975946c518865d73104b2048dbb
    observed: 2026-11-06T01:41:16Z
    duration_seconds: 0
---
# Rotate the retention ledger: keep the newest 100 entries, move older summarized ones into the vault with their archive (KNOW-23)

## Goal

Rotate the retention ledger: keep the newest 100 entries, move older summarized ones into the vault with their archive (KNOW-23)

## Where it stands

## Decisions and why

- PR #127 (KNOW-23) accepted by Leif (0xLeif), 2026-11-05 18:45 MST through the orc: Record and merge; his native review (pass) recorded at 5837c9b4.

## What was verified

- OBS-tc-cli-ledger-keeps-newest-and-moves-the-rest-5d279876a8ea8bfd6438b66f
- OBS-tc-cli-moved-ledger-entries-are-read-47d1a2d0928e050a74bd876f
- OBS-tc-cli-rotation-leftover-read-once-075a83421952b2deb06c02de

## Open threads

## Timeline

- 2026-11-06T01:40:56Z · advance · intent → design · Plan approved natively (0xLeif: N=100 18:28 MST, Re-approve 18:39 MST, via the orc) ^t-2c555e
  - native_change: rotate-the-retention-ledger-keep-the-newest-100-entries-and-move-older-summarized-ones-into-the-vault-with-their
  - revision: f27d44be2fb71df20ba0019ee55190c37ef857e2
- 2026-11-06T01:40:56Z · advance · design → plan · Plan approved natively (0xLeif: N=100 18:28 MST, Re-approve 18:39 MST, via the orc) ^t-25c79d
  - native_change: rotate-the-retention-ledger-keep-the-newest-100-entries-and-move-older-summarized-ones-into-the-vault-with-their
  - revision: f27d44be2fb71df20ba0019ee55190c37ef857e2
- 2026-11-06T01:40:57Z · advance · plan → build · Plan approved natively (0xLeif: N=100 18:28 MST, Re-approve 18:39 MST, via the orc) ^t-b9a998
  - native_change: rotate-the-retention-ledger-keep-the-newest-100-entries-and-move-older-summarized-ones-into-the-vault-with-their
  - gate: native:approved
  - revision: f27d44be2fb71df20ba0019ee55190c37ef857e2
- 2026-11-06T01:41:03Z · advance · build → test · Native change verified (specsync change check --commit) ^t-fa25c6
  - native_change: rotate-the-retention-ledger-keep-the-newest-100-entries-and-move-older-summarized-ones-into-the-vault-with-their
  - gate: native:verifying
  - revision: 3b55b5d0381f2063f46f4739a6fda0b068c99d40
- 2026-11-06T01:41:17Z · advance · test → accept · The three KNOW-23 cases passed on Linux (local-linux-aarch64); all failed first on main ^t-5d52ce
  - native_change: rotate-the-retention-ledger-keep-the-newest-100-entries-and-move-older-summarized-ones-into-the-vault-with-their
  - gate: native:verifying
  - revision: 3b55b5d0381f2063f46f4739a6fda0b068c99d40
- 2026-11-06T01:41:17Z · gate-opened · accept → accept · Native implementation review of this change by Leif (FLOW-10) ^t-f4b74d
  - native_change: rotate-the-retention-ledger-keep-the-newest-100-entries-and-move-older-summarized-ones-into-the-vault-with-their
  - revision: 3b55b5d0381f2063f46f4739a6fda0b068c99d40
- 2026-11-06T01:46:34Z · gate-closed · accept → accept · Native implementation review of this change by Leif (FLOW-10) ^t-359a1d
  - native_change: rotate-the-retention-ledger-keep-the-newest-100-entries-and-move-older-summarized-ones-into-the-vault-with-their
  - revision: 54e9ce1f75d59de368b639993f5631a094ac117f
- 2026-11-06T01:46:41Z · advance · accept → done · Native change archived; Leif recorded his review ^t-79c740
  - native_change: rotate-the-retention-ledger-keep-the-newest-100-entries-and-move-older-summarized-ones-into-the-vault-with-their
  - gate: native:archived
  - revision: 54e9ce1f75d59de368b639993f5631a094ac117f


Closed 2026-11-06: done.
