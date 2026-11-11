---
id: capture-the-2026-11-10-mvp-interview-as-hi-criteria-and-add-the-1-0-roadmap-test-plan-and-draft-issue-mapping
state: archived
type: documentation
base_commit: 996dadc2d26b200a7f745215ef9c151304107e05
---

# Capture the 2026-11-10 MVP interview as hi criteria and add the 1.0 roadmap, test plan and draft issue mapping

## Intent

Capture the 2026-11-10 MVP interview as hi criteria and add the 1.0 roadmap, test plan and draft issue mapping

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- hi check passes with the 22 new criteria (FLOW-21 to FLOW-23, DRIVE-24 to DRIVE-28, KNOW-30 to KNOW-33, LIFE-17 to LIFE-26) and the INTENT.md index regenerated; docs/roadmap-1.0.md defines milestones to 1.0, each tied to one of the four bar items of LIFE-21 with a done definition and its test, ships the human-first redesign (M1 to M3) as one change, Chough 0.6.0, dogfooded on the chough repo before any other repo updates, includes the migration design as settled in the 2026-11-10 design review (session.md frontmatter checkpoint and timeline, agent/ folder, one archived story file per finished session, Markdown records with date+slug names), and re-scopes knowledge to links-never-copies per #235; docs/test-plan-1.0.md covers the per-repo dogfood checklist, the flake tracker and two-week clean window, migration fixtures from every 0.x tag (0.1.0 to 0.5.0) and the host-matrix e2e harness design; the issues exist under the Chough 1.0 milestone: epic #232 with the 19 new issues #213 to #231 and the mapped existing open issues, with #214 and #216 scoped to the settled design; no helper behavior, requirement text, stage semantics, retention or SpecSync contract changes

## No-spec Rationale

No requirement text changes: the PR captures Leif's 2026-11-10 interview decisions as new hi criteria (FLOW-21, FLOW-22, DRIVE-24, KNOW-30, LIFE-17 to LIFE-25) and adds roadmap and test-plan documentation. No helper behavior, stage semantics, retention or SpecSync contract changes.
