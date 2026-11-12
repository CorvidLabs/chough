---
kind: bug
id: 2026-11-02-stage-advance-and-skip-ignore-open-awaiting-human-gates
aliases: [BUG-stage-advance-and-skip-ignore-open-awaiting-human-gates-44e1fb6326c0fb84df93913d]
title: Stage advance and skip ignore open awaiting-human gates
status: verified
classification: product-defect
scope: in-scope
impact: An agent can leave intent or design before the human answers
criteria: [FLOW-3, FLOW-5, FLOW-13]
requirements: [REQ-cli-008]
cases: []
evidence:
  - OBS-product-test-8a888ae0b4d5866b8dedc6ed
  - OBS-tc-cli-open-human-gates-refuse-advance-and-skip-6fc7a43ac647ae0a7d1fe491
  - OBS-tc-cli-open-human-gates-leave-other-moves-and-agree-in-next-4371a46d89acedf07b7ebd72
  - OBS-product-test-c78b4ac4ffd21c898000a98b
receipts:
  - id: 2026-11-02-issue-38-reproduced-at-825a380-with-an-open-awaiting-human
    aliases: OBS-product-test-8a888ae0b4d5866b8dedc6ed
    claim: "Issue #38 reproduced at 825a380: with an open awaiting-human gate, stage advance applied intent -> design and stage skip applied plan -> build (SDD off), both exit 0"
    result: failed
    revision: 825a3805530456005e361dfa6d96d93eee52cd09
    environment: local-linux-x86_64
    content: e4258c3618153b18485f2910a9d0d2b8b32381dc5c4f19d9c01786f0a144de32
    fingerprint: sha256:9d1679a4842f7e7b86241623772dd4033cb7a6400852c3acd1f9ed1ce280cff4
    observed: 2026-11-02T16:33:55Z
  - id: 2026-11-02-tc-cli-open-human-gates-refuse-advance-and-skip
    aliases: OBS-tc-cli-open-human-gates-refuse-advance-and-skip-6fc7a43ac647ae0a7d1fe491
    claim: "Ran cargo test --locked --test stages -- --exact open_human_gates_refuse_advance_and_skip for TC-cli-open-human-gates-refuse-advance-and-skip: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test stages -- --exact open_human_gates_refuse_advance_and_skip
    revision: 825a3805530456005e361dfa6d96d93eee52cd09
    environment: local-linux-x86_64
    content: 64e2f811634fdf18ba8c2e3b49a2a561f7b032acd10cb828e326fa14e45b66ef
    fingerprint: sha256:d27a18c4fe19a138768d0657c6b70f2d6fd4cc052379ea5ea91e2889f8f45b31
    case: TC-cli-open-human-gates-refuse-advance-and-skip
    case_digest: 900ce7adddcd30112f09fdf9c8fe486024cfc55cd6ae12af8d9bc6f1561fae75
    observed: 2026-11-02T16:38:26Z
    duration_seconds: 0
  - id: 2026-11-02-tc-cli-open-human-gates-leave-other-moves-and-agree-in-next
    aliases: OBS-tc-cli-open-human-gates-leave-other-moves-and-agree-in-next-4371a46d89acedf07b7ebd72
    claim: "Ran cargo test --locked --test stages -- --exact open_human_gates_leave_other_moves_and_agree_in_next for TC-cli-open-human-gates-leave-other-moves-and-agree-in-next: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test stages -- --exact open_human_gates_leave_other_moves_and_agree_in_next
    revision: 825a3805530456005e361dfa6d96d93eee52cd09
    environment: local-linux-x86_64
    content: 64e2f811634fdf18ba8c2e3b49a2a561f7b032acd10cb828e326fa14e45b66ef
    fingerprint: sha256:4a140fc6995ed37b096137aa79fd48e16feb5823cbc1de52de3eaac2f50ccc38
    case: TC-cli-open-human-gates-leave-other-moves-and-agree-in-next
    case_digest: 5daf3a235e10e0ccf140ae1ed2a65841700b4b518ce41cd54483fcaa554c46b8
    observed: 2026-11-02T16:38:26Z
    duration_seconds: 0
  - id: OBS-product-test-c78b4ac4ffd21c898000a98b
    removed: .chough/archive/migrations/2026-11-12-from-0-5.md
environment: Linux x86_64, main 825a380, SpecSync 6.1.2, Fledge 1.8.0, hi 0.8.0, examples/shipping copy with SDD off
revision: 825a3805530456005e361dfa6d96d93eee52cd09
found_by: https://github.com/CorvidLabs/chough/issues/38
related: []
created: 2026-11-02T16:33:55Z
updated: 2026-11-02T16:38:46Z
---
# Stage advance and skip ignore open awaiting-human gates

**What happened:** stage advance and skip apply while the checkpoint awaits a human decision; the gate carries into the next stage
Actual: Exit 0; the stage moves and awaiting_human is unchanged

**Steps:** chough checkpoint --awaiting-human <gate>, then chough stage advance or skip

**Expected:** The move is refused until the gate is cleared

**Why it matters:** An agent can leave intent or design before the human answers

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-02T16:33:55Z · Reported (https://github.com/CorvidLabs/chough/issues/38); evidence: OBS-product-test-8a888ae0b4d5866b8dedc6ed
- 2026-11-02T16:38:35Z · status: open -> verified. Cases TC-cli-open-human-gates-* pass on local-linux-x86_64; evidence: OBS-tc-cli-open-human-gates-refuse-advance-and-skip-6fc7a43ac647ae0a7d1fe491, OBS-tc-cli-open-human-gates-leave-other-moves-and-agree-in-next-4371a46d89acedf07b7ebd72
- 2026-11-02T16:38:46Z · Fix in src/stage.rs move_forward (ensure_no_open_human_gate) per REQ-cli-030; full cargo test passes (198/0). Status went open -> verified in one step, skipping fix-pending-verification, because a scripted update failed before its intermediate call.; evidence: OBS-product-test-c78b4ac4ffd21c898000a98b
