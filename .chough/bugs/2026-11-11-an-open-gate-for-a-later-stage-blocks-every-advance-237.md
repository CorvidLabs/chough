---
kind: bug
id: 2026-11-11-an-open-gate-for-a-later-stage-blocks-every-advance-237
aliases: [BUG-an-open-gate-for-a-later-stage-blocks-every-advance-237-056bfe2e370ccfcd398a26d8]
title: An open gate for a later stage blocks every advance (#237)
status: verified
classification: product-defect
scope: in-scope
impact: "Agents clear real gates to keep working, or stall; the release decision has no place to wait (GitHub #237)"
criteria: [FLOW-3, FLOW-5]
requirements: [REQ-cli-030]
cases: [TC-cli-237-gate-holds-its-stage]
evidence:
  - OBS-tc-cli-237-gate-holds-its-stage-bb20e2599081ad8899140f35
  - OBS-tc-cli-237-gate-holds-its-stage-e67a2b8c1d138b94f8605278
receipts:
  - id: 2026-11-11-tc-cli-237-gate-holds-its-stage
    aliases: OBS-tc-cli-237-gate-holds-its-stage-bb20e2599081ad8899140f35
    claim: "Ran cargo test --locked --test humans -- --exact a_gate_holds_only_the_stage_it_names for TC-cli-237-gate-holds-its-stage: failed (exit 101)."
    result: failed
    command: cargo test --locked --test humans -- --exact a_gate_holds_only_the_stage_it_names
    revision: 018412c85bc31e1b2695a93e58ad0920eb5edf60
    environment: local-macos-arm64
    content: ae213eb2e0c4bf6af4ec8e6a82ffa0076d351f73faeac9e7df6f215182044fab
    fingerprint: sha256:1e1461f291774a6a2fabdb3116ee286cb05d3b3bd7e55fe6142795adf42223d5
    case: TC-cli-237-gate-holds-its-stage
    case_digest: d1117e3e2e52ace02f6728437b006782f49617e1de1508232c9b1cc6f5f11427
    observed: 2026-11-11T19:04:52Z
    duration_seconds: 0
  - id: 2026-11-11-tc-cli-237-gate-holds-its-stage-b886a7
    aliases: OBS-tc-cli-237-gate-holds-its-stage-e67a2b8c1d138b94f8605278
    claim: "Ran cargo test --locked --test humans -- --exact a_gate_holds_only_the_stage_it_names for TC-cli-237-gate-holds-its-stage: passed (exit 0)."
    result: passed
    command: cargo test --locked --test humans -- --exact a_gate_holds_only_the_stage_it_names
    revision: 82a7392dda5b59925f1dad704d2c6c2778b23b8b
    environment: local-macos-arm64
    content: c0ad4a455cdd87cc042886e343d1ad1a31016c6ee2b83c35f413758823f87006
    fingerprint: sha256:e5f6b07da6bb0d07a1cfeb3a96a4f90ebb84361368f6ad813cb69c374792606a
    case: TC-cli-237-gate-holds-its-stage
    case_digest: d1117e3e2e52ace02f6728437b006782f49617e1de1508232c9b1cc6f5f11427
    observed: 2026-11-11T19:09:10Z
    duration_seconds: 1
environment: local-macos-arm64
revision: 99b2664f637137a402777a2e023ee66321ffce62
found_by: "GitHub issue #237; red on 0.5.0 at leif/0.6.0-237-red 018412c8"
related: []
created: 2026-11-11T19:05:26Z
updated: 2026-11-11T19:09:40Z
---
# An open gate for a later stage blocks every advance (#237)

**What happened:** chough stage advance is refused with refused:awaiting-human at build while the only open gate is a release decision
Actual: every advance and skip is refused while any gate is open, so a pending release or publication decision stops build, test and accept

**Steps:** Open a human gate meant for release while the session is at build, then advance
1. chough init
2. chough stage advance (three times, to build)
3. chough checkpoint --awaiting-human "Publish the release"
4. chough stage advance

**Expected:** build, test and accept advance as usual; only the move out of release waits for the gate

**Why it matters:** Agents clear real gates to keep working, or stall; the release decision has no place to wait (GitHub #237)

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-11T19:05:26Z · Reported (GitHub issue #237; red on 0.5.0 at leif/0.6.0-237-red 018412c8); evidence: OBS-tc-cli-237-gate-holds-its-stage-bb20e2599081ad8899140f35
- 2026-11-11T19:05:26Z · Added despite similar reports: The verified bug is the opposite defect: gates were ignored and advance went through. This one is the fix's overreach: every gate refuses every move, including gates that hold a later stage.
- 2026-11-11T19:09:40Z · status: open -> verified. Fixed by plan step 6 (PR #238, f70f5727): a gate holds the stage it names (--holds, default the current stage), and advance and skip are refused only out of a stage an open gate holds. TC-cli-237-gate-holds-its-stage passed on the 0.6.0 branch after its 0.5.0 failure.; evidence: OBS-tc-cli-237-gate-holds-its-stage-e67a2b8c1d138b94f8605278
