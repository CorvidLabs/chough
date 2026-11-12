---
kind: session
id: 2026-11-06-size-the-checkpoint-from-measured-use-a-1-mib-cap-with-at-cap
aliases:
  - size-the-checkpoint-from-measured-use-a-1-mib-cap-with-at-cap-budgets-session-shelve-and-sessions-that-never-break-each
objective: "Size the checkpoint from measured use: 1 MiB cap, shelve, per-session robustness (DRIVE-14, #125)"
intent_scope: DRIVE
modules: [cli, history]
stage: done
stage_status: active
iteration: 1
callers: []
native_change: size-the-checkpoint-from-measured-use-a-1-mib-cap-with-at-cap-budgets-session-shelve-and-sessions-that-never-break-each
pr: ""
criteria: [DRIVE-14, DRIVE-13]
requirements: []
cases: []
design: []
files: []
observations:
  - OBS-tc-cli-checkpoint-at-the-cap-c30eb577a8558b5d26ad4336
  - OBS-tc-cli-full-stress-shape-ffec4f41f1cbc310aa65b2f4
  - OBS-tc-history-shelve-and-robust-sessions-1233fcc9c5892ef72419233f
bugs: []
blockers: []
waiting_on_human: []
run_consents: []
next: Wait for Leif to approve the scope, then implement
closed: 2026-11-06
outcome: done
receipts:
  - id: 2026-11-06-tc-cli-checkpoint-at-the-cap
    aliases: OBS-tc-cli-checkpoint-at-the-cap-c30eb577a8558b5d26ad4336
    claim: "Ran cargo test --locked --test next -- --exact a_checkpoint_at_the_cap_stays_fast_and_one_byte_more_is_refused for TC-cli-checkpoint-at-the-cap: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test next -- --exact a_checkpoint_at_the_cap_stays_fast_and_one_byte_more_is_refused
    revision: 9ce3cf486af5c78f9332242a0c91be85501ee1ea
    environment: local-linux-aarch64
    content: 17b65f2bd8de19fce4a1ff98f073821d6814a81d0873fa4ccd0439d8e9b63236
    fingerprint: sha256:f4e63597959252312bbf42dc2ea2dfdd4ecb10d9dd2d7e2f178099c39c43b715
    case: TC-cli-checkpoint-at-the-cap
    case_digest: 292e9a79f4ba866550d457bddefe9a1dbb7fa519bc8001af7f47b9720f33143a
    observed: 2026-11-06T03:27:21Z
    duration_seconds: 6
  - id: 2026-11-06-tc-cli-full-stress-shape
    aliases: OBS-tc-cli-full-stress-shape-ffec4f41f1cbc310aa65b2f4
    claim: "Ran cargo test --locked --test next -- --exact handoff_blocks_stay_within_the_cap_and_are_byte_identical for TC-cli-full-stress-shape: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test next -- --exact handoff_blocks_stay_within_the_cap_and_are_byte_identical
    revision: 9ce3cf486af5c78f9332242a0c91be85501ee1ea
    environment: local-linux-aarch64
    content: 17b65f2bd8de19fce4a1ff98f073821d6814a81d0873fa4ccd0439d8e9b63236
    fingerprint: sha256:d07cfffbe75ffb4056b4970841999bb9ab6fd19975c2b44a0a04aa68582fb123
    case: TC-cli-full-stress-shape
    case_digest: eccb140701a1f8e7b1f41bba0827ba3b64aca939ec142ea4f918df597b8a4922
    observed: 2026-11-06T03:27:26Z
    duration_seconds: 1
  - id: 2026-11-06-tc-history-shelve-and-robust-sessions
    aliases: OBS-tc-history-shelve-and-robust-sessions-1233fcc9c5892ef72419233f
    claim: "Ran cargo test --locked --test sessions -- shelv one_oversized_or_unreadable_session_does_not_break_the_others for TC-history-shelve-and-robust-sessions: passed (exit 0)."
    result: passed
    command: cargo test --locked --test sessions -- shelv one_oversized_or_unreadable_session_does_not_break_the_others
    revision: 9ce3cf486af5c78f9332242a0c91be85501ee1ea
    environment: local-linux-aarch64
    content: 17b65f2bd8de19fce4a1ff98f073821d6814a81d0873fa4ccd0439d8e9b63236
    fingerprint: sha256:b9a6182cfa7ff3c0b5d032209d9e68c84a418f2a08b0b91f98e4016537aae31f
    case: TC-history-shelve-and-robust-sessions
    case_digest: 6e344d54b52beb6ce63433fca2a24e83326eb5dcf3e63f1aedca5301fc60c5d7
    observed: 2026-11-06T03:27:30Z
    duration_seconds: 0
---
# Size the checkpoint from measured use: 1 MiB cap, shelve, per-session robustness (DRIVE-14, #125)

## Goal

Size the checkpoint from measured use: 1 MiB cap, shelve, per-session robustness (DRIVE-14, #125)

## Where it stands

## Decisions and why

- PR #133 (DRIVE-14, #125) accepted by Leif (0xLeif), 2026-11-05 20:33 MST through the orc: Record, merge, delete C3; his native review (pass) recorded at bb8154ca.

## What was verified

- OBS-tc-cli-checkpoint-at-the-cap-c30eb577a8558b5d26ad4336
- OBS-tc-cli-full-stress-shape-ffec4f41f1cbc310aa65b2f4
- OBS-tc-history-shelve-and-robust-sessions-1233fcc9c5892ef72419233f

## Open threads

## Timeline

- 2026-11-06T03:26:43Z · advance · intent → design · Plan approved natively (0xLeif, Approve, 2026-11-05 20:16 MST via the orc) ^t-df899f
  - native_change: size-the-checkpoint-from-measured-use-a-1-mib-cap-with-at-cap-budgets-session-shelve-and-sessions-that-never-break-each
  - revision: ed1e80bfdaa49cd4c12c84ffbd8d94cc016ab81a
- 2026-11-06T03:26:43Z · advance · design → plan · Plan approved natively (0xLeif, Approve, 2026-11-05 20:16 MST via the orc) ^t-e15822
  - native_change: size-the-checkpoint-from-measured-use-a-1-mib-cap-with-at-cap-budgets-session-shelve-and-sessions-that-never-break-each
  - revision: ed1e80bfdaa49cd4c12c84ffbd8d94cc016ab81a
- 2026-11-06T03:26:44Z · advance · plan → build · Plan approved natively (0xLeif, Approve, 2026-11-05 20:16 MST via the orc) ^t-85ab95
  - native_change: size-the-checkpoint-from-measured-use-a-1-mib-cap-with-at-cap-budgets-session-shelve-and-sessions-that-never-break-each
  - gate: native:approved
  - revision: ed1e80bfdaa49cd4c12c84ffbd8d94cc016ab81a
- 2026-11-06T03:27:11Z · advance · build → test · Native change verified (specsync change check --commit) ^t-eec61a
  - native_change: size-the-checkpoint-from-measured-use-a-1-mib-cap-with-at-cap-budgets-session-shelve-and-sessions-that-never-break-each
  - gate: native:verifying
  - revision: 9ce3cf486af5c78f9332242a0c91be85501ee1ea
- 2026-11-06T03:27:31Z · advance · test → accept · The three cap cases passed on Linux (local-linux-aarch64); the cap, next and sessions tests also pass on macOS; all new tests failed first ^t-fb2269
  - native_change: size-the-checkpoint-from-measured-use-a-1-mib-cap-with-at-cap-budgets-session-shelve-and-sessions-that-never-break-each
  - gate: native:verifying
  - revision: 9ce3cf486af5c78f9332242a0c91be85501ee1ea
- 2026-11-06T03:27:31Z · gate-opened · accept → accept · Native implementation review of this change by Leif (FLOW-10) ^t-27c60e
  - native_change: size-the-checkpoint-from-measured-use-a-1-mib-cap-with-at-cap-budgets-session-shelve-and-sessions-that-never-break-each
  - revision: 9ce3cf486af5c78f9332242a0c91be85501ee1ea
- 2026-11-06T03:34:16Z · gate-closed · accept → accept · Native implementation review of this change by Leif (FLOW-10) ^t-8f73fa
  - native_change: size-the-checkpoint-from-measured-use-a-1-mib-cap-with-at-cap-budgets-session-shelve-and-sessions-that-never-break-each
  - revision: 4dd1ebc5b4f58790fb74e54229dd8d963f4c1978
- 2026-11-06T03:34:19Z · advance · accept → done · Native change archived; Leif recorded his review ^t-567832
  - native_change: size-the-checkpoint-from-measured-use-a-1-mib-cap-with-at-cap-budgets-session-shelve-and-sessions-that-never-break-each
  - gate: native:archived
  - revision: 4dd1ebc5b4f58790fb74e54229dd8d963f4c1978


Closed 2026-11-06: done.
