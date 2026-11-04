---
change: order-next-lesson-bundles-by-finalization-and-guide-a-fresh-cycle-to-a-new-native-change-issues-74-and-76
artifact: design
---

# Design

- src/library.rs: `HOT_ARCHIVES` becomes `pub(crate)`. New `finalized(root, dir)` reads
  an archive's `finalization.json` within the native record bound and returns its
  `change_id` and `timestamp` when the directory name ends with that ID (the same check
  `archives` uses). New `finalized_at(root, change)` returns the timestamp of the
  archive whose `change_id` is exactly `change`.
- src/main.rs `archived_bundles`: sort the directories that hold a `lesson-bundle.md`
  by `(timestamp or 0, name)` descending and show `HOT_ARCHIVES` of them. The JSON shape
  is unchanged (`total`, `items` of paths), so only the order and the count change.
- src/stage.rs `assess`: `previous_cycle` is true when the latest stage transition is a
  reset (the old rule) or when the newest reset's `at_unix_seconds` is not earlier than
  the recorded change's finalization timestamp. An archived, authenticated change that
  is the previous cycle's skips `archived_action`; when the next main gate is refused,
  the action is `Continue <stage>: <checkpoint next action>` with the command
  `specsync change new ...`. A refused `refused:no-native-change` gate also names
  `specsync change new ...`. Gates are unchanged, and no native read is added (the
  finalization file is local).
