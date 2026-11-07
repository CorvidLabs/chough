---
change: flag-direct-specsync-lifecycle-writes-in-history-183-drive-21
artifact: requirements
---

# Requirements

- ADDED REQ-cli-060 (`deltas/cli.md`): the watched records; the untracked baseline that Chough's own steps update; findings recorded with the next successful stage move; and `next` and `status` showing findings without writing.
- ADDED REQ-history-013 (`deltas/history.md`): the `direct-native-write` event and its `direct_write` object. The change log names the oldest Chough that reads it.
- MODIFIED REQ-cli-057: the change steps may write their change's direct-write baseline under `.chough/local/`, and nothing else under `.chough/`.

Trace: DRIVE-21, DRIVE-4 (nothing written between check and finalize), DRIVE-12 (append-only history, so no merge conflicts).
