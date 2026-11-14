---
change: fix-276-mark-a-cut-change-ship-summary-with-its-line-count-and-json
artifact: requirements
---

# Requirements

REQ-cli-059 (`specs/cli/requirements.md`): after SpecSync's output, `chough change
ship` prints by default a short text summary of the review, the ship, the gate
close and the move into done, at most 12 lines, and prints each step's full report
under `--json`, so an agent loses nothing. When the summary is longer than 12
lines, its last line names how many lines were dropped and points at `--json` for
the full report.

Acceptance criterion added to REQ-cli-059: a summary longer than 12 lines ends
with a line naming the dropped lines and `--json`.