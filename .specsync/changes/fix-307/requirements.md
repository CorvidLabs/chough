---
change: fix-307
artifact: requirements
---

# Requirements

REQ-cli-067 is modified, in `deltas/cli.md`. Its list of findings gains one
entry and its acceptance criteria gain one line. Nothing else in it changes.

Added finding: each module whose spec lists an affected path in its `files:`, or
a file under an affected path, while the module is not in `affected_specs`. The
finding names the module and the paths it owns, and says that declaring it can
add interview questions. On a draft the fix is one
`chough change answer <id> affected_specs "<the full list>"`; otherwise it is
`specsync change send-back`. A path no spec lists is not a finding.
