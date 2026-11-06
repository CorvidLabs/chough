---
change: a-session-whose-native-change-is-archived-is-finished-and-abandoned-work-closes-with-a-reason-163-drive-16
artifact: plan
---

# Plan

## Approach

A session is finished when it is at done or when its native change is archived on HEAD, which `library::finalized_at` reads from `finalization.json` with no native call (DRIVE-16, A1). `session new` and `session use` archive finished sessions, not just done ones. With no pointer, finished sessions do not count toward several open. `chough session close <key> --reason` closes abandoned work (B).

## Out of scope

- The move into done and its self-archive (#155, REQ-history-006 as #162 amends it), the KNOW-29 roll-up (REQ-history-011), and `session shelve`.
- Archiving from read-only commands such as `next` and `status`: they only stop counting a finished session.
- Reopening a closed or archived session.

## Steps

1. Capture DRIVE-16 in hi/drive.md. Write four failing tests in tests/sessions.rs and run them on Linux.
2. Get scope approval through the orc.
3. Add `finished(root, session)`. Use it in `archive_done` and in `active()`'s pointerless choice. Add `SessionCommand::Close`.
4. Update docs/cli.md, the skill's checkpoint reference (both copies) and CHANGELOG.md.
5. After #161 and #162 land, merge main, add the cases, run `fledge lanes run verify` on Linux, then the native check last, and open the pull request.

## Risks

See design.md.

## Constraints consulted

DRIVE-16, DRIVE-13, DRIVE-3, REQ-history-006 (with #155's amendment), REQ-cli-037, REQ-history-011, issue #163.
