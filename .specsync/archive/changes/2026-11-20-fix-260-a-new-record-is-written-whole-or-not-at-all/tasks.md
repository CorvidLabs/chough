---
change: fix-260-a-new-record-is-written-whole-or-not-at-all
artifact: tasks
---

# Tasks

- [x] Failing tests first in `tests/records.rs`, each red on main: `bug add` and
      `journal add` under a file-size limit leave no file at the new name and no
      temporary file; an empty bug file is named as empty.
- [x] The whole-or-nothing create-new in `src/records.rs`, SIGXFSZ ignored in
      `src/main.rs`, the empty-record message in `src/markdown.rs`.
- [x] `docs/cli.md`, the case definition and the changelog fragment.

Human gates (not checkboxes): scope approval, implementation review, merge.
