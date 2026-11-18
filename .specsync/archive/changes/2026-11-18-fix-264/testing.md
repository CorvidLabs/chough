---
change: fix-264
artifact: testing
---

# Testing

Two tests in `tests/migrate.rs`, both failing on main:

- `a_second_same_day_migration_names_its_own_receipt`: after a migration, a
  stray 0.5.x session record is committed. The preview, as JSON and as text,
  names `<date>-from-0-5-2.md`; the apply writes it with the plan's digest,
  leaves the first receipt byte for byte, removes the stray record, and a third
  run has nothing to do.
- `a_receipt_name_taken_after_the_preview_refuses_before_any_write`: a receipt
  for another plan takes the previewed name after the preview. The apply
  refuses with the tree byte for byte unchanged, and a fresh preview names the
  next receipt and applies.

The existing interrupted-apply test still passes: a receipt this plan's own
apply wrote is used again under the same name and digest.

`.chough/cases/records.migrate-same-day-receipt.cases.toml` binds both tests.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-records-009 | the two tests above and their cases |
