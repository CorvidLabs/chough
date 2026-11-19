---
change: each-change-adds-a-changelog-fragment-that-the-release-task-assembles-into-changelog-md-290
artifact: tasks
---

# Tasks

- [x] `tools/changelog.py` and the `changelog` fledge task: append, insert a release heading, refuse a repeated version, newest first, deterministic.
- [x] Main's Unreleased bullets converted into `changelog.d/` fragments that give the same text back.
- [x] `tests/changelog.rs` reads `CHANGELOG.md` and `changelog.d/` together and tests assembly.
- [x] `AGENTS.md`, `docs/releasing.md`, the workflow skill's fast path (both copies), `KNOWN_PREVIOUS` and the release corpus manifest.

Human gates (not checkboxes): scope approval, implementation review, merge.
