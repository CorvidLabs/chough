---
change: prepare-the-chough-0-6-0-release-cargo-0-6-0-and-the-dated-changelog-with-the-dogfood-results-and-the-fixes-the-dogfood
artifact: testing
---

# Testing

Each fix is red before it and green after, recorded through Chough with `chough
cases run`: TC-records-a-0-5-pointer-still-names-its-session (D1) and
TC-cli-empty-change-dir-not-open (#148), TC-cli-gate-summary-counts-from-the-changes-own-base (D2, with #190's TC-cli-190-gate-summary-counts-the-change still green) and TC-records-the-migrate-preview-warns-of-open-changes. The journal: TC-records-journal-post-uses-the-title-and-summary,
TC-records-journal-post-derives-a-title-and-body and TC-records-journal-add-record-backfills-a-story,
each red then green. Temporary paths: TC-records-doctor-case-temp-paths-are-not-leaks,
red then green. After the backfill, every archived story has a post whose `record`
names it, and `chough library index --check` passes. The fresh-eyes fixes each get a case run red then green, linked to its bug; the
CHANGELOG and docs corrections get drift checks (TC-cli-changelog-0-6-0-names-only-what-ships,
TC-cli-docs-name-what-cases-run-returns, TC-records-migration-docs-name-every-key). The KNOW gap list is recomputed from
`hi/knowledge.md` and the records and must come out empty. `tests/changelog.rs`
checks the dated [0.6.0] section and the Cargo version. Full suites run on hosted CI;
local runs are targeted. `chough doctor` is clean after the closeout.
