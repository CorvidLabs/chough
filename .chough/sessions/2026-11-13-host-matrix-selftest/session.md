---
kind: session
id: 2026-11-13-host-matrix-selftest
aliases: [host-matrix-selftest]
objective: "Host-matrix harness: a scripted Chough cycle with a stand-in host and its self-test in hosted CI on macOS and Linux (#218, first PR)"
intent_scope: LIFE
modules: [cli]
stage: plan
stage_status: active
iteration: 1
callers: []
native_change: host-matrix-harness-one-scripted-chough-cycle-per-agent-host-judged-from-the-files-with-a-stand-in-host-self-test-in
pr: ""
criteria: [LIFE-21, LIFE-22, LIFE-24, LIFE-27, LIFE-28, LIFE-29, LIFE-30, FLOW-10, FLOW-16, FLOW-21]
requirements: [REQ-cli-066]
cases: []
design: []
files: []
observations: []
bugs: []
blockers: []
waiting_on_human:
  - gate: Scope approval of host-matrix-harness-one-scripted-chough-cycle-per-agent-host-judged-from-the-files-with-a-stand-in-host-self-test-in
    holds: plan
    drives: approve
    opened: 2026-11-13T03:52:59Z
run_consents: []
next: "Wait for Leif's scope approval through t370; then build red first: the case and selftest.sh, the failing run recorded, then the harness"
---
# Host-matrix harness: a scripted Chough cycle with a stand-in host and its self-test in hosted CI on macOS and Linux (#218, first PR)

## Goal

Host-matrix harness: a scripted Chough cycle with a stand-in host and its self-test in hosted CI on macOS and Linux (#218, first PR)

## Where it stands

## Decisions and why

- decided: the harness lives in tools/host-matrix/ with adapters at integrations/<host>/adapter.sh beside #219's hc-template.md because t38r's #219 layout and the test plan's bar 4 both put per-host files there
- decided: the self-test runs as its own hosted CI job on Ubuntu and macOS, not as a verify-lane step, because the verify lane's step list is pinned by README wording and tests/docs.rs, and a separate job covers both platforms (FLOW-21)
- decided: each host turn runs with the driver's agent and hc environment cleared and CHOUGH_ACTOR set to the host because chough reads the actor from CLAUDECODE and HC_TERMINAL_ID, which named the harness's own terminal on the stand-in's records
- Leif decided "Approve the intent: the host-matrix harness and its local-agent leg as Leif chose them (LIFE-27 to LIFE-30, under LIFE-21 item 4, LIFE-22 and LIFE-24)": "Plain runs, Herdr optional (Recommended); Docker on the Mac (Recommended); Not sure: keep it generic (Recommended); Next weekly release (Recommended)" (coordinator chat (Claude Code, hc t370), 2026-11-13T03:32Z, his four answers to the design questions for #218 and #220 (run mode, Linux legs, local agent, when a leg blocks a release), relayed by claude-code-t370, who said to quote them at this change's intent and scope gates); recorded by claude-code-t38q; a relayed claim, not authenticated

## What was verified

## Open threads

## Timeline

- 2026-11-13T03:48:59Z · start · intent → intent · Host-matrix harness: a scripted Chough cycle with a stand-in host and its self-test in hosted CI on macOS and Linux (#218, first PR) ^t-65e2dd
  - revision: 4a7a273b3ff6d617ef7475b8a19f28ff8ac22e4c
  - by: claude-code (claude-opus-5-5, hc t38q)
- 2026-11-13T03:49:37Z · gate-opened · intent → intent · Approve the intent: the host-matrix harness and its local-agent leg as Leif chose them (LIFE-27 to LIFE-30, under LIFE-21 item 4, LIFE-22 and LIFE-24) ^t-c853c0
  - revision: 4a7a273b3ff6d617ef7475b8a19f28ff8ac22e4c
  - holds: intent
  - drives: none
  - by: claude-code (claude-opus-5-5, hc t38q)
- 2026-11-13T03:49:53Z · gate-closed · intent → intent · Approve the intent: the host-matrix harness and its local-agent leg as Leif chose them (LIFE-27 to LIFE-30, under LIFE-21 item 4, LIFE-22 and LIFE-24) ^t-c01cb8
  - revision: 4a7a273b3ff6d617ef7475b8a19f28ff8ac22e4c
  - relayed_by:
    - decided_by: Leif
    - words: Plain runs, Herdr optional (Recommended); Docker on the Mac (Recommended); Not sure: keep it generic (Recommended); Next weekly release (Recommended)
    - where: coordinator chat (Claude Code, hc t370), 2026-11-13T03:32Z, his four answers to the design questions for #218 and #220 (run mode, Linux legs, local agent, when a leg blocks a release), relayed by claude-code-t370, who said to quote them at this change's intent and scope gates
    - recorded_by: claude-code-t38q
  - by: claude-code (claude-opus-5-5, hc t38q)
- 2026-11-13T03:50:00Z · advance · intent → design · Leif approved the intent through his four design choices (2026-11-13T03:32Z, relayed by t370) ^t-0a1499
  - revision: 4a7a273b3ff6d617ef7475b8a19f28ff8ac22e4c
  - by: claude-code (claude-opus-5-5, hc t38q)
- 2026-11-13T03:50:00Z · skip · design → plan · Scripts, prompts and CI only; no visuals or interactions change ^t-9012bf
  - revision: 4a7a273b3ff6d617ef7475b8a19f28ff8ac22e4c
  - by: claude-code (claude-opus-5-5, hc t38q)
- 2026-11-13T03:52:59Z · gate-opened · plan → plan · Scope approval of host-matrix-harness-one-scripted-chough-cycle-per-agent-host-judged-from-the-files-with-a-stand-in-host-self-test-in ^t-c01633
  - native_change: host-matrix-harness-one-scripted-chough-cycle-per-agent-host-judged-from-the-files-with-a-stand-in-host-self-test-in
  - revision: 598c4bc527007f9bebe53f7291e31130b2891561
  - holds: plan
  - drives: approve
  - by: claude-code (claude-opus-5-5, hc t38q)
