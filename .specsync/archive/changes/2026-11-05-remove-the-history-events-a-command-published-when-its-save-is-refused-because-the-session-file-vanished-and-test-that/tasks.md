---
change: remove-the-history-events-a-command-published-when-its-save-is-refused-because-the-session-file-vanished-and-test-that
artifact: tasks
---

# Tasks

- [x] Tests in tests/sessions.rs (lock regression; vanished-session cleanup fails first)
- [x] Record and discard published events; call it from the vanished-checkpoint refusals
- [x] docs/cli.md and CHANGELOG.md
- [x] Second #96 review: retire published events whenever the command finds its checkpoint in place (every save does just before it writes), and drop the cursor-head search
