---
change: fix-260-a-new-record-is-written-whole-or-not-at-all
artifact: plan
---

# Plan

## Approach

Option A of the design: in `records::write_new_bytes`, write the new record to a
temporary file in its folder, sync it, then give it its name without replacing anything
(`renamex_np(RENAME_EXCL)` on macOS, `renameat2(RENAME_NOREPLACE)` on Linux, and under
the writer lock a free-name check and rename elsewhere). Remove the temporary file on
every error. Ignore SIGXFSZ at start, so a file-size limit is an error the write
returns. Name an empty record file as empty in the reader's message.

## Out of scope

- A sweep of temporary files a SIGKILL leaves beside bugs and journal posts (#385).
- Readers that list the readable records and name the broken one instead of stopping
  (#352's comment of 2026-11-18).
- Replacing writes (`write_replace`), which are already a temporary file and a rename.

## Steps

1. Red tests in `tests/records.rs` for `bug add` and `journal add` under `ulimit -f`
   and for an empty bug file.
2. The whole-or-nothing create-new and the SIGXFSZ setting.
3. The empty-record message.
4. Docs, the case and the changelog; every verify step; tester2 on Linux and a Docker
   Desktop mount; the tester on full volumes, exFAT and the real repositories.

## Risks

- `renamex_np` or `renameat2` unsupported on some file systems (exFAT, network
  mounts): the fallback under the writer lock covers them; tested on exFAT.
- A writer that does not hold the lock would race the fallback: each of the 19
  callers of `write_new_bytes` is to be checked for it in step 2, before the fallback is
  relied on (REQ-cli-040 lists the writers; not yet checked).
- Ignoring SIGXFSZ changes a kill into an error for every write in the process, which
  is what each write's error handling already expects.

## Constraints consulted

REQ-cli-023 (atomic create-new), REQ-records-001 (create-new for a new record), #313
(no hard links: exFAT), REQ-cli-040 (every record writer holds the writer lock),
REQ-cli-068 (readers skip metadata names; a temporary file is not `*.md`).
