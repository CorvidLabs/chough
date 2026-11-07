# Lesson bundle: after-the-move-into-done-chough-library-index-check-passes-with-no-command-in-between-session-notes-link-archive-notes

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: After the move into done, chough library index --check passes with no command in between: session notes link archive notes from their sources, and the knowledge directory is regenerated after the session archives itself (#191, KNOW-4)
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: src/library.rs, src/vault.rs, src/stage.rs, tests/reduce.rs, docs/cli.md, CHANGELOG.md
- **Acceptance**: REQ-cli-043, REQ-cli-049 and REQ-cli-051 (modified): right after chough stage advance into done, with no command in between, chough library index --check passes. (1) A session rolled into a vault that held no archive note for its change before that move links its change's archive note, because whether the link is written depends only on the archive notes the same build writes, never on notes already in the directory (KNOW-4). (2) When the change's archive lists the session file that archives itself at done, the directory is regenerated once more after that self-archive, so the archive note places the file in Git; applied.librarian_index reports the last regeneration. Two tests in tests/reduce.rs fail first on main f41e7ab4 and pass after; the spec-sync reproduction in #191 passes the check; docs/cli.md describes the order and the change log names #191.

## Evidence

- Verification commit: `12dd018b750323d06319edcddf62a2bbc3c45be1`
- Base commit: `f41e7ab4c6380698783c1e98a4c6cf2fa96c1ff8`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

Issue #191: review finding F1 of the 2026-11-06 Chough review (`_audits/chough-review-2026-11-06.md`). Kimi observed it twice, and the orc reproduced it. Leif's burn-down plan (`_audits/burn-down-plan-2026-11-06.md`, relayed by the orc t18) assigned it to ch-pr69 as the first item: file F1, then fix it.

`chough stage advance` into done reports `librarian_index: refreshed`, yet `chough library index --check` run right after exits 1, and a `chough library index` makes it pass. Reproduced on a fresh clone of spec-sync main 9dd2affd with chough main f41e7ab4: `session use s5-906`, `stage reset --to accept`, `stage advance` (9 sessions rolled), then `--check` failed on 10 files. A diff of the vault written at done against a fresh build shows two causes:

1. **Session notes read earlier output.** `vault::session_notes` writes the `Native change:` link only when `archives/<change>.md` already exists on disk. If the roll-up moves sessions into a vault without those notes (a first build, or an archive note that is new in this pass), the links are missing until the next build. This accounts for all 9 `knowledge/sessions/*.md` differences.
2. **Done changes a source after its own refresh.** An archive entry's file table records where each listed `.chough/` path is now (`working tree`, `working tree (changed since archive)` or `Git <commit>`). The required order is roll-up (REQ-history-011), refresh (REQ-cli-043), rotation (REQ-cli-049), then the session self-archive (REQ-history-006). When the change's archive lists its own session file, the refresh sees it as changed in the working tree, and then the self-archive moves it. This is the `archives/...-906.md` difference.

A first `library index` on an empty vault with no rolled sessions passes its own check. On chough main, the archive for #183 doesn't list its session file, so cause 2 doesn't show there.

## Decisions that bind this change

- **HI.** Traced to the approved KNOW-4 ("regenerating with the same sources gives identical bytes"). No new criterion: the burn-down rule is "HI first, trace or draft".
- **How to make done consistent.** One more regeneration after the session archives itself, and only when the first pass refreshed and a later step changed a source (rotation `rotated` or session `archived`).
  - Rejected: moving the refresh after the rotation. REQ-cli-049 keeps ledger entries whose archive has no note yet (the KNOW-23 safeguard), so the rotation needs the refresh before it. Reordering would weaken that safeguard and change three requirements' order.
  - Rejected: patching only the affected rows. That is a second generator that could drift from the first.
- **Cost.** The extra pass costs one more index build at done. Measured `chough library index` on spec-sync (348 archives) at load 75 took 3.4–4.6 s. A small project takes well under a second. `next` and `status` are unaffected.
- **Session-note link.** It is decided from the archive IDs of the same build: the notes REQ-cli-043 writes, one per archived change.
- **Out of scope:**
  - #165 / F2: the roll-up into an untracked vault.
  - #136: an observation recorded after a rebuild.

## From the change's design.md

# Design

- **Session notes** (`src/vault.rs`). `session_notes(root, dir, archived)` takes the set of archive IDs that `library::build` writes notes for (`found[*].id`). It links `../archives/<change>.md` when the session's native change is in that set, and no longer calls `exists` on the output directory. `build` passes the set it already holds.
- **The move into done** (`src/stage.rs`). After `session_archive`, the helper regenerates again when both hold:
  - `librarian_index.status` is `refreshed`;
  - `ledger_rotation.status` is `rotated` or `session_archive.status` is `archived`.

  It calls `library::refresh_at_done` again and stores that result in `applied.librarian_index`. The earlier steps and their order are unchanged. A refusal of the second pass is reported the same way and never undoes the move.
- **No new output field.** `applied.librarian_index` keeps its shape (`refreshed` with the index path, `skipped` or `refused`).

## From the change's testing.md

# Testing

New tests in `tests/reduce.rs`, using its `project` fixture: archives a1 to a5, `init` with the librarian on, and a stand-in `specsync` reporting the change archived.

- `a_session_rolled_into_a_fresh_vault_links_its_archive_note_and_the_check_passes`: `seven_archived_sessions`, then remove `knowledge/` and set `retention` to `manual`, so no trim builds the vault first (as in #191, where the trim refused), then `stage advance`. The trim is `skipped`, and the roll-up moves s0 and s6. `knowledge/sessions/s0.md` links `../archives/a1.md`, and `library index --check` passes with no command in between.
- `done_leaves_the_vault_current_when_the_archive_lists_the_self_archiving_session`: sessions mode (`history adopt --apply`, `session adopt legacy`). a5's manifest lists `.chough/sessions/legacy.json`, and the vault is current before the move. After `stage advance`, `session_archive` is `archived`, `library index --check` passes, and `knowledge/archives/a5.md` places `.chough/sessions/legacy.json` in Git.

Both fail first on main f41e7ab4, checked in a scratch worktree: the first because `knowledge/sessions/s0.md` lacks the link, the second because `--check` names `knowledge/archives/a5.md`. With the trim on, the first passes on main, because the trim builds the archive notes before the roll-up. Cases: `TC-cli-done-fresh-vault-session-links` and `TC-cli-done-self-archive-vault-current` in `.chough/cases/cli.done-vault-current.cases.toml`, environment `local-macos-arm64`.

Mutation round, each mutant failing at least one new test:
- go back to the on-disk `exists` check for the link;
- drop the final regeneration;
- run it only on `rotated`.

Real-record check: the #191 spec-sync reproduction passes `library index --check` right after `stage advance`.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-cli-043 (modified) | tests/reduce.rs `done_leaves_the_vault_current_when_the_archive_lists_the_self_archiving_session` |
| REQ-cli-049 (modified) | tests/reduce.rs, the existing rotation tests unchanged |
| REQ-cli-051 (modified) | tests/reduce.rs `a_session_rolled_into_a_fresh_vault_links_its_archive_note_and_the_check_passes` |

## Lanes

`fledge lanes run verify` passes on macOS.

## Where these lessons go

- `specs/cli/context.md`
