---
change: capture-the-2026-11-10-mvp-interview-as-hi-criteria-and-add-the-1-0-roadmap-test-plan-and-draft-issue-mapping
artifact: design
---

# Design

No visuals or interactions change (the design stage was skipped with that
reason recorded). The design content of this change is the session slug-folder
migration design inside `docs/roadmap-1.0.md`: the from-layout and to-layout,
what migrates (active and archived session files into per-session folders, the
shared transitions log partitioned per session with a kept legacy log, nothing
else moving), and how a 0.x checkout upgrades (explicit `chough migrate` with
a digest receipt, fail-closed reads of unmigrated trees, chained per-tag
migration steps, Git history as rollback).
