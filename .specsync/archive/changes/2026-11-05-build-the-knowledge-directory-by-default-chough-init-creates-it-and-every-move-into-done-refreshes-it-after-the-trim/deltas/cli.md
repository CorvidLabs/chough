## ADDED

### REQUIREMENT REQ-cli-043

Knowledge directory built and kept current. With the librarian on, which is the default, `chough init` SHALL create the knowledge directory (`knowledge/`, or `library-dir`) with its `README.md`, `briefs/`, `INDEX.md` and one entry per archived native change, built and bounded in memory first and written create-only, so an existing directory is preserved byte-identical and reported as `preserved`. When the index cannot be built or bounded, `chough init` SHALL still succeed, write no knowledge file and report the library as `refused` with the reason. When `chough stage advance` moves into done and its save succeeds, after the trim of REQ-cli-041, the helper SHALL regenerate the index and every archived-change entry, creating `README.md` and `briefs/` if missing, whether or not anything was trimmed, so the change just archived and any proof just trimmed are summarized. It SHALL report the result as `applied.librarian_index` with status `refreshed` and the index path, `skipped` with the librarian off, or `refused` with its reason. A refusal SHALL NOT undo or refuse the move into done, nothing SHALL be built with the librarian off, and no gate SHALL read the directory. Trace: KNOW-1, KNOW-4, KNOW-5, KNOW-13.

Acceptance Criteria
- With defaults, `chough init` reports the library `created`, writes `knowledge/README.md`, `knowledge/INDEX.md`, `knowledge/briefs/` and one entry per archived change, and `chough next --json` names `knowledge/INDEX.md`.
- With an existing `knowledge/README.md` and `knowledge/INDEX.md`, `chough init` reports `preserved` and leaves both byte-identical.
- With an index that cannot be bounded, `chough init` succeeds, reports `refused` naming the 65536-byte bound, and leaves no knowledge directory.
- With defaults, `stage advance` into done reports `librarian_index` `refreshed` after `librarian_trim` `trimmed`, and the trimmed proof's archive entry lists it as pruned, in a project that had no knowledge directory.
- With `retention` set to `manual`, `stage advance` into done reports the trim `skipped` and still reports the index `refreshed`.
- With the librarian off, `stage advance` into done reports the index `skipped` and creates no knowledge directory, and done is still recorded.
