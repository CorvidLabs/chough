## ADDED

### REQUIREMENT REQ-cli-051

Session notes. `chough library index` SHALL write `<library-dir>/sessions/<key>.md` for each session rolled into the vault by REQ-history-011, found at `<library-dir>/attachments/<folder>/sessions/<key>.json`. Each note SHALL have front matter giving `id` (`session/<key>`), `title` (the session's objective), `type: session`, `native_change`, `stage` and `tags`. It SHALL link the session's attachment and, when it exists, the native change's archive note. `library index --check` SHALL cover these notes, and the repository SHALL declare `knowledge/sessions/*.md merge=union`. Trace: KNOW-29, KNOW-14, DRIVE-12.

Acceptance Criteria
- After a roll-up, `knowledge/sessions/<key>.md` names `session/<key>` and the session's objective and links `../attachments/<key>/sessions/<key>.json`, and `library index --check` passes.
- `git check-attr` reports `union` for a session note.
