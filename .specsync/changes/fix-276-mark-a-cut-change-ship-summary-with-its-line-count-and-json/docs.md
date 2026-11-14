---
change: fix-276-mark-a-cut-change-ship-summary-with-its-line-count-and-json
artifact: docs
---

# Docs

- `docs/cli.md`, Ship step 5: when the summary is longer, its last line names
  how many lines were dropped and points at `--json`.
- `CHANGELOG.md`, Unreleased: the `chough change ship` summary bullet now says
  that a longer summary's last line names how many lines were dropped and points
  at `--json`, which prints every step's full report (REQ-cli-059, #276).