---
change: fix-286
artifact: testing
---

# Testing

- `tests/evidence.rs::a_story_receipt_carries_each_field_its_entry_knows` now adds an
  entry with two limitations and expects, on the archived story's receipts:
  `criteria` and `requirements` on every receipt, `limitations` joined by `; ` on
  the limited entry and on the case run (whose entry records that inherited
  environment variables were not recorded), no `limitations` key on an entry
  without one, and the three keys in their place in the field order. It fails on
  main, where a receipt has no `criteria`.
- `tests/migrate_evidence.rs::observations_become_entries_receipts_or_removals_and_every_old_id_resolves`
  allows the three keys on a migrated receipt and expects its `criteria` and
  `requirements` to equal the 0.5.x observation's.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-history-006 | the two tests above and the case in `.chough/cases/history.receipts-keep-links.cases.toml` |
