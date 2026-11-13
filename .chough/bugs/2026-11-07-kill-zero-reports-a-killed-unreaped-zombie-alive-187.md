---
kind: bug
id: 2026-11-07-kill-zero-reports-a-killed-unreaped-zombie-alive-187
aliases: [BUG-kill-zero-reports-a-killed-unreaped-zombie-alive-187-fd5176ab33fee38ff313ffc8]
title: kill-zero reports a killed unreaped zombie alive 187
status: verified
classification: product-defect
scope: blocking
impact: The timing, native and change tests can falsely report a surviving process under load (#187).
criteria: [DRIVE-19]
requirements: [REQ-cli-053]
cases: [TC-cli-killed-unreaped-child-not-alive]
evidence:
  - OBS-tc-cli-killed-unreaped-child-not-alive-9af058ce944cce5d05f3feb7
  - OBS-tc-cli-killed-unreaped-child-not-alive-ab310389bde5c4d51a54535d
receipts:
  - id: 2026-11-07-tc-cli-killed-unreaped-child-not-alive
    aliases: OBS-tc-cli-killed-unreaped-child-not-alive-9af058ce944cce5d05f3feb7
    claim: "Ran cargo test --locked --test native -- --exact a_killed_unreaped_child_is_not_alive_but_a_running_child_is for TC-cli-killed-unreaped-child-not-alive: failed (exit 101, tests run 1)."
    result: failed
    command: cargo test --locked --test native -- --exact a_killed_unreaped_child_is_not_alive_but_a_running_child_is
    revision: 599ff184c3a42d94ce6728eaa0f9bb17cd5b19ea
    environment: local-macos-arm64
    content: f8d14e8410f7a3406713757322acc71fee5f59b73698155987b4fff52cb58d41
    fingerprint: sha256:864b1bfa5a10720a6936231679c2da4a1d24d0c05702a684759003af81bd063b
    case: TC-cli-killed-unreaped-child-not-alive
    case_digest: 396594f0e7fc724973b186bd3d95a7b0bc1ec07c31e7fecc9b3546c1ee069836
    observed: 2026-11-07T03:37:57Z
    duration_seconds: 0
  - id: 2026-11-07-tc-cli-killed-unreaped-child-not-alive-ce9211
    aliases: OBS-tc-cli-killed-unreaped-child-not-alive-ab310389bde5c4d51a54535d
    claim: "Ran cargo test --locked --test native -- --exact a_killed_unreaped_child_is_not_alive_but_a_running_child_is for TC-cli-killed-unreaped-child-not-alive: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test native -- --exact a_killed_unreaped_child_is_not_alive_but_a_running_child_is
    revision: 76ba44981ed1b94b5a2101a52e86ebf1fa9ab3e4
    environment: local-macos-arm64
    content: 551ed13cd453415239abaf7abdcdac9af4eedc5cf060d4705e78736f69e0fb75
    fingerprint: sha256:6b5d51d14e19e807c4f2e3e4958be913857d537e946e2d707215a6a691181ead
    case: TC-cli-killed-unreaped-child-not-alive
    case_digest: 396594f0e7fc724973b186bd3d95a7b0bc1ec07c31e7fecc9b3546c1ee069836
    observed: 2026-11-07T03:40:56Z
    duration_seconds: 2
environment: local-macos-arm64
revision: 599ff184c3a42d94ce6728eaa0f9bb17cd5b19ea
found_by: "GitHub issue #187 and deterministic regression at 599ff184"
related: []
created: 2026-11-07T03:38:09Z
updated: 2026-11-13T04:33:19Z
---
# kill-zero reports a killed unreaped zombie alive 187

**What happened:** The native process liveness probe reports a killed, unreaped child as alive
Actual: kill -0 succeeds and the regression fails

**Steps:** A child is SIGKILLed and its parent has not waited for it yet
1. Spawn sleep 60 and confirm the helper detects it
2. SIGKILL the child without waiting, then probe it and reap it before asserting

**Expected:** The probe treats a zombie as not alive while still detecting a running child

**Why it matters:** The timing, native and change tests can falsely report a surviving process under load (#187).

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-07T03:38:09Z · Reported (GitHub issue #187 and deterministic regression at 599ff184); evidence: OBS-tc-cli-killed-unreaped-child-not-alive-9af058ce944cce5d05f3feb7
- 2026-11-07T03:42:14Z · status: open -> verified. The bounded zombie-aware helper passes the same regression after the failure; the running-child assertion also passes.; evidence: OBS-tc-cli-killed-unreaped-child-not-alive-ab310389bde5c4d51a54535d
- 2026-11-13T04:33:19Z · Tracked by the flake window (DRIVE-19); GitHub #187 · by claude-code (hc t38r)
