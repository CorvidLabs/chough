---
change: teach-archive-first-closeout-verify-human-review-native-review-finalize-then-push-and-observe-ci-122-d5
artifact: context
---

# Context

## The finding

Issue #122, audit finding D5. This is the Chough side; spec-sync#881 is its counterpart. The audit was of main `700dad96`.

Chough's guidance tells an agent to push and observe required CI before the human implementation review and finalize. The hosted Archive gate is archive-first: it refuses while any native change is open. So following the guidance meets a red gate and either waits or works around it.

These places teach the wrong order:
- the shape in `skills/chough-workflow/references/lifecycle.md` (lines 16 to 19 and 82);
- step 6 of `references/accept.md`;
- the closeout diagram and text of `docs/lifecycle.md`;
- `docs/walkthrough.md`.

The installed `.agents/` copies carry the same text.

## Expected

Per the issue: local verification, then the real human implementation review, then the scoped native review, then finalize and archive, and only then push and observe CI before merge. The existing no-intervening-commit rule and every human gate stay as they are.

## Priority

Leif, 2026-11-05 18:22 MST via the orc: C1 is the first Chough item of the 95% plan, after KNOW-23.
