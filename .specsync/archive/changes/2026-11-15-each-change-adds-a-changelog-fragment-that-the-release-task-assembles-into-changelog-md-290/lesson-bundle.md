# Lesson bundle: each-change-adds-a-changelog-fragment-that-the-release-task-assembles-into-changelog-md-290

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Each change adds a changelog fragment that the release task assembles into CHANGELOG.md (#290)
- **Kind**: Operations
- **Specs**: cli
- **Paths**: CHANGELOG.md, changelog.d, tools/changelog.py, fledge.toml, tests/changelog.rs, AGENTS.md, docs/releasing.md, skills/chough-workflow/references/stages.md, .agents/skills/chough-workflow/references/stages.md, src/skill.rs, tests/fixtures/skill-releases/manifest.json, tests/skill_releases.rs, tests/package.rs
- **Acceptance**: A change adds its entry as changelog.d/<date>-<change-id>.md and CHANGELOG.md is not edited by the change

## Evidence

- Verification commit: `957afb4d8b1833f12c868e3535085455f6540130`
- Base commit: `49b1a252ae47a9e7a5fd8fe257c993d29351a90a`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

Issue #290: every change added bullets to `CHANGELOG.md`'s Unreleased section. `.gitattributes` sets `merge=union`, but GitHub's merge doesn't apply it, so two open PRs conflict on the changelog once one merges. The branch update that fixes it stales SpecSync's verification and costs a reopen and a fresh review. Leif chose fragments in chat on 2026-11-14 ("2: fragments").

The assembler, its fledge task, the tests and the one-time conversion were written by Flint 40B (local, Ollama) through Pi in three rounds. Claude Code's review found that the first version dropped text already under the heading and assembled in file-name order, and that the second allowed a duplicate version heading and did not give main's order back. Claude Code wrote this change's records, `AGENTS.md`, `docs/releasing.md` and the workflow skill's fast path.

## From the change's design.md

# Design

A change writes `changelog.d/<YYYY-MM-DD>-<change-id>.md` holding its bullets. `tools/changelog.py`, run as `fledge run changelog -- --release <X.Y.Z> --date <date>` (or `--release Unreleased`), reads the fragments in reverse name order, so newest first, appends their bullets after the section's existing entries, inserts a new `## [X.Y.Z] - <date>` heading right after Unreleased, and removes the fragments. It refuses, writing nothing, when a heading for that version exists with any date. The same fragments give the same bytes.

The 16 bullets under main's Unreleased became fragments named `NN-<date>-<id>.md`, ranked so that assembly gives main's text back byte for byte. A date-named fragment sorts after every ranked one, so new entries come first; the ranked names go away at the next release. `.gitattributes` keeps `CHANGELOG.md merge=union`, which is harmless and keeps `docs/cli.md`'s declared block equal to it.

## From the change's testing.md

# Testing

`tests/changelog.rs` (13 tests) checks that the changelog names this change before release from `CHANGELOG.md` and `changelog.d/` together; that assembly appends after existing entries with no blank line, inserts a versioned heading after Unreleased, refuses a repeated version with nothing written and fragments kept, gives the same bytes twice, and that the converted fragments give back main's Unreleased bullets in main's order. Claude Code reproduced each case by hand against `tools/changelog.py`, and assembling the converted fragments gave cf2ff72's `CHANGELOG.md` byte for byte. `tests/skill_releases.rs` and `tests/package.rs` pass with the new skill bytes. Checks: `fledge run fmt`, `fledge run test`, `fledge run lint`, `fledge lanes run verify`, `chough change check`.

## Where these lessons go

- `specs/cli/cli.3md`
