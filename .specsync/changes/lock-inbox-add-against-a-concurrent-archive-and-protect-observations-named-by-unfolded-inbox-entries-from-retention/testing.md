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
- tests/sessions.rs `an_inbox_add_killed_while_writing_leaves_no_partial_entry`
  (REQ-history-009, #99 review): `inbox add` runs with a zero file-size limit, so it is
  killed at its first write. `inbox list` shows only the earlier entry, `status`, `next`
  and `evidence prune` succeed, the temporary file left is ignored by Git, and the next
  checkpoint save removes it. Fails at 2461254 (an empty INB-*.json; `inbox list` fails
  with "EOF while parsing a value").
- tests/evidence.rs `retention_refuses_while_an_unfolded_inbox_entry_cannot_be_read`
  (REQ-history-009, #99 review): with an unreadable entry, `evidence prune --apply`
  refuses, names the entry and leaves the tree unchanged. This covers behaviour 2461254
  already had, so it passes there too.
- `fledge lanes run verify` passes on Linux.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-history-009 | tests/sessions.rs `inbox_add_racing_an_archive_never_strands_its_entry`, `an_inbox_add_killed_while_writing_leaves_no_partial_entry`; tests/evidence.rs `retention_protects_observations_named_by_unfolded_inbox_entries`, `retention_refuses_while_an_unfolded_inbox_entry_cannot_be_read` |
