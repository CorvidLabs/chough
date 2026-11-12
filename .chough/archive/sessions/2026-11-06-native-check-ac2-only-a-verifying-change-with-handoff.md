---
kind: session
id: 2026-11-06-native-check-ac2-only-a-verifying-change-with-handoff
aliases:
  - native-check-ac2-only-a-verifying-change-with-handoff-readiness-safe-shows-no-note-126-d10
objective: Native check AC2 requires safe verifying readiness to suppress the note (#126, audit D10)
intent_scope: LIFE
modules: [cli]
stage: done
stage_status: active
iteration: 1
callers: []
native_change: native-check-ac2-only-a-verifying-change-with-handoff-readiness-safe-shows-no-note-126-d10
pr: ""
criteria: [LIFE-7, LIFE-3]
requirements: [REQ-cli-039]
cases: [TC-cli-native-check-safe-verifying, TC-cli-native-check-not-passed-note]
design: []
files: []
observations:
  - OBS-tc-cli-native-check-safe-verifying-8b455b7140c543c7009cea93
  - OBS-tc-cli-native-check-not-passed-note-4df482e80586149e4a0c3dbc
bugs:
  - BUG-rebuilding-the-knowledge-index-after-recording-an-observation-makes-tha-1e3c3ae5fbe4e8987cf025ef
blockers: []
waiting_on_human: []
run_consents: []
next: Push the archive, observe hosted CI at the tip, and merge on green plus corvid-agent approval of the exact head, pinned with --match-head-commit
closed: 2026-11-06
outcome: done
receipts:
  - id: 2026-11-06-tc-cli-native-check-safe-verifying
    aliases: OBS-tc-cli-native-check-safe-verifying-8b455b7140c543c7009cea93
    claim: "Ran cargo test --locked --test native_check -- --exact req_cli_039_criteria_clear_the_note_only_for_safe_verifying_readiness for TC-cli-native-check-safe-verifying: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test native_check -- --exact req_cli_039_criteria_clear_the_note_only_for_safe_verifying_readiness
    revision: e4e8371f9e8ec4a96ff35f030d78414b3ee9fb96
    environment: local-macos-arm64
    content: 91bb6cbf5307dfe722c0c17a0d2bff75e4a3a1c8073546ba3a5f4a313128cf7e
    fingerprint: sha256:b805b5283a231e90abaf7147b7d251ca22a81acf35c9fd43870c480fd24ed5bc
    case: TC-cli-native-check-safe-verifying
    case_digest: 910b57b35ae22551ae92737fffbc27551c96667183cbe6c98d7b8dea9898bd76
    observed: 2026-11-06T03:30:28Z
    duration_seconds: 0
  - id: 2026-11-06-tc-cli-native-check-not-passed-note
    aliases: OBS-tc-cli-native-check-not-passed-note-4df482e80586149e4a0c3dbc
    claim: "Ran cargo test --locked --test native_check -- --exact next_says_when_the_native_check_has_not_passed_for_this_tree for TC-cli-native-check-not-passed-note: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test native_check -- --exact next_says_when_the_native_check_has_not_passed_for_this_tree
    revision: e4e8371f9e8ec4a96ff35f030d78414b3ee9fb96
    environment: local-macos-arm64
    content: 91bb6cbf5307dfe722c0c17a0d2bff75e4a3a1c8073546ba3a5f4a313128cf7e
    fingerprint: sha256:b1d9c297cbddf263b4c5512f24506d4da48cee195b29687f8f85f68159daeb74
    case: TC-cli-native-check-not-passed-note
    case_digest: 8b2176339387b98c629440896e988458004de9dbd3e0df6173ec2ee05b872dc6
    observed: 2026-11-06T03:30:34Z
    duration_seconds: 2
---
# Native check AC2 requires safe verifying readiness to suppress the note (#126, audit D10)

## Goal

Native check AC2 requires safe verifying readiness to suppress the note (#126, audit D10)

## Where it stands

- Change package drafted and the wording test req_cli_039_criteria_clear_the_note_only_for_safe_verifying_readiness fails first on AC2's clause 'when verifying, it does not.'
- REQ-cli-039 AC2 materialized (cli version 58); CHANGELOG line added; cases defined in .chough/cases/cli.native-check-safe-verifying.cases.toml
- Linux (chough-linux:3, aarch64, specsync 6.2.0) verify at 01aebd6c passed fmt, lint, test, intent, spec, audit and cases, and stopped at knowledge: INDEX.md had to list the two new observations

## Decisions and why

- Agent's reading, not a human decision: #126 and plan C4 ask only for AC2 wording; the REQ-cli-039 body, the other criteria and the runtime stay as they are.
- Leif (0xLeif), 2026-11-05 20:26 MST, through the orc (t18): 'Approve' for the C4 definition; the orc recorded the native approval on 11083b7f.
- failed: keeping the macOS case observations current at the PR head because the required knowledge/INDEX.md rebuild lists them and changes the content identity; recorded as BUG-rebuilding-the-knowledge-index-after-recording-an-observation-makes-tha-1e3c3ae5fbe4e8987cf025ef (out of scope for C4)
- PR #134 (C4, #126) accepted by Leif (0xLeif), 2026-11-05 20:58 MST through the orc (t18): 'Record and merge'; the orc recorded the native review (pass) at 1da231f4.

## What was verified

- OBS-tc-cli-native-check-safe-verifying-8b455b7140c543c7009cea93
- OBS-tc-cli-native-check-not-passed-note-4df482e80586149e4a0c3dbc

## Open threads

## Timeline

- 2026-11-06T03:19:01Z · gate-opened · intent → intent · Scope decision of native-check-ac2-only-a-verifying-change-with-handoff-readiness-safe-shows-no-note-126-d10 ^t-c68318
  - native_change: native-check-ac2-only-a-verifying-change-with-handoff-readiness-safe-shows-no-note-126-d10
  - revision: ec830cfb001cc5f71f4f83809ca3294d685f42a4
- 2026-11-06T03:26:55Z · gate-closed · intent → intent · Scope decision of native-check-ac2-only-a-verifying-change-with-handoff-readiness-safe-shows-no-note-126-d10 ^t-9f722f
  - native_change: native-check-ac2-only-a-verifying-change-with-handoff-readiness-safe-shows-no-note-126-d10
  - revision: 11083b7f0423b88e48752a38411f2b4af4171417
- 2026-11-06T03:26:55Z · advance · intent → design ·  ^t-16592d
  - native_change: native-check-ac2-only-a-verifying-change-with-handoff-readiness-safe-shows-no-note-126-d10
  - revision: 11083b7f0423b88e48752a38411f2b4af4171417
- 2026-11-06T03:26:55Z · skip · design → plan · Design not applicable: a requirement wording repair with no visuals or interactions ^t-a807fc
  - native_change: native-check-ac2-only-a-verifying-change-with-handoff-readiness-safe-shows-no-note-126-d10
  - revision: 11083b7f0423b88e48752a38411f2b4af4171417
- 2026-11-06T03:26:55Z · advance · plan → build · native:approved ^t-cefb63
  - native_change: native-check-ac2-only-a-verifying-change-with-handoff-readiness-safe-shows-no-note-126-d10
  - gate: native:approved
  - revision: 11083b7f0423b88e48752a38411f2b4af4171417
- 2026-11-06T03:41:22Z · advance · build → test · native:verifying ^t-4096d8
  - native_change: native-check-ac2-only-a-verifying-change-with-handoff-readiness-safe-shows-no-note-126-d10
  - gate: native:verifying
  - revision: 01aebd6c729499767aff4a140266b46ad003a294
- 2026-11-06T03:41:32Z · advance · test → accept · native:verifying ^t-480829
  - native_change: native-check-ac2-only-a-verifying-change-with-handoff-readiness-safe-shows-no-note-126-d10
  - gate: native:verifying
  - revision: 01aebd6c729499767aff4a140266b46ad003a294
- 2026-11-06T03:41:32Z · gate-opened · accept → accept · Implementation review of native-check-ac2-only-a-verifying-change-with-handoff-readiness-safe-shows-no-note-126-d10 ^t-fff319
  - native_change: native-check-ac2-only-a-verifying-change-with-handoff-readiness-safe-shows-no-note-126-d10
  - revision: 01aebd6c729499767aff4a140266b46ad003a294
- 2026-11-06T04:02:19Z · gate-closed · accept → accept · Implementation review of native-check-ac2-only-a-verifying-change-with-handoff-readiness-safe-shows-no-note-126-d10 ^t-ad6e9f
  - native_change: native-check-ac2-only-a-verifying-change-with-handoff-readiness-safe-shows-no-note-126-d10
  - revision: d126a1273660ae9a4b8b33a6eb95039f7e92cfff
- 2026-11-06T04:02:32Z · advance · accept → done · native:archived ^t-eff3f3
  - native_change: native-check-ac2-only-a-verifying-change-with-handoff-readiness-safe-shows-no-note-126-d10
  - gate: native:archived
  - revision: d126a1273660ae9a4b8b33a6eb95039f7e92cfff


Closed 2026-11-06: done.
