---
change: a-session-archives-itself-at-its-own-move-into-done-155
artifact: plan
---

# Plan

## Approach

At a successful move into done, the session archives itself as the last done step, in its own change, and the active pointer is cleared (REQ-history-006, DRIVE-13). This change's own `session new` archives the done sessions open on main: that is the one-time sweep.

## Out of scope

- Any change to `archive_done` on `session new` and `use`, the KNOW-29 roll-up (REQ-history-011), the trim, the refresh or the rotation.
- Archiving at any stage other than done, and reopening an archived session.
- The legacy single-checkpoint mode.

## Steps

1. Write three failing tests in tests/reduce.rs (archived, kept open, skipped) and run them on Linux.
2. Get scope approval through the orc.
3. Add `sessions::archive_at_done` and call it last in the done steps, re-pinning the checkpoint to the archived file.
4. Update docs/cli.md, the skill's checkpoint reference (both copies) and CHANGELOG.md.
5. Add cases. Run `fledge lanes run verify` on Linux, then the native check last. Open the pull request.

## Risks

See design.md: the archive holds six sessions between moves into done; a refused step is reported and never undoes the move into done.

## Constraints consulted

DRIVE-13, DRIVE-3, REQ-history-006, REQ-history-008, REQ-history-011, issue #155 (option D).
