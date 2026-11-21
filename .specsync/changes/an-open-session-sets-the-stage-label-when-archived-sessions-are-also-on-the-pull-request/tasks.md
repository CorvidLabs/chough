---
change: an-open-session-sets-the-stage-label-when-archived-sessions-are-also-on-the-pull-request
artifact: tasks
---

# Tasks

- The test expects an open `intent` session to replace an archived `done` label, and an archived-only `done` story to apply `done`.
- Both script copies choose the stage from open sessions when any remain, and from archived stories only when none remain.
- The template README states that rule.
- The test is bound as `TC-cli-open-session-sets-the-stage-label`, and the changelog fragment is added.
- The label tests passed. Push the branch after the review gate is open. Do not merge. The implementation review stays open until a person reviews it.
