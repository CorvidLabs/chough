---
change: plan-split
artifact: docs
---

# Docs

The deliverable is documentation: the workflow skill files themselves.

- `references/plan.md` gains the optional execution split inside the existing
  one-approval section: how many workers (one unless a split is clearly cheaper,
  cap about 4), per-worker file or module ownership with no shared file, the
  context budget and the short list each worker must read, where each worker runs
  (its own checkout and its own session; a separate clone is the default, a
  worktree an explicit option), and why that number. Fan-out conditions, the 3-sample
  history rule, `chough inbox add` reporting, the post-slice session record
  (worker count, wall-clock, conflicts, inbox questions, stage-backs; tokens only
  when the host reports a number), and replanning inside approved paths versus
  back through scope approval.
- `references/testing.md` gains the bounded fix-and-rerun loop on the focused set,
  grouping by cause before filing, searching existing bugs first, one lesson or
  one bug per fixed group, one open bug per still-open group with every failed
  observation linked, and stopping after a few passes or when the focused set is
  green, naming what remains.
- `references/bugs.md` gains the grouping rule cross-reference (a group, not one
  bug per assertion).
- `references/build.md` gains the worker-ownership pointer to the plan split.
- `references/stages.md` gains the same wording in the fast path where the stage
  steps say build and test.
- `SKILL.md` changes only if the stage table must change.
- The installed copies under `.agents/skills/chough-workflow/` must match the
  source byte for byte.
- A `changelog.d/2026-11-21-plan-split.md` fragment describes the guidance.