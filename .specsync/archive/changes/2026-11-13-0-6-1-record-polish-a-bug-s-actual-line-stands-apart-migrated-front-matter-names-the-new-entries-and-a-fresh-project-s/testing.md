---
change: 0-6-1-record-polish-a-bug-s-actual-line-stands-apart-migrated-front-matter-names-the-new-entries-and-a-fresh-project-s
artifact: testing
---

# Testing

`tests/bugs.rs`: a bug's Actual line is its own paragraph. `tests/migrate_evidence.rs`: a
migrated session's `observations` and a bug's `evidence` name the new entries, the old
IDs resolving as aliases. `tests/records.rs`: a fresh project's reading guide names no 0.5.x folder, and
`tests/docs.rs`: the README's init paragraph is one sentence. Red at the test commits, green at the fixes.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-cli-012 | `tests/bugs.rs`: a bug body's Actual line is its own paragraph |
| REQ-records-009 | `tests/migrate_evidence.rs`: migrated front matter names the new entries, keeping old IDs as aliases |
| REQ-records-002 | `tests/records.rs`: a fresh project's reading guide names no folder kept from 0.5.x |
| REQ-cli-016 | `tests/docs.rs`: the README's init paragraph names the host directories without a long parenthetical |
