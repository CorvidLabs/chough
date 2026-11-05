# Lesson bundle: record-the-2026-11-04-retention-lessons-in-the-history-lesson-source

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Record the 2026-11-04 retention lessons in the history lesson source
- **Kind**: Documentation
- **Specs**: history
- **Paths**: specs/history/context.md
- **Acceptance**: specs/history/context.md Lessons gains the two supported 2026-11-04 retention lessons (the slow trim at done and trimmed proof without a committed knowledge folder), each with a certainty label, source, future use and limits; no requirement text changes; fledge lanes run verify passes on Linux.

## Evidence

- Verification commit: `a9e43f9a690a7ade4c4fe8a2f381c32214adc5ad`
- Base commit: `2660d7e3a6a2e9ab01f7d304f48a3b51fabc0ba5`
- Verified by: `specsync check --spec history`

## From the change's context.md

# Context

The history module's lesson source said no lessons were recorded yet. On 2026-11-04 Leif
(0xLeif) answered "yes" to folding that day's lessons into the cli and history lesson
sources. The cli lessons came with the knowledge-directory change in this PR (#106).
Files under `specs/` always need a covering change, so the two retention lessons for
`specs/history/context.md` come here: the slow trim at done, and trimmed proof without
a committed knowledge folder. No requirement text changes.

## Where these lessons go

- `specs/history/context.md`
