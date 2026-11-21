---
change: a-pull-request-that-already-has-its-stage-label-keeps-that-label
artifact: tasks
---

# Tasks

- Add a test where the pull request already has `chough-stage:build` and the session is `build`, and show that the current script removes that label.
- Skip the matching label in the remove loop, and add it only when it is missing, in both script copies.
- Bind that test as a case.
- Refresh the pull request body so it no longer says the session is at build.
- Run the label tests. Push the branch. Do not merge.
