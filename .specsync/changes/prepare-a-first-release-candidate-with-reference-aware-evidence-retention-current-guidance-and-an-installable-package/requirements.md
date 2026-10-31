---
change: prepare-a-first-release-candidate-with-reference-aware-evidence-retention-current-guidance-and-an-installable-package
artifact: requirements
---

# Requirements

Adds REQ-cli-018 (reference-aware retention) and REQ-cli-019 (installable package);
see `deltas/cli.md`. They trace to LIFE-8 and LIFE-9, captured in `hi/life.md` on 2026-10-31 in the
wording 0xLeif approved:

- LIFE-8: Routine reruns do not keep raw test logs forever, while the proof
  behind active work, open and fixed bugs, and accepted or released baselines stays,
  and anything removed is shown as removed rather than as passed.
- LIFE-9: I can install Chough into another project and get the helper and
  skills without Chough's own backlog or run history.
