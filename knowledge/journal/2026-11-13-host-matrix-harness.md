---
id: 2026-11-13-host-matrix-harness
title: Host-matrix harness
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-13
time: 2026-11-13T14:19:19Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-13-host-matrix-selftest.md
  - .specsync/archive/changes/2026-11-13-host-matrix-harness-one-scripted-chough-cycle-per-agent-host-judged-from-the-files-with-a-stand-in-host-self-test-in/
record: .chough/archive/sessions/2026-11-13-host-matrix-selftest.md
actor:
  - agent: claude-code
    model: claude-opus-5-5
    terminal: t38q
---
# Host-matrix harness

Host-matrix harness: a scripted Chough cycle with a stand-in host and its self-test in hosted CI on macOS and Linux (#218, first PR)

Why: the harness lives in tools/host-matrix/ with adapters at integrations/<host>/adapter.sh beside #219's hc-template.md because t38r's #219 layout and the test plan's bar 4 both put per-host files there

Verified: After merging main (#242) at 30d6405e, clean tree: fledge lanes run verify (Fledge 1.8.0) passed all 12 steps, 737 tests in 48 targets, 0 failed; TC-cli-host-matrix-selftest passed with sh as dash (2b475a) and as bash (33ae9c).

Read more: [the story](../../.chough/archive/sessions/2026-11-13-host-matrix-selftest.md) · [the archived change](../../.specsync/archive/changes/2026-11-13-host-matrix-harness-one-scripted-chough-cycle-per-agent-host-judged-from-the-files-with-a-stand-in-host-self-test-in/)
