---
id: remove-the-history-events-a-command-published-when-its-save-is-refused-because-the-session-file-vanished-and-test-that
state: verifying
type: bug_fix
base_commit: 75a732d8243181c375ee3c4f2651408ff9b41ab2
---

# Remove the history events a command published when its save is refused because the session file vanished, and test that session use waits for stage advance (issue 82)

## Intent

Remove the history events a command published when its save is refused because the session file vanished, and test that session use waits for stage advance (issue 82)

## Affected Canonical Specs

- `history`

## Acceptance Criteria

- A stage advance whose session file is moved away during its native read refuses with nothing saved and leaves no new event file. Session use started during a stage advance waits for it. Each new test fails where stated and fledge lanes run verify passes on Linux.

## No-spec Rationale

Not applicable
