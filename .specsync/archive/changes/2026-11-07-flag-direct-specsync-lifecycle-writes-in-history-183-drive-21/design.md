---
change: flag-direct-specsync-lifecycle-writes-in-history-183-drive-21
artifact: design
---

# Design

- **Watched set.** In `.specsync/changes/<id>/`: `state.json`, `approvals.json`, `review.json`, `review-attempts.json`, `verification.json` and `verification-attempts.json`. The fingerprint is a map from file name to SHA-256 of its bytes for each present file, plus the `state` value of `state.json`.
- **Local file.** `.chough/local/native-writes/<id>.json` holds `{schema_version: 1, baseline: {files, state}, pending: [finding]}`. It is written whole to a temporary file and renamed into place.
- **Finding.** `{files: [{path, change}], before_state, after_state}`, where each `change` is `added`, `changed` or `removed`.
- **Change steps** (`src/change.rs`): before the SpecSync step, a difference from the baseline is appended to `pending`. After the step, the baseline is set to the current records, whatever the step's exit code: the step was Chough's. For `new`, the change ID comes from SpecSync's JSON output (`change.id`). Ship's refusals before running write nothing, as REQ-cli-059 requires.
- **Stage moves** (`src/stage.rs`): with a native change recorded, the save that publishes a successful move first publishes one `direct-native-write` event per pending finding and per current difference. The event has `from` and `to` set to the stage before the move. Then the local file is cleared and re-baselined. With no local file, the move sets the baseline and records nothing.
- **Events.** `Transition` gains `direct_write: Option<DirectWrite>`, skipped when it is `None`, so other records keep their bytes. `validate_payload` accepts the kind `direct-native-write` only with `direct_write` and `from == to`, and refuses `direct_write` on any other kind.
- **Views.** `next` prints `Direct lifecycle writes: N not yet recorded (approvals.json changed, …)` when any exist. `status` adds `direct_writes: {pending: [...], baseline: true|false}`. Neither writes.
