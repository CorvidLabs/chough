---
change: prepare-a-first-release-candidate-with-reference-aware-evidence-retention-current-guidance-and-an-installable-package
artifact: requirements
---

# Requirements

Adds REQ-cli-018 (reference-aware retention) and REQ-cli-019 (installable package);
see `deltas/cli.md`. They trace to two proposed HI criteria that need the human's
agreement before capture; the wording below is a proposal, not captured:

- LIFE-8 (proposed): Routine reruns do not keep raw test logs forever, while the proof
  behind active work, open and fixed bugs, and accepted or released baselines stays,
  and anything removed is shown as removed rather than as passed.
- LIFE-9 (proposed): I can install Chough into another project and get the helper and
  skills without Chough's own backlog or run history.
