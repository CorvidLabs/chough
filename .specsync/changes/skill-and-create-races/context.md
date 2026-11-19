---
change: skill-and-create-races
artifact: context
---

# Context

Issues #373 (lead-F5, F6), #386 (s3-F2, F7), #403 (from the check of #362) and #408 (t4b, on main), with fable-lead's addition of uncommitted unreadable records beside #408 (the testers' triage notes).

Measured on main (9d26031), after #362:
- #373: an unreadable skill file now reads as `edited` (#362), so the assertion text no longer comes from it; it still comes from the race (#386). A missing file in a read-only folder gives a bare `Permission denied (os error 13)` naming no file. A CRLF copy reads `edited` and is never updated. With a symlinked host folder, doctor reports `skill_uninspected` (#362) and next says nothing.
- #386: eight parallel `skill update` runs over missing files fail 0 to 7 of 8 with `File exists (os error 17)` or `Condition failed: write_new(...)`; six parallel `init --objective` runs fail 1 to 5 of 6 with a bare `File exists`; the end state is usable. The cause is `write_new_bytes`, which checks for the file and then opens it create-new, with no path in the error.
- #403: plain `skill status` and `skill update` exit 1 with `symlink is not an allowed source: <absolute path>/.codex/skills`; `--host openai` works.
- #408: doctor walks tracked files only, so an uncommitted session failing validation, an uncommitted 0-byte evidence entry and an uncommitted cut-off bug get no finding while `next` and `status` stop on them; committed ones are reported. A FIFO in an evidence folder stops `status` and `next` ("reading <file>") while doctor is silent: Git lists no FIFO as untracked, so the walk reads the folders, not Git's list.

Based on main: no requirement here is modified by #407, D1, L2 or #311. #408's sentence is a new requirement, not D1's REQ-history-012, so it does not wait on D1's send-back.
