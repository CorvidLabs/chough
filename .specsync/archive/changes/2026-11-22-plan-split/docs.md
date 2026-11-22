---
change: plan-split
artifact: docs
---

# Docs

The deliverable is documentation: the workflow skill files themselves.

- `references/plan.md` gains an optional fork inside the existing one-approval
  section. One question: can the slice be cut into a left half and a right half
  with no shared file? If it cannot, one worker does the whole slice. If it can,
  one manager spawns two workers, each in its own clone and session, with no
  shared file. Do not use a worktree for a fork. A worker that finishes stops.
  Before the merge, a path outside that worker's list is left out. The manager does
  not write either half and merges both reports into one answer. One fork is the batch. A half is
  forked again only when it still has its own seam, and the total stays at about
  four agents. Chough does not launch the agents. After the merge the session
  records whether the halves joined cleanly, the wall-clock, and any conflict.
- `references/testing.md` gains the bounded fix-and-rerun loop on the focused set,
  grouping by cause before filing, searching existing bugs first, one lesson or
  one bug per fixed group, one open bug per still-open group with every failed
  observation linked, and stopping after a few passes or when the focused set is
  green, naming what remains.
- `references/bugs.md` gains the grouping rule cross-reference (a group, not one
  bug per assertion).
- `references/build.md` says that when the plan forked the slice, each worker edits
  only the files in its half, and the manager does not write either half.
- `references/stages.md` gains the same wording in the fast path where the stage
  steps say build and test.
- `SKILL.md` changes only if the stage table must change.
- The installed copies under `.agents/skills/chough-workflow/` must match the
  source byte for byte.
- A `changelog.d/2026-11-21-plan-split.md` fragment describes the guidance.
