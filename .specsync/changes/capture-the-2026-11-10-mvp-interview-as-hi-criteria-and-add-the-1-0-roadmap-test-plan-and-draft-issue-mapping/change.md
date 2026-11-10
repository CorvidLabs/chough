---
id: capture-the-2026-11-10-mvp-interview-as-hi-criteria-and-add-the-1-0-roadmap-test-plan-and-draft-issue-mapping
state: draft
type: documentation
base_commit: 996dadc2d26b200a7f745215ef9c151304107e05
---

# Capture the 2026-11-10 MVP interview as hi criteria and add the 1.0 roadmap, test plan and draft issue mapping

## Intent

Capture the 2026-11-10 MVP interview as hi criteria and add the 1.0 roadmap, test plan and draft issue mapping

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- hi check passes with the 13 new criteria (FLOW-21, FLOW-22, DRIVE-24, KNOW-30, LIFE-17 to LIFE-25) and the INTENT.md index regenerated; docs/roadmap-1.0.md defines milestones to 1.0, each tied to one of the four bar items of LIFE-21 with a done definition and its test, and includes the session-slug-folder migration design; docs/test-plan-1.0.md covers the per-repo dogfood checklist, the flake tracker and two-week clean window, migration fixtures from every 0.x tag (0.1.0 to 0.5.0) and the host-matrix e2e harness design; the PR body carries the draft issue list grouped by milestone with acceptance criteria, dependencies and the mapping of existing open issues; no helper behavior, requirement text, stage semantics, retention or SpecSync contract changes

## No-spec Rationale

No requirement text changes: the PR captures Leif's 2026-11-10 interview decisions as new hi criteria (FLOW-21, FLOW-22, DRIVE-24, KNOW-30, LIFE-17 to LIFE-25) and adds roadmap and test-plan documentation. No helper behavior, stage semantics, retention or SpecSync contract changes.
