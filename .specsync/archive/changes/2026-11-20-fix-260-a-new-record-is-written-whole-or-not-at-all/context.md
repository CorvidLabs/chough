---
change: fix-260-a-new-record-is-written-whole-or-not-at-all
artifact: context
---

# Context

#260, reopened for 0.6.2 when #306's hard-link writer was reverted (#313, exFAT), and
#352's "cut-off record after a failed write" entry. Every new record goes through
`records::write_new_bytes`: create-new at the final name, then one write, no sync and
no clean-up. A write that fails or is cut off leaves a partial file at the record's name.

Measured on main 056cbb80 (t4b, 2026-11-20, macOS):
- `bug add` with large fields under `ulimit -f 4`: killed by SIGXFSZ (exit 153), leaves
  a 2,048-byte cut-off bug at `.chough/bugs/<date>-truncated-bug.md`, and `chough bug
  list` lists it as a whole bug (exit 0).
- A 0-byte bug (what a full disk leaves, made by hand here): `status` and `next` exit 1
  with "a record starts with a `---` line before its front matter", which does not fit
  an empty file; `chough doctor` reports nothing.
- A 0-byte journal post: `library index --check` fails.
- `ulimit -f 0` dies at the lock file before any record exists, so the 0-byte case here
  comes from ENOSPC, as the issue's tmpfs run shows.

REQ-cli-023 already says "Creation SHALL use atomic create-new", and REQ-records-001
"by create-new for a new record". The code claims the name atomically but not its
content; the fix brings it in line. No refusal is added and no sentence changes: one
criterion is added to each requirement.
