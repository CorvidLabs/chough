---
kind: evidence
id: 2026-11-12-tc-records-the-migrate-preview-warns-of-open-changes
aliases: []
claim: "Ran cargo test --locked --test migrate -- --exact the_preview_names_open_native_changes_that_make_apply_refuse for TC-records-the-migrate-preview-warns-of-open-changes: failed (exit 101, tests run 1)."
result: failed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-the-migrate-preview-warns-of-open-changes]
case_digest: [0fd96f550b0e9596f0d30e1a6af6367f05f965adde515adc9600f5fce621d5d8]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - migrate
  - "--"
  - "--exact"
  - the_preview_names_open_native_changes_that_make_apply_refuse
exit_code: 101
tests_run: 1
duration_seconds: 5
revision: 0fa8cb7f4b803b880ae8fdbdfe982d4a4477af32
content: b39f26f9837581f0594555309a6e1a3d8ecfb892dcd6c3735082e1a31419cd3e
environment: local-macos-arm64
fingerprint: sha256:32fa2fa46243e5b7f2a87d346bf2d6cfb694e12c47e1537fd785966377b2c739
criteria: [DRIVE-27, LIFE-20]
requirements: [REQ-records-009]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T15:34:10Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test migrate -- --exact the_preview_names_open_native_changes_that_make_apply_refuse for TC-records-the-migrate-preview-warns-of-open-changes: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision 0fa8cb7f4b80, observed 2026-11-12T15:34:10Z. The raw log stays in the machine-local store.

## Key failing lines

- ---- the_preview_names_open_native_changes_that_make_apply_refuse stdout ----
- thread 'the_preview_names_open_native_changes_that_make_apply_refuse' (917893166) panicked at tests/migrate.rs:1381:5:
- assertion `left == right` failed: {
-   "aliases": {
-     "INB-0590e45a5d1e2245e91e1121": ".chough/sessions/2026-11-12-open-two/agent/inbox/2026-11-11-keep-the-second-run.md",
-     "INB-5daf329377423eec43892694": ".chough/sessions/2026-11-12-open-two/agent/inbox/2026-11-11-use-the-smaller-fixture.md",
-     "adopter": ".chough/archive/sessions/2026-11-11-adopter.md",
-     "archived-1": ".chough/archive/sessions/2026-11-11-archived-1.md",
-     "archived-2": ".chough/archive/sessions/2026-11-11-archived-2.md",
-     "archived-3": ".chough/archive/sessions/2026-11-11-archived-3.md",
-     "archived-4": ".chough/archive/sessions/2026-11-11-archived-4.md",
-     "archived-5": ".chough/archive/sessions/2026-11-11-archived-5.md",
-     "archived-6": ".chough/archive/sessions/2026-11-11-archived-6.md",
-     "finisher": ".chough/archive/sessions/2026-11-11-finisher.md",
-     "open-one": ".chough/sessions/2026-11-11-open-one/session.md",
-     "open-two": ".chough/sessions/2026-11-12-open-two/session.md"
-   },
-   "apply": "chough migrate --apply 03fefd151239829524473efee3410d03fc353775c4e12ed20926b4a3c601cca4 --consent \"<who consented, their words and where>\"",
-   "becomes": {
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 18 filtered out; finished in 0.75s
