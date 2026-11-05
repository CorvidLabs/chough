# Lesson bundle: build-the-knowledge-directory-by-default-chough-init-creates-it-and-every-move-into-done-refreshes-it-after-the-trim

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Build the knowledge directory by default: chough init creates it and every move into done refreshes it after the trim; fold the 2026-11-04 lessons into the module lesson sources
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: src/library.rs, src/main.rs, src/stage.rs, src/skill.rs, tests/knowledge_default.rs, tests/librarian_trim.rs, tests/evidence.rs, tests/optional.rs, skills/chough-workflow/references/librarian.md, skills/chough-workflow/references/accept.md, docs/cli.md, docs/FRD.md, CHANGELOG.md
- **Acceptance**: With the librarian on, chough init creates knowledge/ with README.md, INDEX.md, briefs/ and one entry per archived change, preserves an existing folder, and still succeeds writing no knowledge file when the index cannot be bounded. stage advance into done regenerates the index and entries after the trim whether or not anything was trimmed, reporting applied.librarian_index refreshed, skipped with the librarian off, or refused with its reason, and never undoes the move. Chough own repository commits its knowledge/ folder, and the cli and history lesson sources gain the supported 2026-11-04 lessons with certainty labels. The new tests fail first and fledge lanes run verify passes on Linux.

## Evidence

- Verification commit: `505097a71ad58905595c6f552201b7b710e3865a`
- Base commit: `f7e576410539c2922330afd773b64fa5df4d9636`
- Verified by: `specsync check --spec cli --spec history`

## From the change's context.md

# Context

On 2026-11-04 Chough's own `chough next` said "Knowledge index: not built; run `chough
library init` to create knowledge/". The librarian is on by default (KNOW-5), but no
command builds `knowledge/` except `library init` and `library index`. The trim at done
(REQ-cli-041) regenerates entries only when it removes something, and does so before it
removes and ledgers the files. The trim ran on this repository five times that day and
left 85 ledger entries in `.chough/pruned.jsonl` pointing to
`knowledge/archives/<change>.md` files that were never committed. The one folder it did
write, on #97's branch, was deleted as a stray by the agent. Nothing is lost:
`chough library index` regenerates all 79 entries in about 2 seconds, with the trimmed
observations shown as "not-run (pruned)" (KNOW-4). But a fresh agent is told there is no
index, against KNOW-1.

The module lesson sources hold 5 lessons, all in cli and all from 2026-10-31. The
history and reporting sections say none are recorded yet. No change on 2026-11-04 ran
the closeout lesson step from the librarian reference, including the agent's own
closeouts.

Leif (0xLeif) answered "yes" on 2026-11-04 to building and committing `knowledge/`
here, making the librarian default create it at init and refresh it when the trim runs
at done, and folding that day's lessons into the cli and history lesson sources.

## From the change's design.md

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

## From the change's testing.md

# Testing

New tests, each failing before the change:
- tests/knowledge_default.rs: init builds the directory; init preserves an existing one;
  init still succeeds and writes no knowledge when the index cannot be bounded.
- tests/librarian_trim.rs: done refreshes the entries after the trim (the trimmed proof
  reads as pruned); done refreshes the index when nothing is trimmed; a librarian-off
  project gets no knowledge directory at done.

Two existing tests change with the requirement. A refused `library init` test in
tests/evidence.rs removes the folder that init now creates first, and its assertions are
unchanged. tests/optional.rs expects the index right after init, where it expected none. Cases bind the 6 new
tests. `fledge lanes run verify` runs on Linux.

## Where these lessons go

- `specs/cli/context.md`
