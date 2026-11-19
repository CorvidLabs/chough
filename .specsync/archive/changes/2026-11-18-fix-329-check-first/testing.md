---
change: fix-329-check-first
artifact: testing
---

# Testing

`tests/bugs.rs::bug_add_refuses_before_writing_when_the_session_is_full`, red on fix-329 before this change: with the active session holding 200 bug links, `bug add` refuses naming the session and the limit, no new `.chough/bugs` record exists, and every file is unchanged. The fix-329 tests of the join still pass.
