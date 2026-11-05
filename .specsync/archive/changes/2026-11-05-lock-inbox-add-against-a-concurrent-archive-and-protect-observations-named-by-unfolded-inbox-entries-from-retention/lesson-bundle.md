# Lesson bundle: lock-inbox-add-against-a-concurrent-archive-and-protect-observations-named-by-unfolded-inbox-entries-from-retention

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Lock inbox add against a concurrent archive and protect observations named by unfolded inbox entries from retention (issue 98)
- **Kind**: BugFix
- **Specs**: history
- **Paths**: src/sessions.rs, tests/sessions.rs, tests/evidence.rs, docs/cli.md, CHANGELOG.md
- **Acceptance**: While another process holds the lock and archives the session inbox add waits and then refuses with no open session and writes no entry. An old observation named only by an unfolded inbox entry is protected and survives evidence prune apply. Each new test fails on 75a732d and fledge lanes run verify passes on Linux.

## Evidence

- Verification commit: `e23690432a2d72c9b1b8d4e479f85cf0d4837ab9`
- Base commit: `75a732d8243181c375ee3c4f2651408ff9b41ab2`
- Verified by: `specsync check --spec history`

## From the change's context.md

# Context

Issue https://github.com/CorvidLabs/chough/issues/98, found in an outside review of #77
and reproduced at 75a732d:

1. `inbox add --session alpha` checks that alpha is open and then writes its entry with
   no lock, so a concurrent `session use beta` can archive a done alpha in between; the
   entry is stranded (4 of 300 unpaused runs; deterministic with a pause).
2. Retention does not count observation IDs named by unfolded inbox entries, so
   `evidence prune --apply` can remove an observation a subagent has reported, and the
   later fold adds a missing ID.

Leif asked on 2026-11-04 for a PR stacked on #77 (the "Open a PR on #77" option).
Criteria: DRIVE-3 and LIFE-8. This branch is stacked on #77 at 75a732d.

The fix stays in history-owned code: retention gathers checkpoint references through
`sessions::every_checkpoint`, its only caller, so that function also returns each
session's unfolded inbox observations, labelled `inbox of session <key>`. src/evidence.rs
(cli-owned) is unchanged.

Authority: Leif's 2026-11-04 standing approval for bounded Chough bug fixes and his
answer above.

## From the change's design.md

# Design

- src/sessions.rs `inbox add`: take `lock_project` before checking the session is open,
  like `session new`, `use`, `adopt` and `fold`; `archive_done` runs under that lock, so
  the add either precedes the switch's unfolded-entry check (the session stays open) or
  follows the archive (the add refuses with nothing written).
- src/sessions.rs `every_checkpoint`: after the open and archived sessions, add one entry
  per session (open or archived) with unfolded `observation` inbox entries, labelled
  `inbox of session <key>`, whose checkpoint holds those observation IDs. Retention then
  protects them like the session's own. An unreadable entry is an error, as an
  unreadable session is.

## From the change's testing.md

# Testing

- tests/sessions.rs `inbox_add_racing_an_archive_never_strands_its_entry` (REQ-history-009):
  the test holds the project lock as `session use` does, starts `inbox add`, archives alpha
  while still holding it, then releases; the add must refuse with "no open session alpha"
  and leave no entry. Fails at 75a732d (the add does not wait and the entry is stranded).
- tests/evidence.rs `retention_protects_observations_named_by_unfolded_inbox_entries`
  (REQ-history-009): an old observation named only by an unfolded inbox entry is listed as
  protected (`inbox of session alpha`) and survives `evidence prune --apply`. Fails at
  75a732d.
- tests/sessions.rs `an_inbox_add_killed_while_writing_leaves_no_partial_entry`
  (REQ-history-009, #99 review): `inbox add` runs with a zero file-size limit, so it is
  killed at its first write. `inbox list` shows only the earlier entry, `status`, `next`
  and `evidence prune` succeed, the temporary file left is ignored by Git, and the next
  checkpoint save removes it. Fails at 2461254 (an empty INB-*.json; `inbox list` fails
  with "EOF while parsing a value").
- tests/evidence.rs `retention_refuses_while_an_unfolded_inbox_entry_cannot_be_read`
  (REQ-history-009, #99 review): with an unreadable entry, `evidence prune --apply`
  refuses, names the entry and leaves the tree unchanged. This covers behaviour 2461254
  already had, so it passes there too.
- `fledge lanes run verify` passes on Linux.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-history-009 | tests/sessions.rs `inbox_add_racing_an_archive_never_strands_its_entry`, `an_inbox_add_killed_while_writing_leaves_no_partial_entry`; tests/evidence.rs `retention_protects_observations_named_by_unfolded_inbox_entries`, `retention_refuses_while_an_unfolded_inbox_entry_cannot_be_read` |

## Where these lessons go

- `specs/history/context.md`
