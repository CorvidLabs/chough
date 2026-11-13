---
change: fix-247-the-verify-lane-s-example-waits-past-the-failing-run-s-second-before-its-rerun
artifact: context
---

# Context

On PR #244's head f904d498, CI run 43357893936 failed the verify lane's `exercise`
step: `examples/feature-bug-loop.sh` logged the failing run of `TC-shipping-0001` at
03:19:28.106Z and the fixed rerun at 03:19:28.962Z. Observed times have one-second
resolution, and REQ-cli-012 refuses `verified` for a pass observed in the same second
as the latest linked failure, so `chough bug update --status verified` refused, as
specified. A fast runner reaches the rerun in under a second. t384 filed it as GitHub
#247; #169 was a similar second-boundary flake.

On 2026-11-13T03:44Z Leif approved the fix ("Approve (Recommended)", relayed by t370):
the script waits past the failing run's second before rerunning; t37f builds it with
a failing test first. REQ-cli-012's same-second rule is specified and stays as it is;
weakening it was ruled out.
