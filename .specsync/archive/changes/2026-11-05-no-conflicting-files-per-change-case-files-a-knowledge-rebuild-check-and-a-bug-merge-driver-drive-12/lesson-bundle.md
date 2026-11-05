# Lesson bundle: no-conflicting-files-per-change-case-files-a-knowledge-rebuild-check-and-a-bug-merge-driver-drive-12

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: No conflicting files: per-change case files, a knowledge rebuild check and a bug merge driver (DRIVE-12)
- **Kind**: Feature
- **Specs**: cli
- **Paths**: src/cases.rs, src/library.rs, src/bugs.rs, tests/cases.rs, tests/bugs.rs, tests/vault.rs, docs/cli.md, docs/FRD.md, CHANGELOG.md, fledge.toml, .gitattributes, AGENTS.md
- **Acceptance**: Parallel pull requests that each add their own case file, edit one bug and rebuild the vault merge without hand-resolved conflicts in files Chough writes: case files per change load together with IDs unique; library index --check fails naming stale or missing generated files and passes after library index (line endings ignored), and the verify and ci lanes run it; chough bug merge merges two edits of one bug and leaves a real disagreement as a conflict with ours unchanged. The new tests fail first, a parallel pull request simulation merges clean, and fledge lanes run verify passes on Linux.

## Evidence

- Verification commit: `fa6bab4853a5bfe5dab775ccf837cb86ce7a9c38`
- Base commit: `eb35dd3a1d9ca5ce12dee7fd50e7224432241fee`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

DRIVE-12, widened on 2026-11-05: parallel pull requests that each record Chough work merge with no hand-resolved conflict in any file Chough writes. #111 adopted per-change sessions and event history here, and gave the retention ledger and CHANGELOG.md Git's union driver. The 89-merge replay found these still conflicting:
- **Case definitions:** `.chough/cases/cli.cases.toml` conflicted in 31 merges. Every change appends to one file per module.
- **Generated knowledge:** `knowledge/` conflicted in 16 merges. The index, module, intent and archive notes are rebuilt by every move into done.
- **Bug records:** two branches noting or verifying one bug both edit the same JSON file.

Evidence and history events are immutable files with unique names, and finished sessions are archived by identical renames, so they don't conflict.

## From the change's design.md

# Design

- **Cases:** `load` accepts `<module>.cases.toml` and `<module>.<part>.cases.toml`. The module is the stem's first dot-separated segment and must equal the file's `module` field. `<part>` is a valid module-style name. Case IDs stay unique across files, as now. Each change adds its cases in a file of its own, for example `cli.no-conflicting-files.cases.toml`.
- **Knowledge:** `chough library index --check` builds every generated file in memory, writes nothing, and fails, naming each file that is missing or differs from the rebuild, with CRLF read as LF. The verify and ci lanes run it as a `knowledge` task. `.gitattributes` gives `knowledge/**` the union driver: a merge never stops on the vault, and the check makes sure it is rebuilt with `chough library index` before CI passes.
- **Bugs:** `chough bug merge <base> <ours> <theirs>` is a three-way merge over the record:
  - A field only one side changed takes that change.
  - Lists (`notes`, `observations`, `cases`, `criteria`, `requirements`, `steps`, `related`) keep base's entries, then ours' additions, then theirs', without duplicates. An entry either side removed is removed.
  - `updated_at_unix_seconds` is the later one.
  - Any other field that both sides changed differently, such as status or scope, is a conflict. `ours` is left unchanged and the command exits nonzero for a human to resolve.
  - The result must parse as a bug record, and it is written the way `bug` commands write one.
- `.gitattributes` declares `.chough/bugs/*.json merge=chough-bug`. AGENTS.md and docs/cli.md show the one-time `git config` for both Chough drivers.

## From the change's testing.md

# Testing

Each of these fails before the change:
- tests/cases.rs `cases_split_per_change_load_together_and_ids_stay_unique`: `cli.cases.toml` and `cli.extra.cases.toml` load together, and a case ID defined in both files is refused.
- tests/vault.rs `index_check_names_stale_files_and_passes_after_a_rebuild`: after `library index`, the check passes. It fails naming `INDEX.md` after that file is edited, and passes again after a rebuild. A CRLF copy of a current note passes.
- tests/bugs.rs `two_edits_of_one_bug_merge_and_a_real_disagreement_stays_a_conflict`: a base and two edited copies of one bug. One adds a note and verifies it; the other adds a note and an observation. They merge with both notes, the observation, the verified status and the later update time. Two different statuses exit nonzero and leave ours unchanged.

A scripted simulation on Linux: two branches from this branch's tip each start a session, add a case in their own file, record a case run, edit one shared bug, rebuild the vault, append a ledger line and add a CHANGELOG bullet. With both drivers configured, `git merge` is clean, and after `chough library index` the check passes. `fledge lanes run verify` passes on Linux, including the new knowledge step.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-cli-047 | tests/cases.rs `cases_split_per_change_load_together_and_ids_stay_unique`, tests/vault.rs `index_check_names_stale_files_and_passes_after_a_rebuild`, tests/bugs.rs `two_edits_of_one_bug_merge_and_a_real_disagreement_stays_a_conflict` |

## Where these lessons go

- `specs/cli/context.md`
