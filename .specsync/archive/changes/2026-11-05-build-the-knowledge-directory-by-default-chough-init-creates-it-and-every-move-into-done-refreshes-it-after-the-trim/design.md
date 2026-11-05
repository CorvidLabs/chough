---
change: build-the-knowledge-directory-by-default-chough-init-creates-it-and-every-move-into-done-refreshes-it-after-the-trim
artifact: design
---

# Design

- **src/library.rs**:
  - `create_at_init(root)`: librarian off gives `skipped`; an existing `INDEX.md` gives
    `preserved`. Otherwise it builds every file in memory (the existing bounded
    `build`), then writes `README.md` and each generated file create-only and ensures
    `briefs/`, reporting `created`. A build error gives `refused` with nothing written.
  - `refresh_at_done(root)`: librarian off gives `skipped`. Otherwise it does what
    `library init` does (README created only if missing, `briefs/` ensured, index and
    entries regenerated), reporting `refreshed` with the index path, or `refused` with
    the reason.
  - `library init` and both functions share one `ensure_built` helper.
- **src/main.rs**: `init` adds `library` to its output from `create_at_init`, after the
  checkpoint and skill files are written.
- **src/stage.rs**: after `librarian_trim` on the move into done,
  `applied.librarian_index = refresh_at_done(root)`.
- **Skill references** (bundle and this repository's installed copy):
  - `references/accept.md`: commit the refreshed knowledge directory with the done
    checkpoint.
  - `references/librarian.md`: says init creates the folder, done refreshes it, and a
    merge conflict in it is resolved by regenerating with `chough library index`.
  - `src/skill.rs` keeps the previous digests of both files in `KNOWN_PREVIOUS`, so
    unmodified earlier installs read as known-outdated, not edited (#102).
- **Lessons**: `specs/cli/context.md` and `specs/history/context.md` gain the supported
  2026-11-04 lessons, each with a certainty label, source, scope and future use.
- **This repository**: `chough library index` builds `knowledge/`, which is committed.
