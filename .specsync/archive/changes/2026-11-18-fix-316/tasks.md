---
change: fix-316
artifact: tasks
---

# Tasks

- [x] Add tests that fail on main: an `sk-` run after a hyphen is not a token, a bug titled "Fix flaky sk-learn tests in CI pipeline" is written, and a committed 0.6.0 bug with `-sk-` in its ID updates and passes `doctor --leaks`; `x-ghp_<token>` is still refused.
- [x] Give `sk-` back 0.6.0's boundary, where a hyphen before the prefix continues a word, and keep #303's boundary for every other form.
- [x] Add the case definition and the changelog fragment.

Human gates (not checkboxes): scope approval, implementation review, merge.
