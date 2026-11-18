---
change: fix-262-open-race
artifact: design
---

# Design

One function, `open_regular` in `src/records.rs`, opens a path for reading and returns it only when the opened handle is a regular file.

- On Unix the open carries `O_NONBLOCK`, so opening a FIFO returns at once instead of waiting for a writer. The type is then read from the opened handle with `fstat`, not from the path, so there is no gap between a check and the open for a replacement to slip into. `O_NONBLOCK` changes nothing for reading a regular file.
- On other platforms there are no FIFOs to block on; the path check stays before the open.
- `read_bounded_record` reads through `open_regular` and drops its earlier path check.
- `session shelve` reads `session.md` and each file it lists through `open_regular` too.

Rejected: keeping the path check and adding a second check after the open. A blocking open of a FIFO never returns, so a check after it never runs.

Risk: `libc::O_NONBLOCK` is used directly; `libc` is already a Unix dependency of this crate.
