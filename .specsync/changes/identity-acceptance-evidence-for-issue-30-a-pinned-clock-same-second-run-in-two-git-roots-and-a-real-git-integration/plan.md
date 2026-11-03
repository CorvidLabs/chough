---
change: identity-acceptance-evidence-for-issue-30-a-pinned-clock-same-second-run-in-two-git-roots-and-a-real-git-integration
artifact: plan
---

# Plan

## Approach

Tests only, plus cfg(test) hooks. See design.md.

## Out of scope

Issue #30 criteria 2, 4, 5 and 6; any product behavior change; SKILL.md and references/lifecycle.md.

## Steps

Follow tasks.md in order.

## Risks

A test hook that leaks into release builds. Every hook is cfg(test) only.

## Constraints consulted

specs/cli/context.md lessons: tasks.md checkboxes block `change check`, so cases and human gates stay out of tasks.md.
