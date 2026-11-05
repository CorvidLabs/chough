---
id: keep-the-knowledge-index-under-its-size-bound-by-listing-recent-archives-and-moving-the-full-list-to-a-monthly-timeline
state: approved
type: bug_fix
base_commit: 7934800355522c0c5f31f1898c4720c0dae7f786
---

# Keep the knowledge index under its size bound by listing recent archives and moving the full list to a monthly timeline

## Intent

Keep the knowledge index under its size bound by listing recent archives and moving the full list to a monthly timeline

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- With hundreds of archived changes, library index succeeds: INDEX.md stays within 65,536 bytes listing the 10 newest archived changes and lesson bundles and linking timeline.md; timeline.md links one page per month, and each archive appears once with its date and lesson bundle, every page within the bound. The new test fails first, fledge lanes run verify passes on Linux, and the refresh runs at closeout on this repository.

## No-spec Rationale

Bug fix within REQ-cli-043 and REQ-cli-044 (the index stays within the entry bound); the timeline is part of confirmed KNOW-28. No requirement text changes.
