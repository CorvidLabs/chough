---
kind: evidence
id: 2026-11-12-tc-cli-accept-reference-lessons-before-case-runs
aliases: []
claim: "Ran cargo test --locked --test docs -- --exact the_accept_reference_writes_lessons_before_the_case_runs for TC-cli-accept-reference-lessons-before-case-runs: failed (exit 101, tests run 1)."
result: failed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-accept-reference-lessons-before-case-runs]
case_digest: [1131dd1ee910b5059c51b566e991ca0c9244d150b51e6bbcc8dfe5f5a2c542d8]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - docs
  - "--"
  - "--exact"
  - the_accept_reference_writes_lessons_before_the_case_runs
exit_code: 101
tests_run: 1
duration_seconds: 1
revision: 0c44e742c25af1a4bd717991b53b49f49342f300
content: 0d739975bf00ae228930e62297f6b0156014c9c12119a6f26cea471c15d2a9a0
environment: local-macos-arm64
fingerprint: sha256:95b513c8b4b6c9f434627d20193b1b4eab449467e56e94faf6578683fa88704d
criteria: [FLOW-18, LIFE-3]
requirements: [REQ-cli-002]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T23:54:19Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test docs -- --exact the_accept_reference_writes_lessons_before_the_case_runs for TC-cli-accept-reference-lessons-before-case-runs: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision 0c44e742c25a, observed 2026-11-12T23:54:19Z. The raw log stays in the machine-local store.

## Key failing lines

- ---- the_accept_reference_writes_lessons_before_the_case_runs stdout ----
- thread 'the_accept_reference_writes_lessons_before_the_case_runs' (928571960) panicked at tests/docs.rs:1895:32:
- skills/chough-workflow/references/accept.md step 2 does not place the lesson: 2. Run the affected cases: `chough cases run <ID> --environment <env>`, plus the
-    project lanes.
- note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 18 filtered out; finished in 0.00s
