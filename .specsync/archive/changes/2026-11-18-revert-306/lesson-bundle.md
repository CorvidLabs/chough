# Lesson bundle: revert-306

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Revert 306
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: src/records.rs, tests/bugs.rs, tests/no_hard_links.rs, .chough/cases/records.no-partial-new-record.cases.toml, .chough/cases/records.no-hard-links.cases.toml, changelog.d/2026-11-18-fix-260.md, changelog.d/2026-11-18-revert-306.md
- **Acceptance**: On an exFAT volume chough init, chough bug add and chough journal add succeed and write whole records, as in 0.6.0. The create-new writer in src/records.rs is the one 0.6.0 shipped. #305's non-blocking reader and #308's journal lock are unchanged.

## Evidence

- Verification commit: `62cf3cc4b160968508c38e86124449659d939ba6`
- Base commit: `66543c2bf509627ec2a165414acae0bc8ab59c4a`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

#306 (the fix for #260, merged 2026-11-18 as be6c9f1) changed how a new record is
written: the bytes go to a temporary file under `.chough/local/`, which is then
hard-linked to the record's name. Two regressions were found before 0.6.1:

- #313: a file system without hard links, such as exFAT, refuses the link
  ("Operation not supported (os error 45)"), so `chough init`, `bug add`,
  `journal add` and `session new` fail, and `init` leaves a half-made `.chough/`.
- The temporary file is written before the writer lock is taken, and a writer
  holding the lock sweeps it as left by a stopped write. The tester's parallel
  checkpoint and evidence storms failed in 3 of 4 runs on main and 0 of 3 on
  0.6.0.

Leif's decision, relayed by his planning session (hc t4c) on 2026-11-18:
"Revert #306". This change reverts #306's code, its test, its case and its
changelog fragment. It keeps #305's non-blocking reader and #308's journal
lock. The archived change fix-260 and its session story stay as history. #260
reopens for 0.6.2, to be redone with an exFAT test and a parallel sweep test
from the start.

A fix-forward fallback for #313 was started and is parked, unpushed.

## From the change's design.md

# Design

Nothing visual changes. The create-new writer opens the record's name exclusively and
writes to it, as in 0.6.0. No command, flag or output changes.

## From the change's testing.md

# Testing

`tests/no_hard_links.rs::new_records_are_written_on_a_volume_without_hard_links`
makes and mounts a 64 MB exFAT image with `hdiutil`, checks that the volume
refuses a hard link, and runs `chough init`, `bug add` and `journal add` on it.
It fails on main (be6c9f1 and later) with "Operation not supported (os error
45)" and passes with the revert. It is macOS only and ignored by default,
because `hdiutil` can hang and does not exist on Linux CI; it is run by hand
with `-- --ignored` and that run is recorded as evidence.

The #260 test and unit test that #306 added are removed with the writer they
tested. The parallel temporary-file sweep has no test in this change; the
tester's repro script checks it.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-records-001 | the exFAT test above, run by hand and recorded with `chough evidence add` |

## Where these lessons go

- `specs/cli/cli.3md`
