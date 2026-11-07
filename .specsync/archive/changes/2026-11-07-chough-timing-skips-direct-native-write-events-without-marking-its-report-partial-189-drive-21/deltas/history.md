## MODIFIED

### REQUIREMENT REQ-history-013

Direct-write events. A `direct-native-write` event SHALL have `from` and `to` equal to the current stage, and a `direct_write` object with:
- `change`, the native change;
- `files`, each with its `path` under `.specsync/changes/<id>/` and `change`, which is `added`, `changed` or `removed`;
- `before_state` and `after_state`, the change's state from `state.json`, or `null` when it is absent.

No other event SHALL carry `direct_write`, and other records SHALL keep their bytes. Readers of history that count unknown kinds, such as `chough timing`, SHALL treat it as a known kind that starts or ends no stage segment. Chough builds before this requirement refuse a history holding the event ("unknown transition kind"), so the change log SHALL name the first release that contains it as the oldest Chough that reads it. Trace: DRIVE-21.

Acceptance Criteria
- A recorded `direct-native-write` event passes history validation, and a `direct_write` object on any other kind is refused as an invalid payload.
- `chough timing` reads a history holding a `direct-native-write` event as complete: it skips the event as part of no stage segment, reports no unrecognised kind for it, and does not mark the report partial.
- The change log's entry for this change names the event and the oldest Chough release that reads it.
