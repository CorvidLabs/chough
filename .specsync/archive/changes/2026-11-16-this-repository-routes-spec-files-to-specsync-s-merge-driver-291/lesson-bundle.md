# Lesson bundle: this-repository-routes-spec-files-to-specsync-s-merge-driver-291

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: This repository routes spec files to SpecSync's merge driver (#291)
- **Kind**: Feature
- **Specs**: cli
- **Paths**: .gitattributes, AGENTS.md, docs/cli.md, tests/package.rs, changelog.d/2026-11-16-291-specsync-merge-driver.md, .chough/cases/cli.specsync-merge-driver.cases.toml
- **Acceptance**: This repository's .gitattributes routes specs/**/*.spec.md and specs/**/requirements.md to merge=specsync, AGENTS.md names specsync merge --install, and two branches that each bump one module's version and Change Log merge locally without conflict markers

## Evidence

- Verification commit: `bd2c14259d78b7c5a5e705a87e39550feef53a61`
- Base commit: `2b50b058ed10c8e02ec8fcd2223aeea98908785d`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

Issue #291. Two PRs touching one module both bump its spec's version and add the same numbered Change Log row, so they always conflict (#261, #253 and #252 did on 2026-11-14). SpecSync ships an ancestor-aware driver for this, `specsync merge-file`, which this repository did not use. Leif chose to start this in chat on 2026-11-14 ("Yes, start #291"). Configuring the driver for every Chough project through `chough doctor` and `chough init` is a follow-up with its own spec wording.

## From the change's design.md

# Design

`specsync merge --install` wrote the two `.gitattributes` lines and this clone's `merge.specsync` config; a comment explains them, and AGENTS.md names the one-time setup beside Chough's two drivers. No visual or interaction change.

## From the change's testing.md

# Testing

- The package test fails without the `.gitattributes` lines and passes with them.
- Manual check in a throwaway clone: two branches each bumped `specs/cli/cli.spec.md` to the same next version with their own Change Log row; with the driver installed, `git merge` finished with no conflict markers, rows renumbered and `version:` advanced to the larger number.

## Where these lessons go

- `specs/cli/cli.3md`
