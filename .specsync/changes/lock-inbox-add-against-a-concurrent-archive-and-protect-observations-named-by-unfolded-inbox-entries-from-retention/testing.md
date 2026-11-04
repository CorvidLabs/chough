---
change: lock-inbox-add-against-a-concurrent-archive-and-protect-observations-named-by-unfolded-inbox-entries-from-retention
artifact: testing
---

# Testing

- tests/sessions.rs `inbox_add_racing_an_archive_never_strands_its_entry` (REQ-history-009):
  the test holds the project lock as `session use` does, starts `inbox add`, archives alpha
  while still holding it, then releases; the add must refuse with "no open session alpha"
  and leave no entry. Fails at 75a732d (the add does not wait and the entry is stranded).
- tests/evidence.rs `retention_protects_observations_named_by_unfolded_inbox_entries`
  (REQ-history-009): an old observation named only by an unfolded inbox entry is listed as
  protected (`inbox of session alpha`) and survives `evidence prune --apply`. Fails at
  75a732d.
- `fledge lanes run verify` passes on Linux.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-history-009 | tests/sessions.rs `inbox_add_racing_an_archive_never_strands_its_entry`; tests/evidence.rs `retention_protects_observations_named_by_unfolded_inbox_entries` |
