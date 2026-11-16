---
change: the-knowledge-index-and-feed-are-rebuilt-and-checked-at-release-not-at-each-move-into-done-or-in-each-pull-request-289
artifact: design
---

# Design

- **The move into done:** `refresh_at_done` reports `deferred` with the index path, or `skipped` with the librarian off, and writes no generated file.
- **`--check`:** `stale_files` is the strict comparison against a rebuild, unchanged from cf2ff72.
- **Lanes:** the verify and CI lanes drop the `knowledge` step. A new `release-check` task runs `library index`, then `library index --check`.
- **Docs:** `docs/releasing.md` rebuilds and commits the index on the release branch before it merges. The skill references and the docs say the vault is refreshed at release.
