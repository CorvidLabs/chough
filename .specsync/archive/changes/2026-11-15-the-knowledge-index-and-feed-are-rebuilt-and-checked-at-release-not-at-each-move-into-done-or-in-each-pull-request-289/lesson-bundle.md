# Lesson bundle: the-knowledge-index-and-feed-are-rebuilt-and-checked-at-release-not-at-each-move-into-done-or-in-each-pull-request-289

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: The knowledge index and feed are rebuilt and checked at release, not at each move into done or in each pull request (#289)
- **Kind**: Feature
- **Specs**: cli, records, history
- **Paths**: src/library.rs, src/sessions.rs, src/stage.rs, fledge.toml, .github/workflows/ci.yml, docs, README.md, AGENTS.md, skills, .agents/skills, src/skill.rs, tests
- **Acceptance**: A change's move into done and its pull request leave knowledge/index.md and feed.xml unchanged, no PR or CI lane runs library index --check, and fledge run release-check rebuilds both and passes the strict check before the release PR merges

## Evidence

- Verification commit: `c6b6e3a8f11b71ebf991906445aa93667bc4cb3c`
- Base commit: `4d3c1a5cfddc9e6710be2dad1c39162816adacf0`
- Verified by: `specsync check --spec cli --spec history --spec records --spec reporting`

## From the change's context.md

# Context

Issue #289: every pull request regenerated `knowledge/index.md` and `feed.xml` at its move into done. GitHub ignores the `chough-generated` merge driver, so any two open PRs conflicted on those files.

Leif's choices in Claude Code chat on 2026-11-15:
- **A1, rejected:** a CI push to main needed a bypass that classic branch protection would not take.
- **A3, chosen:** rebuild the index at release.
- **Design B, chosen:** after Grok showed that a relaxed "behind but not wrong" check passed wrong indexes, run only the strict check, and only at release.

## From the change's design.md

# Design

- **The move into done:** `refresh_at_done` reports `deferred` with the index path, or `skipped` with the librarian off, and writes no generated file.
- **`--check`:** `stale_files` is the strict comparison against a rebuild, unchanged from cf2ff72.
- **Lanes:** the verify and CI lanes drop the `knowledge` step. A new `release-check` task runs `library index`, then `library index --check`.
- **Docs:** `docs/releasing.md` rebuilds and commits the index on the release branch before it merges. The skill references and the docs say the vault is refreshed at release.

## From the change's testing.md

# Testing

| Requirement | Evidence |
|---|---|
| REQ-cli-043 | tests/journal.rs: the move into done reports `deferred` and leaves the index and feed byte-identical |
| REQ-cli-047 | tests/vault.rs: a hand-edited linked bullet fails `--check`; a test that the verify lane runs no `--check` and `release-check` does |
| REQ-cli-052 | the existing one-read-into-done test |
| REQ-records-011 | tests/journal.rs: the feed lists the post after `library index` |
| REQ-history-006 | the existing archive-at-done tests |

## Where these lessons go

- `specs/cli/cli.3md`
- `specs/records/context.md`
- `specs/history/context.md`
