# Session roll-up copies, never moves (issue 165)

## Modified

### REQUIREMENT REQ-history-011


Archived session roll-up. When `chough stage advance` moves into done and its save succeeds, with the librarian on, after the trim of REQ-cli-041 and before the knowledge refresh of REQ-cli-043, the helper SHALL keep the five most recent archived sessions in `.chough/sessions/archive/` as they are, apart from the session that adopted the legacy history, and SHALL copy each older one into the vault without moving it. Recency SHALL be the finalization timestamp of the session's native change, newest first, then the key, the later key first. A session without a native change, or whose change has no readable finalization, SHALL count as the oldest. The archived session whose history cursor holds the legacy anchor (the session that adopted the legacy history) SHALL never be rolled into the vault: it SHALL stay in `.chough/sessions/archive/` in addition to the five and SHALL NOT count toward them, so history never depends on the knowledge directory. Each other archived session SHALL be copied byte for byte to `<library-dir>/attachments/<folder of its key>/sessions/<key>.json`, with the folder formed as in REQ-cli-046, and SHALL remain in `.chough/sessions/archive/` unchanged: the roll-up copies, it never moves, and a tracked session record never leaves the tree, whether or not the vault is committed (#165). An attachment that already holds the same bytes SHALL be kept as the copy and the roll-up has nothing to do for that session. One that holds different bytes SHALL leave the session and the attachment as they are and be reported. It SHALL report `applied.session_rollup` with status `rolled` and the copied keys, `unchanged`, `skipped` (librarian off), or `refused` with its reason, and SHALL NOT undo the move into done. A rolled session stays a retention reference source and `session list` keeps listing it; the vault copy serves the knowledge summary. Nothing SHALL be deleted from Git history or from the tree. Trace: KNOW-29, DRIVE-13, KNOW-13.

Acceptance Criteria
- With seven archived sessions, five tied to archives finalized 60 to 20 days ago, one tied with the oldest, and one without a native change, the move into done keeps the five newest in `.chough/sessions/archive/` as they are and copies the other two byte for byte into `knowledge/attachments/<key>/sessions/<key>.json`, reported as `rolled`, with every archived record still in the tree.
- With the librarian off, no session moves and the step reports `skipped`.
- With seven archived sessions of which only the one that would roll first holds the legacy anchor, the move into done keeps that session and the five newest of the other six in `.chough/sessions/archive/`, rolls only the remaining one, and the anchor holder has no copy in the vault.

### REQ-history-012

Doctor findings and fixes. `chough doctor` (REQ-cli-055) SHALL find this debt without a native read, and its fixes SHALL use Chough's own steps:
- `legacy_checkpoint`: `.chough/session.json` exists. The fix SHALL adopt history first when the checkpoint has no history cursor, and then adopt it as a session (REQ-history-006) under its native change ID or under `--adopt-as <key>`. With neither key, the finding SHALL be `skipped`, with a fix that names `--adopt-as <key>`.
- `stale_pointer`: the active pointer names a session that is missing or archived. The fix SHALL remove the pointer.
- `finished_session`: an open session that is finished (REQ-history-006) and has no open awaiting-human gate. The fix SHALL archive it with its bytes unchanged. A session with unfolded inbox entries SHALL instead be reported as `skipped`, with a fix that names `chough inbox fold`.
- `stale_gates`: an open session whose native change is archived on HEAD and that has open awaiting-human gates. This fix is a decision. With `--reason`, it SHALL withdraw those gates as `chough checkpoint --clear-awaiting-human` does, recording no human decision, and SHALL then archive the session as the `finished_session` fix does.
- `unfinished_session`: an open, unfinished session named with `--close <key>`. This fix is a decision. With `--reason`, it SHALL close the session as `chough session close <key> --reason` does.

The fixes SHALL run in that order. They SHALL never replace a file, and SHALL leave the bytes of every session they do not fix unchanged. Trace: DRIVE-17, DRIVE-16, DRIVE-13, DRIVE-3.

Acceptance Criteria
- A finished session at stage intent and one at done are archived byte for byte, a stale pointer is removed, and open work stays open.
- A legacy checkpoint with a native change and no history cursor is adopted under that change's ID, with a cursor. One with no native change is skipped, naming `--adopt-as <key>`, and `--adopt-as <key>` adopts it.
- A finished session with unfolded inbox entries stays open and is reported as skipped, naming `chough inbox fold`.
- Without `--reason`, a stale gate and a session named by `--close` are skipped with their files unchanged. With `--reason`, the gate is withdrawn with no human decision recorded and the session is archived, and the named session is closed with the reason.

### REQ-history-013

Direct-write events. A `direct-native-write` event SHALL have `from` and `to` equal to the current stage, and a `direct_write` object with:
- `change`, the native change;
- `files`, each with its `path` under `.specsync/changes/<id>/` and `change`, which is `added`, `changed` or `removed`;
- `before_state` and `after_state`, the change's state from `state.json`, or `null` when it is absent.

No other event SHALL carry `direct_write`, and other records SHALL keep their bytes. Readers of history that count unknown kinds, such as `chough timing`, SHALL treat it as a known kind that starts or ends no stage segment. Chough builds before this requirement refuse a history holding the event ("unknown transition kind"), so the change log SHALL name the first release that contains it as the oldest Chough that reads it. Trace: DRIVE-21.

Acceptance Criteria
- A recorded `direct-native-write` event passes history validation, and a `direct_write` object on any other kind is refused as an invalid payload.
- `chough timing` reads a history holding a `direct-native-write` event as complete: it skips the event as part of no stage segment, reports no unrecognised kind for it, and does not mark the report partial.
- The change log's entry for this change names the event and the oldest Chough release that reads it.

### REQ-history-014

Summary digests on relays. A `relayed_by` record (REQ-history-010) MAY carry `summary_digest`, 64 lowercase hex digits, and `list`, 12 lowercase hex digits, both or neither. A record with only one of them, or with a malformed value, SHALL be refused as an invalid payload. Records without them SHALL keep their bytes. Chough builds before this requirement refuse a relay holding them (unknown field), so the change log SHALL name the first release that contains it as the oldest Chough that reads it. Trace: FLOW-19, FLOW-20.

Acceptance Criteria
- A relay with both fields reads back; a relay with one of them, or with a malformed value, is refused as an invalid transition.
- The change log's entry for this change names `summary_digest` and the oldest Chough release that reads it.
