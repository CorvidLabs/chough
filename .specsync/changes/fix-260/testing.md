---
change: fix-260
artifact: testing
---

# Testing

`cargo test --locked --test bugs a_bug_add_whose_write_fails_part_way_leaves_no_partial_record` (Unix only) runs `chough bug add` with a record of about 12,000 bytes under a file size limit of 4,096 bytes, with the limit's signal ignored so the write returns an error as it does on a full disk. It checks that the command exits non-zero, that `.chough/bugs/` holds no file, and that `bug list`, `status` and `next` exit 0. It fails on main, where a 4,096-byte cut-off record is left. A full disk itself was not reproduced; the test uses the size limit because it gives the same failed write on macOS and Linux without a dedicated file system. Journal posts go through the same writer and have no test of their own here.
