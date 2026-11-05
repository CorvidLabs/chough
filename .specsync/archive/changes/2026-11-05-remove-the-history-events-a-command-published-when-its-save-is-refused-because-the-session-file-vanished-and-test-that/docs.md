---
change: remove-the-history-events-a-command-published-when-its-save-is-refused-because-the-session-file-vanished-and-test-that
artifact: docs
---

# Docs

- docs/cli.md: the sessions section says the refusal removes the events the command wrote.
- CHANGELOG.md: one Unreleased entry citing #82.
- docs/cli.md: only events written since the command last found its checkpoint in place
  are removed, so an event a save may have written never is (second #96 review).
