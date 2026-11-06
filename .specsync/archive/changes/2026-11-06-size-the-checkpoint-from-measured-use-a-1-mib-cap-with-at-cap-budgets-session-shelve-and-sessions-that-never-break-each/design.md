---
change: size-the-checkpoint-from-measured-use-a-1-mib-cap-with-at-cap-budgets-session-shelve-and-sessions-that-never-break-each
artifact: design
---

# Design

## Approach

- **The cap:** `SESSION_LIMIT` becomes 1,048,576. The refusal text names the cap and the shelve command for the file that was refused.
- **Per-session reads:** `sessions::keys_in` and readers (`list`, `new`, `use`, the active session's `next`) read each session independently. An unreadable one becomes an entry with `readable: false`, a reason, and "chough session shelve <key>". Only retention (`every_checkpoint`) still refuses on an unreadable session, and it also reads `shelved/` within 16 MiB.
- **Shelve:** `session shelve <key>` checks that the file exists, then moves it with a no-replace rename into `.chough/sessions/shelved/`. It writes `<key>.shelved.json` (size, SHA-256, time, reason), clears `.chough/local/active-session` if it named the key, and holds the checkpoint lock. `--legacy` does the same for `.chough/session.json` as `legacy`, creating `.chough/sessions/` so sessions mode starts.
- **Tests:** the test suite uses one `CHECKPOINT_CAP` constant.

## Risks

- A shelved session no longer drives work, but its bytes stay in place and in Git, and retention still protects what it references.
- Test runtime: the at-cap test writes 1 MiB files. That is fine.

## Constraints consulted

- REQ-cli-032, REQ-cli-021, REQ-history-006.
- DRIVE-14, DRIVE-13.
- The no-replace rule (#109).
- The 16 KiB focused view.
