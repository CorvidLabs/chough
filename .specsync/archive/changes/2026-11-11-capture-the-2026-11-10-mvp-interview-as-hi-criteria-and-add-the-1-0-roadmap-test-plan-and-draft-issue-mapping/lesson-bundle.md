# Lesson bundle: capture-the-2026-11-10-mvp-interview-as-hi-criteria-and-add-the-1-0-roadmap-test-plan-and-draft-issue-mapping

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Capture the 2026-11-10 MVP interview as hi criteria and add the 1.0 roadmap, test plan and draft issue mapping
- **Kind**: Documentation
- **Specs**: cli
- **Paths**: hi, INTENT.md, docs/roadmap-1.0.md, docs/test-plan-1.0.md, docs/ROADMAP.md, README.md
- **Acceptance**: hi check passes with the 22 new criteria (FLOW-21 to FLOW-23, DRIVE-24 to DRIVE-28, KNOW-30 to KNOW-33, LIFE-17 to LIFE-26) and the INTENT.md index regenerated; docs/roadmap-1.0.md defines milestones to 1.0, each tied to one of the four bar items of LIFE-21 with a done definition and its test, ships the human-first redesign (M1 to M3) as one change, Chough 0.6.0, dogfooded on the chough repo before any other repo updates, includes the migration design as settled in the 2026-11-10 design review (session.md frontmatter checkpoint and timeline, agent/ folder, one archived story file per finished session, Markdown records with date+slug names), and re-scopes knowledge to links-never-copies per #235; docs/test-plan-1.0.md covers the per-repo dogfood checklist, the flake tracker and two-week clean window, migration fixtures from every 0.x tag (0.1.0 to 0.5.0) and the host-matrix e2e harness design; the issues exist under the Chough 1.0 milestone: epic #232 with the 19 new issues #213 to #231 and the mapped existing open issues, with #214 and #216 scoped to the settled design; no helper behavior, requirement text, stage semantics, retention or SpecSync contract changes

## Evidence

- Verification commit: `068d4f9ed90f8ef8e14992a242390830e30853b4`
- Base commit: `996dadc2d26b200a7f745215ef9c151304107e05`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

Leif's MVP / 1.0 interview of 2026-11-10 (audit file
`chough-mvp-interview-2026-11-10.md`, held outside this repo) decided what 1.0
means, the four-part bar, the host scope, the session slug-folder design,
digest-committed evidence, KNOW fully in scope, Herdr/Rune out of core, and the
weekly 0.x release cadence. The same day he decided #14 (fix all four case
bindings), #165 (roll-up, first worded copy-never-move, superseded the same day by #235's links-never-copies), #167 (append-only `chough
correct`) and #201 (record agent, model and hc terminal).

This change captures those decisions as hi criteria and turns them into a
roadmap, a test plan and a draft issue list. No code changes.

Constraints: hi IDs are never reused or renumbered; existing criteria are
never reworded or removed, so the FLOW-17/FLOW-21 and KNOW-29/KNOW-30 tensions
are flagged in the PR body rather than edited. DRIVE-23 is reserved by the
in-flight slice-6 branch, so the new DRIVE criterion is DRIVE-24. Docs must
match current main behavior (FLOW-18); the docs drift suite in tests/docs.rs
enforces links and command tables.

## From the change's design.md

# Design

No visuals or interactions change (the design stage was skipped with that
reason recorded). The design content of this change is the session slug-folder
migration design inside `docs/roadmap-1.0.md`: the from-layout and to-layout,
what migrates (active and archived session files into per-session folders, the
shared transitions log partitioned per session with a kept legacy log, nothing
else moving), and how a 0.x checkout upgrades (explicit `chough migrate` with
a digest receipt, fail-closed reads of unmigrated trees, chained per-tag
migration steps, Git history as rollback).

## Where these lessons go

- `specs/cli/cli.3md`
