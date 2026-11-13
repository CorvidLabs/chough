---
change: 0-6-1-at-the-400-observation-cap-every-write-that-adds-an-observation-refuses-before-writing-anything-and-names-chough
artifact: testing
---

# Testing

A fixture session holding 400 observation references: `chough cases run`, `cases
record`, `evidence add` and `checkpoint --observation` each refuse naming the bound and
`chough session shelve <id>`, with every file byte-identical and no case run. At accept,
an unreferenced entry newer than a case's referenced run does not count as its latest
run. Red at the test commit, green at the fix.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-cli-032 | `tests/cases.rs`: at 400 references, observation writes refuse before writing, naming `chough session shelve` |
| REQ-cli-021 | `tests/next.rs`: an unreferenced newer entry is not a case's latest run |
