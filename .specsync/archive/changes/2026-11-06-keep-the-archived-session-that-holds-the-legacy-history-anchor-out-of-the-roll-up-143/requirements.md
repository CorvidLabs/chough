---
change: keep-the-archived-session-that-holds-the-legacy-history-anchor-out-of-the-roll-up-143
artifact: requirements
---

# Requirements

REQ-history-011 (MODIFIED, in `deltas/history.md`):
- The archived session whose history cursor holds the legacy anchor is never rolled into the vault.
- It stays in `.chough/sessions/archive/` in addition to the five and does not count toward them.
- A new acceptance criterion covers seven archived sessions where only the one that would roll first holds the anchor.

Every other sentence of REQ-history-011 and its two existing criteria are unchanged. Trace: KNOW-29 (amended), DRIVE-13, KNOW-13.
