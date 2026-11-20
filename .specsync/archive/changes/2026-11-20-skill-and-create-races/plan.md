---
change: skill-and-create-races
artifact: plan
---

# Plan

## Approach

Fix the create-new race at its one source, `write_new_bytes`; read CRLF copies as their LF bytes; let the skill commands skip what the scan already skips; extend doctor's `unreadable` check to untracked records.

## Out of scope

- `next` naming an uninspected host: doctor's advisory finding covers it.
- Following a symlinked skill folder: it stays uninspected.
- The other findings (leaks, oversize, conflict markers) for untracked files: they stay on tracked files, as REQ-records-006 and REQ-history-012 say.

## Steps

1. Failing tests: CRLF, uninspected host, read-only folder, the parallel runs, the uncommitted records.
2. `write_new_bytes` and the skill update loop.
3. CRLF matching and writing.
4. The uninspected host in status and update.
5. Doctor's untracked records.
6. Docs, changelog, cases, verify.

## Risks

- `write_new_bytes` has other callers; treating a lost race as "it exists" is what each already does when the file was there before the call, so their reports stay as they are.
- A CRLF copy updated with CRLF is a working-tree change Git may show as a line-ending change under some `core.autocrlf` settings.

## Constraints consulted

Declare every owning module and one cases file per module; `pub(super)` helpers; no new dependency; a sentence change after approval needs Leif's words.
