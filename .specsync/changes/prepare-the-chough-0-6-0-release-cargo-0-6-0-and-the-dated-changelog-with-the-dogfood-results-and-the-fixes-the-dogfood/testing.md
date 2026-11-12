---
change: prepare-the-chough-0-6-0-release-cargo-0-6-0-and-the-dated-changelog-with-the-dogfood-results-and-the-fixes-the-dogfood
artifact: testing
---

# Testing

Each fix is red before it and green after, recorded through Chough with `chough
cases run`: TC-records-a-0-5-pointer-still-names-its-session (D1) and
TC-cli-empty-change-dir-not-open (#148). The KNOW gap list is recomputed from
`hi/knowledge.md` and the records and must come out empty. `tests/changelog.rs`
checks the dated [0.6.0] section and the Cargo version. Full suites run on hosted CI;
local runs are targeted. `chough doctor` is clean after the closeout.
