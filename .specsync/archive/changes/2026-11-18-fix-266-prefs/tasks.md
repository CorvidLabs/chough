---
change: fix-266-prefs
artifact: tasks
---

# Tasks

- [x] Add a test that fails first: pref set and unset write `.chough/preferences.json` mid-conflict.
- [x] Take the writer lock in `pref set` and `pref unset` before reading the preferences.

Human gates (not checkboxes): scope approval, implementation review, merge.
