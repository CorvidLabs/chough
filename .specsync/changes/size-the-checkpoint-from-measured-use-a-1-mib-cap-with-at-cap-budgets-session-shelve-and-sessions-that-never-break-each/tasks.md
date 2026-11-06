---
change: size-the-checkpoint-from-measured-use-a-1-mib-cap-with-at-cap-budgets-session-shelve-and-sessions-that-never-break-each
artifact: tasks
---

# Tasks

- [ ] Commit the failing tests.
- [ ] Raise the cap to 1,048,576, and name shelve in the refusal.
- [ ] Read sessions independently: report unreadable ones per session, and keep retention strict.
- [ ] Add `chough session shelve <key>` and `--legacy`.
- [ ] Make retention read `shelved/` within 16 MiB.
- [ ] Update the docs, FRD, CHANGELOG and a case file.
- [ ] Run verify on Linux.
