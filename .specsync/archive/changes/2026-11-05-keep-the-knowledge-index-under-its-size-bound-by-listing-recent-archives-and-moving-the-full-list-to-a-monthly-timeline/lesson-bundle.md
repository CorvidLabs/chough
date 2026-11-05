# Lesson bundle: keep-the-knowledge-index-under-its-size-bound-by-listing-recent-archives-and-moving-the-full-list-to-a-monthly-timeline

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Keep the knowledge index under its size bound by listing recent archives and moving the full list to a monthly timeline
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: src/library.rs, tests/vault.rs, docs/cli.md
- **Acceptance**: With hundreds of archived changes, library index succeeds: INDEX.md stays within 65,536 bytes listing the 10 newest archived changes and lesson bundles and linking timeline.md; timeline.md links one page per month, and each archive appears once with its date and lesson bundle, every page within the bound. The new test fails first, fledge lanes run verify passes on Linux, and the refresh runs at closeout on this repository.

## Evidence

- Verification commit: `d4cdacd1ea138999e795f354710d6344ba564ed2`
- Base commit: `7934800355522c0c5f31f1898c4720c0dae7f786`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

The move into done for #112's first change refused to refresh the knowledge directory with "index exceeds 65536 bytes". `knowledge/INDEX.md` lists every archived change and every lesson bundle, one line each, and with 104 archives it has passed the entry bound (main is at 64,918 bytes). Every later move into done would refuse the refresh, and #112's new `library index --check` step could never pass. Leif chose "Widen #112 with the fix" (2026-11-05 09:21 MST, through the orc). KNOW-28, already confirmed, says the full list of archived changes belongs on a timeline page.

## From the change's testing.md

# Testing

tests/vault.rs `the_index_stays_bounded_and_a_monthly_timeline_lists_every_archive`: 300 archives with 100-character IDs, split over October and November 2026. `library index` succeeds. `INDEX.md` stays within 65,536 bytes, lists 10 archived changes, and links `timeline.md`. `timeline.md` links both month pages with their counts. Each archive appears on exactly one month page, with its date and lesson bundle, and every page has front matter. Before the fix, `library index` fails with "index exceeds 65536 bytes". `fledge lanes run verify` passes on Linux, including `library index --check`.

## Where these lessons go

- `specs/cli/context.md`
