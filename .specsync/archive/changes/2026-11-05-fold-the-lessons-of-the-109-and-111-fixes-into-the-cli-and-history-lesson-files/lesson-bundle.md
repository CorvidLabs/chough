# Lesson bundle: fold-the-lessons-of-the-109-and-111-fixes-into-the-cli-and-history-lesson-files

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Fold the lessons of the 109 and 111 fixes into the cli and history lesson files
- **Kind**: Documentation
- **Specs**: cli, history
- **Paths**: specs/cli/context.md, specs/history/context.md
- **Acceptance**: The lessons of the #109 and #111 fixes are in specs/cli/context.md and specs/history/context.md in the existing lesson format, each citing a source that resolves, and fledge lanes run verify passes on Linux.

## Evidence

- Verification commit: `c1d88c1a6a752703f98bd5f015fe7421f7c8f450`
- Base commit: `4143558d3f99af0be75ed53eeb6dd4e77b6ef395`
- Verified by: `specsync check --spec cli --spec history`

## From the change's context.md

# Context

Leif asked that Chough learn from everything (2026-11-04), and on 2026-11-05 wrote "DELETE AND SUMMARIZE AND LEARN.. KNOWLEDGE AND COMPOUND". The fixes in #109 and #111 taught lessons that their lesson bundles route to `specs/cli/context.md` and `specs/history/context.md`, but those lessons were never folded in. This change folds them, in the files' existing format: date, title, kind, then what happened, Source, Future use and Limits.

## From the change's design.md

# Design

Nine lessons are appended to the `## Lessons` sections:
- six in cli: the Windows path limit, create-only moves that keep the mode, folder references, per-file ownership, test Git identity, and the merge replay;
- three in history: sessions after legacy adoption, open changes staling each other, and the slow `history show`.

Each one cites its source (a commit, a review or a hosted job), so it can be checked.

## Where these lessons go

- `specs/cli/context.md`
- `specs/history/context.md`
