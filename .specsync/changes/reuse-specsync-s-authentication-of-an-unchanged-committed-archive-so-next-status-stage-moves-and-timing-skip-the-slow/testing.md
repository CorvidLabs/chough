---
change: reuse-specsync-s-authentication-of-an-unchanged-committed-archive-so-next-status-stage-moves-and-timing-skip-the-slow
artifact: testing
---

# Testing

`tests/native.rs` with the counting SpecSync stub: a first read records and later next, status, stage advance and timing make zero reads with the cached label; changed archive byte, policy, stub binary, corrupt entry, active workspace and missing snapshot read live; stale and corrupt-history are not recorded; retention reads stay live; `stage advance --to accept` makes one read and three transitions, and a refused later step records nothing. The tests fail before the fix. `fledge lanes run verify` runs the full suite.
