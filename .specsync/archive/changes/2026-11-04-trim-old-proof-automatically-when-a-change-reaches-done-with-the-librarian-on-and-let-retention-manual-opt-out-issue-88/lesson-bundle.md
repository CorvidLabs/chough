# Lesson bundle: trim-old-proof-automatically-when-a-change-reaches-done-with-the-librarian-on-and-let-retention-manual-opt-out-issue-88

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Trim old proof automatically when a change reaches done with the librarian on, and let retention manual opt out (issue 88)
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: src/library.rs, src/prefs.rs, src/stage.rs, tests/librarian_trim.rs, docs/cli.md, docs/FRD.md, CHANGELOG.md
- **Acceptance**: With defaults stage advance into done reports trimmed and prunes the cold archived proof that library archive would with a ledger entry whose authority names KNOW-13. With retention manual or the librarian off it reports skipped and removes nothing. With a native change open it reports refused naming it and removes nothing and done is still recorded. Each new test fails on main 91a1fdd and fledge lanes run verify passes on Linux.

## Evidence

- Verification commit: `c51de0157cd645cc65fe1464575bdb5a63e5ddaf`
- Base commit: `2896f5ada1947a307b9958007a109c06dc3a2831`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

Issue https://github.com/CorvidLabs/chough/issues/88 (field test 2026-11-03): with the
librarian on, nothing trims after archive; `chough library archive` only previews and
`--apply` needs `chough consent grant retention` first. Leif prefers trimming to be
automatic by default with an opt-out.

Leif chose on 2026-11-04 ("Auto-trim at done"), captured as KNOW-13: with the librarian
on (the default), `stage advance` into done, after the archive, trims cold archived
proof, keeping the ledger-first and KNOW-10 restore checks, and `chough pref set
retention manual` opts out; the librarian default counts as the standing preference
KNOW-11 names. A preview with a one-step apply, and an opt-in standing preference, were
offered and not chosen. hi keeps KNOW-11's wording (criteria are added, not rewritten).

Trimming reads native status for the cold archives whose files it would remove (KNOW-10),
which can take time on projects with many archives; it runs only on the move into done,
never inside `next` or `status`.

Authority: Leif's 2026-11-04 standing approval for bounded Chough bug fixes, his brief to
fix issues 81 to 88 one PR each, and his #88 choice.

## From the change's design.md

# Design

- src/prefs.rs: `retention` accepts `manual` beside `off` (default) and `auto`.
- src/library.rs: the `library archive` body becomes `archive_value(root, dir,
  authority)`, used by the command unchanged. New `trim_at_done(root)` returns JSON and
  never fails: `skipped` when the librarian is off or `retention` is `manual`; otherwise
  it applies the post-archive summary with the recorded authority (a run consent or
  `retention auto`) or, failing that, "standing preference: librarian on (KNOW-13)", and
  returns `trimmed` with the summary, or `refused` with the reason (an open native
  change, an unreadable manifest, a validity change restored by KNOW-10).
- src/stage.rs: after `stage advance` into done is saved, `applied.librarian_trim`
  carries that result; the move itself never depends on it.

## From the change's testing.md

# Testing

- tests/librarian_trim.rs, with five stub archives (two cold) and a checkpoint at accept:
  - `entering_done_trims_cold_archived_proof_by_default` (REQ-cli-041): the move
    succeeds, `applied.librarian_trim.status` is `trimmed`, the cold observation is
    pruned and ledgered, its authority names KNOW-13.
  - `retention_manual_opts_out_of_trimming_at_done` (REQ-cli-041): `skipped`, nothing
    removed.
  - `a_librarian_off_project_is_not_trimmed_at_done` (REQ-cli-041): `skipped`.
  - `a_recorded_retention_off_is_not_trimmed_at_done` (REQ-cli-041, #97 review):
    `skipped`; fails at 4e755d0.
  - `an_open_native_change_keeps_proof_and_the_move_still_succeeds` (REQ-cli-041):
    `refused` naming the open change; done is recorded; nothing removed.
- Each fails on main 91a1fdd; `fledge lanes run verify` passes on Linux.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-cli-041 | tests/librarian_trim.rs `entering_done_trims_cold_archived_proof_by_default`, `retention_manual_opts_out_of_trimming_at_done`, `a_librarian_off_project_is_not_trimmed_at_done`, `a_recorded_retention_off_is_not_trimmed_at_done`, `an_open_native_change_keeps_proof_and_the_move_still_succeeds` |

## Where these lessons go

- `specs/cli/context.md`
