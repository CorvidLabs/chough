---
id: plan-split
state: approved
type: documentation
base_commit: 51b76c626599ce28494a23b1edd740d1dff21854
---

# Plan split

## Intent

plan split

## Affected Canonical Specs

- None

## Acceptance Criteria

- The workflow skill's plan reference teaches the optional execution split (worker count, per-worker file ownership with no shared file, context budget and reading list, where each worker runs, and why that number, fanning out only when cuts share no files, each cut fits the budget, and at least 3 similar history samples show a wall-clock win), the session records worker count, wall-clock, conflicts, inbox questions and stage-backs after the slices, the testing reference teaches the bounded fix-and-rerun loop on the focused set with grouping by cause before filing and one bug or one lesson per group, and the stages and build references agree; tests pin those sentences and the installed skill copy matches the source.

## No-spec Rationale

Skill guidance for the plan-stage execution split and the test-stage grouping loop; no canonical spec sentence changes and no helper behavior changes.
