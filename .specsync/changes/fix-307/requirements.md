---
change: fix-307
artifact: requirements
---

# Requirements

REQ-cli-067 is modified, in `deltas/cli.md`. Its list of findings gains one
entry and its acceptance criteria gain one line. Nothing else in it changes.

Added finding: each module whose spec lists, in its `files:`, an affected path
that is an existing file outside a `tests/`, `test/` or `fixtures/` folder, while
no declared module lists that path. That is when SpecSync 6.3.0's approval
refuses (read from its `validate_declared_path_ownership`). The finding names the
module and the paths it owns, and says that declaring it can add interview
questions. On a draft the fix is one
`chough change answer <id> affected_specs "<the full list>"`; otherwise it is
`specsync change send-back`. A folder, a path that does not exist yet, a test or
fixture file and a path no spec lists are not findings.
