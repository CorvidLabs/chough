---
change: size-the-checkpoint-from-measured-use-a-1-mib-cap-with-at-cap-budgets-session-shelve-and-sessions-that-never-break-each
artifact: tasks
---

# Tasks

- [x] Commit the failing tests.
- [x] Raise the cap to 1,048,576, and name shelve in the refusal.
- [x] Read sessions independently: report unreadable ones per session, and keep retention strict.
- [x] Add `chough session shelve <key>` and `--legacy`.
- [x] Make retention read `shelved/` within 16 MiB.
- [x] Update the docs, FRD, CHANGELOG and a case file.
- [x] Run verify on Linux.
