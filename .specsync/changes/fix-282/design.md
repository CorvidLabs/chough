---
change: fix-282
artifact: design
---

# Design

- `CheckpointArgs` gains `--pr <number or URL>`. The update checks it as one line of text (`ensure_text`, as for the objective) and sets `session.meta.pr`. An empty value clears it.
- The save's existing leak guard on `pr` refuses a local path or a token, or redacts it under `--redact`.
- No Timeline entry is queued, as for `--objective` and `--title`.
- `CheckpointArgs::only_shows` counts `pr`, so `--pr` is a write and is refused while `.chough/` is unmerged (REQ-cli-069).
