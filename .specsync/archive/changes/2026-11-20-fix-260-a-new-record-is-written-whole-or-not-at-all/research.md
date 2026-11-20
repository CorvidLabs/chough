---
change: fix-260-a-new-record-is-written-whole-or-not-at-all
artifact: research
---

# Research

- `write_new_bytes` (src/records.rs:548): create-new at the final name, one
  `write_all`, no sync, no clean-up; 19 call sites outside records.rs.
- #306's hard-link writer (temporary file, then `link` to the name) was reverted for
  exFAT (#313, `tests/no_hard_links.rs`).
- libc 0.2.18x has `renamex_np` (macOS) and `renameat2` (Linux glibc), with
  `RENAME_EXCL` and `RENAME_NOREPLACE`.
- Measured on main (context): a 2,048-byte cut-off bug listed as whole; a 0-byte bug
  stops `status` and `next` with a message that does not fit; `doctor` silent.
