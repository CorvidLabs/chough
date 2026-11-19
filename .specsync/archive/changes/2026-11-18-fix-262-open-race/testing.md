---
change: fix-262-open-race
artifact: testing
---

# Testing

`cargo test --locked --bin chough records::tests::a_fifo_at_the_moment_of_the_open_is_refused_without_blocking` (Unix only) calls `open_regular` and `read_bounded_record` on a FIFO from a thread with a 10 second limit and checks both refuse it as not a regular file, and that a regular file still reads. The reader no longer has a check before its open, so a FIFO present at the open is the replaced-after-the-check case. This test was not seen failing first: it passes on the previous code too, where the earlier path check catches a FIFO that is already there, and the reviewer's replacement between check and open needs an interposer this suite does not have. The two `tests/special_files.rs` tests from fix-262 still pass and cover the commands end to end. Case `TC-history-a-fifo-at-the-open-is-refused-without-blocking` binds the unit test.
