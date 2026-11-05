# Lesson bundle: move-old-proof-into-the-knowledge-vault-trim-loose-artifacts-of-cold-changes-and-move-key-items-into-knowledge

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Move old proof into the knowledge vault: trim loose artifacts of cold changes and move key items into knowledge attachments (KNOW-18, KNOW-19)
- **Kind**: Feature
- **Specs**: cli
- **Paths**: src/evidence.rs, src/attachments.rs, src/library.rs, src/vault.rs, src/main.rs, src/skill.rs, tests/reduce.rs, tests/evidence.rs, tests/librarian_trim.rs, tests/vault.rs, skills/chough-workflow/references/librarian.md, docs/cli.md, docs/FRD.md, CHANGELOG.md
- **Acceptance**: Loose artifact entries no record references are trimmed and ledgered as loose artifacts once the change that listed or made them is cold, and listed in its knowledge note; entries of current work, named by the checkpoint or an open bug, or listed by a hot archive stay; a closed bug releases its fixture. Key items of a cold change (manual or agent-assisted observations whose only protection is being non-routine, failed-run logs, screenshots and failure-first logs) move to knowledge/attachments/<archive>/, mapped in the ledger and linked from the note. New tests fail first, the trim on this repository is measured, and fledge lanes run verify passes on Linux.

## Evidence

- Verification commit: `17b7a1c3301b77a6883670d7d1871d3836c99f07`
- Base commit: `784b1406d9e5b4b3a53176c3772aaeb003483412`
- Verified by: `specsync check --spec cli --spec history`

## From the change's context.md

# Context

Leif asked on 2026-11-04 to make sure "all of our artifacts and evidence are getting
like deleted and transitioned over to the knowledge vault", not left in `.chough/`. He
chose "Loose artifact folders" (KNOW-18) and "Key items move" (KNOW-19), and confirmed
KNOW-16 to KNOW-19.

#108 reduced observation proof. This change covers what #108 left: artifacts no record
references, and the key items, which now move into the vault instead of staying in
`.chough/` or being deleted. The lessons from #107 and #108 are folded into the lesson
sources at closeout.

## From the change's design.md

# Design

- **`attachments::is_key_item(path)`:** screenshot extensions, or a lowercased name
  containing `failure-first`, `failing-first`, `fails-first`, `before-fix`,
  `pre-fix` or `reproduction`.
- **`attachments::attachment(dir, archive, file)`:** gives
  `<dir>/attachments/<archive>/<file minus .chough/>`.
- **`attachments::first_commits(root)`:** one `git log --diff-filter=A
  --format=@%ct --name-only -- .chough/artifacts`, keeping the oldest time per file.
- **`attachments::loose_groups`:** groups the unreferenced files by top-level entry,
  skipping hidden ones. It applies the checkpoint and open-bug protections and the
  hot-archive protection, then picks the owner: a cold lister, or by cycle time from the
  earliest first commit.
- **`SummaryCandidate`:** gains `loose` and `moves`.
  - An observation set whose only reasons are non-routine moves whole.
  - Otherwise the artifacts of a failed set, and its key-item artifacts, move.
  - A loose group moves its key items.
- **Apply:** loads evidence only for observation sets. Loose groups are ledgered with
  `kind: loose-artifact`, `removed`, `moved` and `artifacts` digests. Moved files
  are copied before removal, and a restore deletes the copies. The entries are written
  again after a successful apply.
- **Archive notes:** gain "Loose artifacts" (from the ledger) and "Attachments" (the
  files under `attachments/<archive>/`) sections, with links.

## From the change's testing.md

# Testing

New tests in tests/reduce.rs, each failing first:
- Loose artifacts of a cold change are trimmed (listed by a cold archive, or committed in
  its cycle), and current or bug-held ones stay.
- A closed bug releases its loose fixture.
- Key items move into the vault (a manual observation with a screenshot, and an older
  failed run's log), and a routine record is removed.

The existing reduce, evidence, librarian and vault tests keep passing. The trim is
measured on this repository, and `fledge lanes run verify` runs on Linux.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-cli-046 | tests/reduce.rs `loose_artifacts_of_a_cold_change_are_trimmed_and_current_or_bug_held_ones_stay`, `a_closed_bug_releases_its_loose_fixture`, `key_items_of_a_cold_change_move_into_the_vault`; tests/evidence.rs `archive_summary_previews_cold_archive_proof_and_keeps_protections`, `archive_summary_apply_is_authorized_ledgered_and_restores_on_validity_change`; src/attachments.rs unit tests |

## Where these lessons go

- `specs/cli/context.md`
