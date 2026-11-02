---
change: new-project-setup-path-and-sdd-off-hint-document-hi-specsync-init-scaffold-and-specsync-change-adopt-for-a-new-project
artifact: plan
---

# Plan

## Approach

Test first, then a reporting-only change in `src/stage.rs`, then docs. See design.md.

## Out of scope

Chough running `specsync change adopt` or writing the policy. Any change to gate labels or allow/refuse outcomes. SKILL.md, references/lifecycle.md and the README rewrite (PR #44).

## Steps

Follow tasks.md in order.

## Risks

The README edit could conflict with PR #44, so it is limited to one line.

## Constraints consulted

specs/cli/context.md lessons: tasks.md checkboxes block `change check`, so cases and human gates stay out of tasks.md.
