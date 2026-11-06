## MODIFIED

### REQUIREMENT REQ-cli-049

Retention ledger rotation. When `chough stage advance` moves into done, with the librarian on, after the trim of REQ-cli-041 and the knowledge refresh of REQ-cli-043, the helper SHALL rotate `.chough/pruned.jsonl`. It SHALL keep the newest 100 entries there, newest by `pruned_at_unix_seconds` and then by later line, and SHALL keep every entry that has no archive or names an archive without a knowledge note under `<library-dir>/archives/`. Each other entry SHALL move into `<library-dir>/attachments/<folder of its first archive>/pruned.jsonl`, the folder formed as in REQ-cli-046, and a `restored` event SHALL move with the entry it follows for the same ID (KNOW-23). A rotation SHALL append the moved lines to their shards and only then rewrite `.chough/pruned.jsonl` without them, so an interruption leaves at most lines held in both a shard and the ledger. It SHALL refuse, moving nothing, when a shard would exceed 16 MiB. It SHALL report `applied.ledger_rotation` with status `rotated` and the counts moved and kept, `unchanged`, `skipped` (librarian off), or `refused` with its reason, and SHALL NOT undo or refuse the move into done. In sessions mode the rotation SHALL run before the active session archives itself, which is the last step of the move into done (REQ-history-006, #155). Every reader of the retention ledger SHALL read each shard in path order and then `.chough/pruned.jsonl`, each file within 16 MiB. Each shard copy of a line SHALL cancel one identical copy in `.chough/pruned.jsonl`, the leftover of an interrupted rotation, which the next rotation drops; identical lines within one file SHALL all be kept, since a prune, a restore and another prune in one second write two identical entries. These readers are observation labels, bug views, `evidence prune`, the post-archive summary and the knowledge notes. Shard files SHALL NOT be listed as attachments. The repository SHALL declare `knowledge/attachments/*/pruned.jsonl merge=union`. Trace: KNOW-23, KNOW-13, DRIVE-12.

Acceptance Criteria
- With 105 old entries of cold `a1`, a restore of the oldest, and an old entry of an archive without a note, the move into done:
  - keeps 100 entries and the entry without a note in `.chough/pruned.jsonl`;
  - moves the rest, and the restore, into `knowledge/attachments/a1/pruned.jsonl`;
  - loses and duplicates no line;
  - reports `rotated` with the moved count.
- After rotation:
  - a moved ID linked by an open bug is still labeled pruned;
  - its archive note still lists it;
  - `library index --check` passes.
- A leftover copy in `.chough/pruned.jsonl` of a shard line, after its restore, does not make the restored ID pruned again.
- `git check-attr` reports `union` for an attachments folder's `pruned.jsonl`.
- In sessions mode, the move into done reports `applied.ledger_rotation` and then `applied.session_archive`, and the session is archived only after the rotation ran.
