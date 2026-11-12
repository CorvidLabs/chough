---
kind: evidence
id: 2026-11-12-tc-records-migrate-preview-is-text-with-json-behind-a-flag
aliases: []
claim: "Ran cargo test --locked --test migrate -- --exact the_preview_prints_the_plan_as_text_and_the_same_plan_as_json for TC-records-migrate-preview-is-text-with-json-behind-a-flag: failed (exit 101, tests run 1)."
result: failed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-migrate-preview-is-text-with-json-behind-a-flag]
case_digest: [a94aad04371868914b182bd9fcaac5a8a6b8cc85a1683bbc206be23e8eacbd7b]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - migrate
  - "--"
  - "--exact"
  - the_preview_prints_the_plan_as_text_and_the_same_plan_as_json
exit_code: 101
tests_run: 1
duration_seconds: 1
revision: dc25d77d349b766bd5a32245bb56fce84091b2ff
content: a75b0ad2ae67d0b316dbe9153b8186da895f7ff9092710483969141efd075684
environment: local-macos-arm64
fingerprint: sha256:3273e879b5d4f71dc7f3e4767339023feea69474ceb0911ecc10032a755ffce3
criteria: [DRIVE-27, LIFE-20]
requirements: [REQ-records-009]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T22:41:22Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test migrate -- --exact the_preview_prints_the_plan_as_text_and_the_same_plan_as_json for TC-records-migrate-preview-is-text-with-json-behind-a-flag: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision dc25d77d349b, observed 2026-11-12T22:41:22Z. The raw log stays in the machine-local store.

## Key failing lines

- ---- the_preview_prints_the_plan_as_text_and_the_same_plan_as_json stdout ----
- thread 'the_preview_prints_the_plan_as_text_and_the_same_plan_as_json' (926738449) panicked at tests/migrate.rs:1572:5:
- the preview is JSON:
- {
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
-   "apply": "chough migrate --apply 8aff678a3a718c4d99435a02adc3e7886ca9a48ea5ce0667ee245f224c51f2a7 --consent \"<who consented, their words and where>\"",
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 24 filtered out; finished in 0.80s
