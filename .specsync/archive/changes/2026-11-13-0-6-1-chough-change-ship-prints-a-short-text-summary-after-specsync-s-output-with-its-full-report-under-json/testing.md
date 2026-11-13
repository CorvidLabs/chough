---
change: 0-6-1-chough-change-ship-prints-a-short-text-summary-after-specsync-s-output-with-its-full-report-under-json
artifact: testing
---

# Testing

`tests/change.rs`: a ship prints SpecSync's output first, then at most 12 lines naming
the review, the ship, the gate close and the move into done; with `--json` it prints
every step's full report as before. Red at the test commit, green at the fix.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-cli-059 | `tests/change.rs`: change ship prints a short summary after SpecSync's output, and every report under `--json` |
