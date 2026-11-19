---
change: fix-264
artifact: tasks
---

# Tasks

- [x] Add tests that fail on main: a second same-day migration, and a receipt name taken after the preview.
- [x] The plan names its receipt, with a `-<n>` suffix while the name is taken by another plan, and the name is in the plan view and its digest.
- [x] `--apply` checks the receipt with every other target before it writes anything.
- [x] Document the `receipt` key and the same-day name in `docs/cli.md`.
- [x] Add the case definitions and the changelog fragment.

Human gates (not checkboxes): scope approval, implementation review, merge.
