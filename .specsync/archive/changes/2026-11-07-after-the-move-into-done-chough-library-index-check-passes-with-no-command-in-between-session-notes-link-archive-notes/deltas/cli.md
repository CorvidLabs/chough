## MODIFIED

### REQUIREMENT REQ-cli-043

Knowledge directory built and kept current. With the librarian on, which is the default, `chough init` SHALL create the knowledge directory (`knowledge/`, or `library-dir`) with its `README.md`, `briefs/`, `INDEX.md` and one entry per archived native change, built and bounded in memory first and written create-only, so an existing directory is preserved byte-identical and reported as `preserved`. When the index cannot be built or bounded, `chough init` SHALL still succeed, write no knowledge file and report the library as `refused` with the reason. When `chough stage advance` moves into done and its save succeeds, after the trim of REQ-cli-041, the helper SHALL regenerate the index and every archived-change entry, creating `README.md` and `briefs/` if missing, whether or not anything was trimmed, so the change just archived and any proof just trimmed are summarized. When that first regeneration reports `refreshed` and a later step of the same move changes what the directory is generated from, that is, the ledger rotation of REQ-cli-049 reports `rotated` or the active session archives itself (REQ-history-006), the helper SHALL regenerate once more after the session's archive step, so that right after the move the directory is what its sources generate and `chough library index --check` passes (KNOW-4). It SHALL report the result of the last regeneration as `applied.librarian_index` with status `refreshed` and the index path, `skipped` with the librarian off, or `refused` with its reason. A refusal SHALL NOT undo or refuse the move into done, nothing SHALL be built with the librarian off, and no gate SHALL read the directory. Trace: KNOW-1, KNOW-4, KNOW-5, KNOW-13.

Acceptance Criteria
- With defaults, `chough init` reports the library `created`, writes `knowledge/README.md`, `knowledge/INDEX.md`, `knowledge/briefs/` and one entry per archived change, and `chough next --json` names `knowledge/INDEX.md`.
- With an existing `knowledge/README.md` and `knowledge/INDEX.md`, `chough init` reports `preserved` and leaves both byte-identical.
- With an index that cannot be bounded, `chough init` succeeds, reports `refused` naming the 65536-byte bound, and leaves no knowledge directory.
- With defaults, `stage advance` into done reports `librarian_index` `refreshed` after `librarian_trim` `trimmed`, and the trimmed proof's archive entry lists it as pruned, in a project that had no knowledge directory.
- With `retention` set to `manual`, `stage advance` into done reports the trim `skipped` and still reports the index `refreshed`.
- With the librarian off, `stage advance` into done reports the index `skipped` and creates no knowledge directory, and done is still recorded.
- In sessions mode, where the change's archive lists the session file that archives itself at done, `library index --check` passes right after `stage advance` into done, with no command in between, and the change's entry places that file in Git.

### REQUIREMENT REQ-cli-049

Retention ledger rotation. When `chough stage advance` moves into done, with the librarian on, after the trim of REQ-cli-041 and the knowledge refresh of REQ-cli-043, the helper SHALL rotate `.chough/pruned.jsonl`. It SHALL keep the newest 100 entries there, newest by `pruned_at_unix_seconds` and then by later line, and SHALL keep every entry that has no archive or names an archive without a knowledge note under `<library-dir>/archives/`. Each other entry SHALL move into `<library-dir>/attachments/<folder of its first archive>/pruned.jsonl`, the folder formed as in REQ-cli-046, and a `restored` event SHALL move with the entry it follows for the same ID (KNOW-23). A rotation SHALL append the moved lines to their shards and only then rewrite `.chough/pruned.jsonl` without them, so an interruption leaves at most lines held in both a shard and the ledger. It SHALL refuse, moving nothing, when a shard would exceed 16 MiB. It SHALL report `applied.ledger_rotation` with status `rotated` and the counts moved and kept, `unchanged`, `skipped` (librarian off), or `refused` with its reason, and SHALL NOT undo or refuse the move into done. In sessions mode the rotation SHALL run before the active session archives itself, which is the last step of the move into done that changes a record (REQ-history-006, #155); only the final knowledge regeneration of REQ-cli-043 follows it. Every reader of the retention ledger SHALL read each shard in path order and then `.chough/pruned.jsonl`, each file within 16 MiB. Each shard copy of a line SHALL cancel one identical copy in `.chough/pruned.jsonl`, the leftover of an interrupted rotation, which the next rotation drops; identical lines within one file SHALL all be kept, since a prune, a restore and another prune in one second write two identical entries. These readers are observation labels, bug views, `evidence prune`, the post-archive summary and the knowledge notes. Shard files SHALL NOT be listed as attachments. The repository SHALL declare `knowledge/attachments/*/pruned.jsonl merge=union`. Trace: KNOW-23, KNOW-13, DRIVE-12.

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

### REQUIREMENT REQ-cli-051

Session notes. `chough library index` SHALL write `<library-dir>/sessions/<key>.md` for each session rolled into the vault by REQ-history-011, found at `<library-dir>/attachments/<folder>/sessions/<key>.json`. Each note SHALL have front matter giving `id` (`session/<key>`), `title` (the session's objective), `type: session`, `native_change`, `stage` and `tags`. It SHALL link the session's attachment and, when the same build writes an archive note for the native change (one per archived change, REQ-cli-043), that note. Whether the link is written SHALL depend only on the directory's sources, never on notes the directory already holds, so a first build and a rebuild give the same bytes (KNOW-4). `library index --check` SHALL cover these notes, and the repository SHALL declare `knowledge/sessions/*.md merge=union`. Trace: KNOW-29, KNOW-14, KNOW-4, DRIVE-12.

Acceptance Criteria
- After a roll-up, `knowledge/sessions/<key>.md` names `session/<key>` and the session's objective and links `../attachments/<key>/sessions/<key>.json`, and `library index --check` passes.
- After the move into done in a project with no knowledge directory, a rolled session's note links its change's archive note, and `library index --check` passes with no command in between.
- `git check-attr` reports `union` for a session note.
