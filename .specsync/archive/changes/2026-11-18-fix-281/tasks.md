---
change: fix-281
artifact: tasks
---

# Tasks

- [x] Add a test that fails on main: a session that needs no native change reaches done and is archived as done.
- [x] `chough checkpoint --no-native-change <reason>` records the reason in the session; a native change and the reason never coexist.
- [x] A gated move with no native change is allowed and labelled `ungated:no-native-change-required` while the reason is recorded.
- [x] Document the flag and the label in `docs/cli.md`.
- [x] Add the case definition and the changelog fragment.

Human gates (not checkboxes): scope approval, implementation review, merge.
