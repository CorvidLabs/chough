---
change: each-change-adds-a-changelog-fragment-that-the-release-task-assembles-into-changelog-md-290
artifact: context
---

# Context

Issue #290: every change added bullets to `CHANGELOG.md`'s Unreleased section. `.gitattributes` sets `merge=union`, but GitHub's merge doesn't apply it, so two open PRs conflict on the changelog once one merges. The branch update that fixes it stales SpecSync's verification and costs a reopen and a fresh review. Leif chose fragments in chat on 2026-11-14 ("2: fragments").

The assembler, its fledge task, the tests and the one-time conversion were written by Flint 40B (local, Ollama) through Pi in three rounds. Claude Code's review found that the first version dropped text already under the heading and assembled in file-name order, and that the second allowed a duplicate version heading and did not give main's order back. Claude Code wrote this change's records, `AGENTS.md`, `docs/releasing.md` and the workflow skill's fast path.
