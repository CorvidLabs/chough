---
change: keep-the-archived-session-that-holds-the-legacy-history-anchor-out-of-the-roll-up-143
artifact: design
---

# Design

## Approach

In `sessions::roll_up`, each archived session is already read to find its native change. The same read also checks `history_cursor.legacy`, and a session with that field set is left out of the ranking. So it never moves, and the five are counted among the others. Nothing else changes:
- ordering, the no-replace rule and the byte-for-byte move;
- reporting (`applied.session_rollup`);
- `adopted_anchor`, which still finds an anchor already rolled into the vault or shelved (#141).

An archived session that can't be read stays where it is, as today.

## Risks

- **Several archived sessions claiming the anchor:** only possible in hand-made fixtures. All of them would stay; that's safe, and nothing is lost.
- **An anchor holder already rolled into the vault** before this change: it stays there, and #141 still finds it.

## Constraints consulted

KNOW-29 (amended 2026-11-06), REQ-history-011, REQ-history-002, #141.
