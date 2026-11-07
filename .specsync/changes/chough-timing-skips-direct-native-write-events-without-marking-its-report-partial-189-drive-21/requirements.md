---
change: chough-timing-skips-direct-native-write-events-without-marking-its-report-partial-189-drive-21
artifact: requirements
---

# Requirements

MODIFIED REQ-history-013 (see `deltas/history.md`):
- Readers of history that count unknown kinds, such as `chough timing`, treat `direct-native-write` as a known kind that starts or ends no stage segment.
- A new criterion: `chough timing` reads a history holding the event as complete, with no unrecognised kind and no partial mark.

MODIFIED REQ-cli-057 (see `deltas/cli.md`): `chough change new` passes `--no-spec-change` and `--rationale` through, and refuses the first without the second.

Trace: DRIVE-21, DRIVE-20.
