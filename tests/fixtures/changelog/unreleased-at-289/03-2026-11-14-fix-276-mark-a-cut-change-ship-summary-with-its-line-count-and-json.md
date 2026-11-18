- `chough change ship` prints SpecSync's output, then a short text summary of at most 12
  lines naming the review, the ship, the gate close and the move into done, in place of
  the gate close's long JSON report and the move's summary; when the summary is longer,
  its last line names how many lines were dropped and points at `--json`, which prints
  every step's full report (REQ-cli-059, #276).
