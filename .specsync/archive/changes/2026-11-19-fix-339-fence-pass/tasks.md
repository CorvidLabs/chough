---
change: fix-339-fence-pass
artifact: tasks
---

# Tasks

- [x] Add a test that fails first: a session with many unclosed fence openers is slow to write.
- [x] Find each opener's close in one bounded pass, keeping the rule that an unclosed fence hides nothing.

Human gates (not checkboxes): scope approval, implementation review, merge.
