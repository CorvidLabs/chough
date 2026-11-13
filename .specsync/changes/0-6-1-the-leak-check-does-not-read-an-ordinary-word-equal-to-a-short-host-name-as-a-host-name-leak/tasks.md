---
change: 0-6-1-the-leak-check-does-not-read-an-ordinary-word-equal-to-a-short-host-name-as-a-host-name-leak
artifact: tasks
---

# Tasks

- [x] A failing test: with a short all-letter host name, a record using it as a word is refused, and doctor reports it.
- [x] The fix in `src/leaks.rs`, with the docs and the CHANGELOG.
- [x] Red then green recorded through Chough, and the full local suite.
