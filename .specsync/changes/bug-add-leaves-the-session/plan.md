---
change: bug-add-leaves-the-session
artifact: plan
---

# Plan

## Approach

Move the "filed in this session" link from the session's record to the new bug's own record, and read the session's bugs as its checkpoint's list plus the bugs filed in it wherever a session's bugs are shown or counted.

## Out of scope

- `checkpoint --bug` and its 200-link cap, which stay for the checkpoint's own list.
- `bug update --observation`'s join for a bug not filed in the session: it stays.
- The story's 1 MiB bound with very many bugs (#392, 0.6.3).
- A merge driver for `session.md`.

## Steps

1. Failing tests in `tests/bugs.rs`, each red on main.
2. The `session` key and `bug add`; remove `session_takes_a_bug` and `check_save`.
3. `of_session`, `related_to`, the update join.
4. `checkpoint`, `context` and the story list the session's bugs.
5. Docs, changelog, cases (retire the full-session case, retitle the join case), verify.

## Risks

- Order: filed bugs follow the held ones in filing order (`created`, then name); two bugs filed in one second order by name.
- A teammate on v0.6.1 does not count a bug filed with nothing linked as blocking, as v0.6.1 never did.

## Constraints consulted

Rule 5 (prefer removing a rule); DRIVE-12; declare every owning module and one cases file per module; `pub(super)` helpers; no new dependency; a sentence change after approval needs Leif's words.
