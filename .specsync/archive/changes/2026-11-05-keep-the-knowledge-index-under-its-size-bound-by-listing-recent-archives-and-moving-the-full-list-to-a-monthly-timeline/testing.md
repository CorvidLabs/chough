---
change: keep-the-knowledge-index-under-its-size-bound-by-listing-recent-archives-and-moving-the-full-list-to-a-monthly-timeline
artifact: testing
---

# Testing

tests/vault.rs `the_index_stays_bounded_and_a_monthly_timeline_lists_every_archive`: 300 archives with 100-character IDs, split over October and November 2026. `library index` succeeds. `INDEX.md` stays within 65,536 bytes, lists 10 archived changes, and links `timeline.md`. `timeline.md` links both month pages with their counts. Each archive appears on exactly one month page, with its date and lesson bundle, and every page has front matter. Before the fix, `library index` fails with "index exceeds 65536 bytes". `fledge lanes run verify` passes on Linux, including `library index --check`.
