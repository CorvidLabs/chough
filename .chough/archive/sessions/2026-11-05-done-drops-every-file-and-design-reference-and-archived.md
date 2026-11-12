---
kind: session
id: 2026-11-05-done-drops-every-file-and-design-reference-and-archived
aliases:
  - done-drops-every-file-and-design-reference-and-archived-sessions-hold-no-proof-by-them-know-21
objective: Done drops every file and design reference from the checkpoint, and retention ignores archived sessions references (KNOW-21)
intent_scope: KNOW
modules: [cli]
stage: done
stage_status: active
iteration: 1
callers: []
native_change: done-drops-every-file-and-design-reference-and-archived-sessions-hold-no-proof-by-them-know-21
pr: ""
criteria: [KNOW-21, KNOW-16]
requirements: []
cases: []
design: []
files: []
observations:
  - OBS-tc-cli-done-drops-file-and-design-refs-7fd113db7423ebb35826babb
  - OBS-tc-cli-archived-session-refs-protect-nothing-5436779791b5f92c5b6ee59c
bugs: []
blockers: []
waiting_on_human: []
run_consents: []
next: Wait for Leif to approve the definition, then implement against the two failing tests
closed: 2026-11-05
outcome: done
receipts:
  - id: 2026-11-05-tc-cli-done-drops-file-and-design-refs
    aliases: OBS-tc-cli-done-drops-file-and-design-refs-7fd113db7423ebb35826babb
    claim: "Ran cargo test --locked --test reduce -- --exact done_drops_every_file_and_design_reference for TC-cli-done-drops-file-and-design-refs: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test reduce -- --exact done_drops_every_file_and_design_reference
    revision: 894d94a3d31e46b4de8b65b45d74f36551d02f81
    environment: local-linux-aarch64
    content: 3cffd764d97ed57e83f23d8db9530a7cc278a09d567996d3be61b7832a83d507
    fingerprint: sha256:5e399856f79897f01d6e9cc2923c4c5e9c9682eab6b0a6b1f3c4865f5810489e
    case: TC-cli-done-drops-file-and-design-refs
    case_digest: 99d363548a3d8e65f636ab23750e2f77a46a92c625d8e87a17df435f96482a52
    observed: 2026-11-05T23:47:25Z
    duration_seconds: 0
  - id: 2026-11-05-tc-cli-archived-session-refs-protect-nothing
    aliases: OBS-tc-cli-archived-session-refs-protect-nothing-5436779791b5f92c5b6ee59c
    claim: "Ran cargo test --locked --test reduce -- --exact an_archived_session_file_reference_protects_nothing for TC-cli-archived-session-refs-protect-nothing: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test reduce -- --exact an_archived_session_file_reference_protects_nothing
    revision: 894d94a3d31e46b4de8b65b45d74f36551d02f81
    environment: local-linux-aarch64
    content: 3cffd764d97ed57e83f23d8db9530a7cc278a09d567996d3be61b7832a83d507
    fingerprint: sha256:a61e3f44149699fa33a72a153c7160fd0b5111a85dfd8d8e80213987fe72a24c
    case: TC-cli-archived-session-refs-protect-nothing
    case_digest: 69cf997734abaf2930ed5ec51883ffd84f72bf727ef0f22193f9f3ad2f2be2ab
    observed: 2026-11-05T23:47:28Z
    duration_seconds: 0
---
# Done drops every file and design reference from the checkpoint, and retention ignores archived sessions references (KNOW-21)

## Goal

Done drops every file and design reference from the checkpoint, and retention ignores archived sessions references (KNOW-21)

## Where it stands

## Decisions and why

- PR #120 (KNOW-21) accepted by Leif (0xLeif), 2026-11-05 16:51 MST through the orc: Record and merge; his native review (pass) recorded at a5625b8c.

## What was verified

- OBS-tc-cli-done-drops-file-and-design-refs-7fd113db7423ebb35826babb
- OBS-tc-cli-archived-session-refs-protect-nothing-5436779791b5f92c5b6ee59c

## Open threads

## Timeline

- 2026-11-05T23:46:50Z · advance · intent → design · Plan approved natively (0xLeif, Approve scope, 2026-11-05 16:44 MST via the orc) ^t-55bff9
  - native_change: done-drops-every-file-and-design-reference-and-archived-sessions-hold-no-proof-by-them-know-21
  - revision: 8880412c2743a2fb3de04d873d6f9d06965e5e8b
- 2026-11-05T23:46:50Z · advance · design → plan · Plan approved natively (0xLeif, Approve scope, 2026-11-05 16:44 MST via the orc) ^t-8d5a8f
  - native_change: done-drops-every-file-and-design-reference-and-archived-sessions-hold-no-proof-by-them-know-21
  - revision: 8880412c2743a2fb3de04d873d6f9d06965e5e8b
- 2026-11-05T23:46:50Z · advance · plan → build · Plan approved natively (0xLeif, Approve scope, 2026-11-05 16:44 MST via the orc) ^t-37a344
  - native_change: done-drops-every-file-and-design-reference-and-archived-sessions-hold-no-proof-by-them-know-21
  - gate: native:approved
  - revision: 8880412c2743a2fb3de04d873d6f9d06965e5e8b
- 2026-11-05T23:47:22Z · advance · build → test · Native change verified (specsync change check --commit) ^t-1ab4af
  - native_change: done-drops-every-file-and-design-reference-and-archived-sessions-hold-no-proof-by-them-know-21
  - gate: native:verifying
  - revision: 894d94a3d31e46b4de8b65b45d74f36551d02f81
- 2026-11-05T23:47:28Z · advance · test → accept · TC-cli-done-drops-file-and-design-refs and TC-cli-archived-session-refs-protect-nothing passed on Linux (local-linux-aarch64); both failed first on main ^t-85d2bd
  - native_change: done-drops-every-file-and-design-reference-and-archived-sessions-hold-no-proof-by-them-know-21
  - gate: native:verifying
  - revision: 894d94a3d31e46b4de8b65b45d74f36551d02f81
- 2026-11-05T23:47:28Z · gate-opened · accept → accept · Native implementation review of this change by Leif (FLOW-10) ^t-9dceab
  - native_change: done-drops-every-file-and-design-reference-and-archived-sessions-hold-no-proof-by-them-know-21
  - revision: 894d94a3d31e46b4de8b65b45d74f36551d02f81
- 2026-11-05T23:51:45Z · gate-closed · accept → accept · Native implementation review of this change by Leif (FLOW-10) ^t-091915
  - native_change: done-drops-every-file-and-design-reference-and-archived-sessions-hold-no-proof-by-them-know-21
  - revision: 79a3100cd536c20db22ae085c62888465cd2c2f2
- 2026-11-05T23:51:46Z · advance · accept → done · Native change archived; Leif recorded his review ^t-6799b6
  - native_change: done-drops-every-file-and-design-reference-and-archived-sessions-hold-no-proof-by-them-know-21
  - gate: native:archived
  - revision: 79a3100cd536c20db22ae085c62888465cd2c2f2


Closed 2026-11-05: done.
