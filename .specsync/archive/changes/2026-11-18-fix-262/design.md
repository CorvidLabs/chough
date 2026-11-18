---
change: fix-262
artifact: design
---

# Design

Opening a FIFO for reading blocks until a writer appears, so the fix is to never open one.

1. `read_bounded_record` in `src/records.rs` is the one reader every Chough record goes through. It now checks that the path is a regular file before it opens it and refuses otherwise, naming the path. This alone ends every hang.
2. The three folder readers in `src/evidence.rs` that `status` and `next` use (receipts of bugs and stories, the observation index, the receipt views) skip an entry that is not a regular file. This matches the bug readers in `src/bugs.rs`, so `status`, `next` and `bug list` agree that such an entry is not a record.
3. `open_ids` and `active` in `src/sessions.rs` asked whether `session.md` is a regular file, so a folder with a FIFO there was not a session at all. They now ask whether it exists. The folder is then a session whose record cannot be read, and the existing path in `session list` reports it with the reason and `chough session shelve <id>`.
4. `shelve` read `session.md` to describe it. It now reads it only when it is a regular file, and the shelved note lists a non-regular entry as `not a regular file, not read` instead of as a symbolic link.

Rejected: opening with `O_NONBLOCK`. It needs platform-specific code and still leaves a device or directory to handle; a metadata check covers all of them.

Risk: a session folder whose `session.md` is a directory or a device is now listed as not readable where it was ignored before. That is what REQ-history-006 asks for.
