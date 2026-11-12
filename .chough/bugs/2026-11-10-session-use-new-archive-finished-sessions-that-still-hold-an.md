---
kind: bug
id: 2026-11-10-session-use-new-archive-finished-sessions-that-still-hold-an
aliases:
  - BUG-session-use-new-archive-finished-sessions-that-still-hold-an-open-human-7f275557438d8b4e21bf17d5
title: session use/new archive finished sessions that still hold an open human gate
status: verified
classification: product-defect
scope: in-scope
impact: Not yet assessed.
criteria: []
requirements: [REQ-history-006, REQ-history-012]
cases: [TC-history-switch-keeps-gated-finished-sessions-open]
evidence:
  - OBS-tc-history-switch-keeps-gated-finished-sessions-open-1973aa42cca65e34173a3992
  - OBS-tc-history-switch-keeps-gated-finished-sessions-open-d53f71624ec3301d1560a05e
receipts:
  - id: 2026-11-10-tc-history-switch-keeps-gated-finished-sessions-open
    aliases: OBS-tc-history-switch-keeps-gated-finished-sessions-open-1973aa42cca65e34173a3992
    claim: "Ran cargo test --locked --test sessions -- --exact a_finished_session_with_an_open_gate_stays_open_on_switch for TC-history-switch-keeps-gated-finished-sessions-open: failed (exit 101, tests run 1)."
    result: failed
    command: cargo test --locked --test sessions -- --exact a_finished_session_with_an_open_gate_stays_open_on_switch
    revision: be1dc60a654a7392c74b6626a288ba91feae8864
    environment: local-macos-arm64
    content: 4632416d203dd4675f9740b178cb8bfcad00a02341faff6e33e86bcb631ba3ea
    fingerprint: sha256:c756d207bc0a603656228d5dbde20cd07b86622c300e59208f4c0d1a7a568982
    case: TC-history-switch-keeps-gated-finished-sessions-open
    case_digest: 93fbec0191654753d98fa9eaf2408c91c486b9c230efa456802c7b7baaa9d78c
    observed: 2026-11-10T21:14:11Z
    duration_seconds: 2
  - id: 2026-11-10-tc-history-switch-keeps-gated-finished-sessions-open-faffbc
    aliases: OBS-tc-history-switch-keeps-gated-finished-sessions-open-d53f71624ec3301d1560a05e
    claim: "Ran cargo test --locked --test sessions -- --exact a_finished_session_with_an_open_gate_stays_open_on_switch for TC-history-switch-keeps-gated-finished-sessions-open: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact a_finished_session_with_an_open_gate_stays_open_on_switch
    revision: be1dc60a654a7392c74b6626a288ba91feae8864
    environment: local-macos-arm64
    content: ab235469ee6d005e2a81c9afbc57b63bc69d3a42cab4f2732e428974bea0fa5a
    fingerprint: sha256:663f21b4c7d26166e0cd41fe85700c05b51acf759d1baf2c83193e55b3963c1e
    case: TC-history-switch-keeps-gated-finished-sessions-open
    case_digest: 93fbec0191654753d98fa9eaf2408c91c486b9c230efa456802c7b7baaa9d78c
    observed: 2026-11-10T21:14:35Z
    duration_seconds: 1
environment: local-macos-arm64
revision: be1dc60a654a7392c74b6626a288ba91feae8864
found_by: manual report
related: []
created: 2026-11-10T21:14:18Z
updated: 2026-11-10T21:14:35Z
---
# session use/new archive finished sessions that still hold an open human gate

**What happened:** A finished session with a non-empty awaiting_human list is moved to .chough/sessions/archive/ by another session's session new or session use, dropping the open gate without a recorded decision
Actual: Both gated sessions are archived alongside the ungated one; kept_open is empty

**Steps:** chough session new <key> or chough session use <key> while another open session is finished (stage done or native change archived on HEAD) and has an open awaiting-human gate
1. Open two finished sessions, one with an open awaiting-human gate
2. Run chough session new gamma
3. Both are archived; the gate is dropped

**Expected:** Switches archive only sessions doctor would close mechanically (finished_session, no open gate); a gated finished session stays open and is named with a recovery (REQ-history-006, REQ-history-012)

**Why it matters:** Not yet assessed.

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-10T21:14:18Z · Reported (manual report); evidence: OBS-tc-history-switch-keeps-gated-finished-sessions-open-1973aa42cca65e34173a3992
- 2026-11-10T21:14:35Z · status: open -> verified. Passes after the fix through the same case: gated sessions stay open with their recoveries.; evidence: OBS-tc-history-switch-keeps-gated-finished-sessions-open-d53f71624ec3301d1560a05e
