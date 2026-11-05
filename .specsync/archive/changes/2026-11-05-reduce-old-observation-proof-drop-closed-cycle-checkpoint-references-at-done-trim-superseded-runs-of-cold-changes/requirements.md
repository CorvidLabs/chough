---
change: reduce-old-observation-proof-drop-closed-cycle-checkpoint-references-at-done-trim-superseded-runs-of-cold-changes
artifact: requirements
---

# Requirements

MODIFIED REQ-cli-033 (see `deltas/cli.md`):
- unlisted proof is attributed by cycle time (KNOW-20);
- only the newest run and the newest failure of a case and environment stay protected as
  newest (KNOW-17);
- validity is read in parallel, with at most 8 reads at once;
- a knowledge entry summarizes the sets attributed to it.

ADDED REQ-cli-045: with the librarian on, the move into done drops closed-cycle
observation references from the checkpoint before the trim (KNOW-16).
