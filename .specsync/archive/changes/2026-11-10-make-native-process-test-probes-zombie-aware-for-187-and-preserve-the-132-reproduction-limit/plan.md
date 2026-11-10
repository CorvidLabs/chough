---
change: make-native-process-test-probes-zombie-aware-for-187-and-preserve-the-132-reproduction-limit
artifact: plan
---

# Plan

## Approach

Prove the zombie defect with one focused failing test, then fix only the shared test liveness probe and migrate all three existing callers. Record the failure and passing observations through per-change Chough cases, stress the original timing scenario on both supported hosts, and finish with the Linux verify lane and archive-first CI.

## Out of scope

Product process termination, checkpoint locking, new stderr/timing instrumentation for #132, contract or HI edits, new requirement IDs, release activity, and unrelated flakes. Codex P2 fixes are capped at two rounds before one tracking issue; P1s are always fixed.

## Steps

1. Add the current `kill -0` helper and deterministic killed-unreaped-child regression; run and commit it failing before the fix.
2. Record the failing case and local bug, then implement the bounded zombie-aware helper and migrate the three probes.
3. Add focused cli and reporting cases. Run the regression and affected native/change/timing tests, then loaded repetitions on macOS and Linux.
4. Run `fledge run linux-verify`, record observations, refresh the knowledge index and complete native verification.
5. Present #187 evidence and #132's unchanged open status for orc-mediated human review; archive before waiting on hosted CI.

## Risks

Platform `ps` output or signal timing could differ; the regression checks both zombie and live-process behavior on macOS and Linux. A full second of polling for a real survivor adds bounded test time but cannot hide one. Failure to reproduce #132 remains an explicit limit, not evidence that its hypothesis is fixed.

## Constraints consulted

DRIVE-19 requires an actual cause. The handoff requires failing test first, one theme per PR, archive-first CI, no speculative #132 fix, and orc-only human gate records. Existing native timeout and reporting requirements remain unchanged.
