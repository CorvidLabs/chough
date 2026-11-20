---
change: fix-260-a-new-record-is-written-whole-or-not-at-all
artifact: plan
---

# Plan

## Approach

Option A of the design: a temporary file in `.chough/local/`, synced, then a rename into
the target folder that never replaces; where that rename is not supported, create-new
at the final name with a sync and removal on error. SIGXFSZ ignored only around each
record write. An empty record named as empty.

## Out of scope

- A sweep of temporary files a SIGKILL leaves in `.chough/local/` (#385).
- Readers that list the readable records and name the broken one instead of stopping
  (#352's comment of 2026-11-18).
- Taking the writer lock in the six sites that create without it.
- Replacing writes (`write_replace`), which are already a temporary file and a rename.

## Steps

1. After D2 merges: red tests in `tests/records.rs` for `bug add` and `journal add`
   under `ulimit -f`, for an empty bug file, and for a case command that still dies of
   SIGXFSZ under a size limit.
2. The whole-or-nothing create-new, its fallback and the scoped SIGXFSZ setting.
3. The empty-record message.
4. Docs, the case and the changelog; every verify step; tester2 on Linux, a Docker
   Desktop mount and exFAT; the tester on full volumes and the real repositories.

## Risks

- The no-replace rename unsupported where a person keeps a project (exFAT, network or
  Docker mounts): the fallback is create-new, never a replace; tested by tester2.
- `.chough/local/` on another file system than a record folder (a knowledge directory
  elsewhere): `EXDEV` takes the fallback.
- Six create-new sites run without the writer lock on some path: the design's fallback
  does not rely on it.

## Constraints consulted

REQ-cli-023 (atomic create-new), REQ-records-001 (create-new for a new record), #313
(no hard links: exFAT), REQ-cli-040 (the writer lock), REQ-cli-068 (readers skip
metadata names), REQ-cli-011 (children of the helper), D1's archive-finish rule that
every removed byte is committed.
