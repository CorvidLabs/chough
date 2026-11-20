# Receipts written by released builds

`chough migrate` reads every receipt of an earlier migration to judge 0.5.x files that
come back (REQ-records-009, #370), so the receipts released builds wrote must read by
that rule. The test `receipts_from_released_builds_list_the_files_read` in
`src/migrate.rs` reads:

- Chough's own receipt, `.chough/archive/migrations/2026-11-12-from-0-5.md`, which
  the 0.6.0 build wrote when this repository migrated (#239);
- `from-0-6-1.md`, which the v0.6.1 build (tag `v0.6.1`, cef86e26) wrote on a
  synthetic project: a copy of `tests/fixtures/migrate/adopted`, committed, migrated
  with `chough migrate --apply <digest> --consent "SYNTHETIC-human said migrate it
  (fixture)"` under a scratch `HOME` and `USER=fixture`.

To regenerate `from-0-6-1.md`, build `v0.6.1` from its tag in a worktree, repeat those
steps in a scratch folder with that build, and copy the receipt here. No other
project's receipt belongs here: a receipt lists that project's paths and IDs.
