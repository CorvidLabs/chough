---
change: an-open-session-sets-the-stage-label-when-archived-sessions-are-also-on-the-pull-request
artifact: testing
---

# Testing

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-cli-070 | `tests/github_labels.rs`: `the_script_follows_an_open_session_ahead_of_an_archived_one` |

The test uses the same stand-in `gh` as the other script tests. One run introduces an archived story at `done` and an open session at `intent`, with `chough-stage:done` already applied, and expects the label to become `intent`. Another introduces only the archived story and expects `done`.

`TC-cli-open-session-sets-the-stage-label` binds that test. The earlier label cases stay in their own files.
