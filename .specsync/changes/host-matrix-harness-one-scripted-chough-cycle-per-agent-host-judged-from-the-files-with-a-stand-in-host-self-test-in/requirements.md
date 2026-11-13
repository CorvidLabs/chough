---
change: host-matrix-harness-one-scripted-chough-cycle-per-agent-host-judged-from-the-files-with-a-stand-in-host-self-test-in
artifact: requirements
---

# Requirements

Adds REQ-cli-066 (see `deltas/cli.md`), tracing LIFE-21 item 4, LIFE-22, LIFE-24,
LIFE-27, FLOW-10, FLOW-16 and FLOW-21. LIFE-28 to LIFE-30 are captured here and
delivered by later changes.

Observable outcomes:

1. `sh tools/host-matrix/selftest.sh` prints `HOST MATRIX SELFTEST PASSED` on macOS and
   Linux: the stand-in's good run is PASS, and each known-bad run is FAIL on the check
   that names its fault.
2. Hosted CI runs that self-test as `Host matrix self-test` on `ubuntu-latest` and
   `macos-latest`.
3. No record the harness writes names a person: every stand-in gate decision, native
   approval and native review names `SIMULATED-host-matrix-not-a-human`.
