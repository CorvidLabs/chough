---
change: integrate-merged-main-134ee19-pr-46-report-reads-into-pr-48-identity-acceptance-evidence
artifact: design
---

# Design

- **Checkpoint.** Run `chough checkpoint --merge` with main's three-way merge, against base 16fe55d. Theirs has its current-cycle group set to base values, so that PR #48 (the owner of this branch) keeps its cycle; all collections are merged three ways. Result: 197 observations equal to main's set (none of the 24 removals come back), 37 file refs equal to main's, and 64466 bytes.
- **Transitions.** Main's lines in their order, followed by the 10 lines PR #48 added after base.
- **Text conflicts.**
  - CHANGELOG: main's Unreleased entry first, then PR #48's Tests entry.
  - cli.spec.md: PR #48's history row is renumbered to 25 and the front matter to `version: 25`.
- **Test.** `src/main.rs` reads freshness through `evidence::Reads::new(&a).observations(&current, Some("local"))` and asserts that each of the two records is `current-observation`.
- **Requalification.** The PR #48 cases are rerun at the combined source, and the older identity observations are kept as history.
