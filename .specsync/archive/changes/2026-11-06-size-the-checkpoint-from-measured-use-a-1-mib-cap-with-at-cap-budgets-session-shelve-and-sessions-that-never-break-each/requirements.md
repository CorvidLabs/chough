---
change: size-the-checkpoint-from-measured-use-a-1-mib-cap-with-at-cap-budgets-session-shelve-and-sessions-that-never-break-each
artifact: requirements
---

# Requirements

These requirements are MODIFIED (see `deltas/`):
- **REQ-cli-032:** the cap becomes 1,048,576 bytes for every checkpoint. The refusal names `chough session shelve`. At the cap, `next` and `status` answer within 250 ms in release, and the focused view stays within 16 KiB.
- **REQ-cli-007, REQ-cli-034 and REQ-cli-042:** acceptance criteria name the new cap.
- **REQ-cli-021:** AC1's original shape is placed within the cap.
- **REQ-history-006:**
  - `chough session shelve <key>` and `--legacy`;
  - an unreadable session never breaks the others, and `session list` reports it per session;
  - retention also reads shelved sessions.
