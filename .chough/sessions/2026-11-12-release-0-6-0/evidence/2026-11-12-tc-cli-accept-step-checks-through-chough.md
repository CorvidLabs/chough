---
kind: evidence
id: 2026-11-12-tc-cli-accept-step-checks-through-chough
aliases: []
claim: "Ran cargo test --locked --test docs -- --exact the_accept_steps_first_check_runs_through_chough for TC-cli-accept-step-checks-through-chough: failed (exit 101, tests run 1)."
result: failed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-accept-step-checks-through-chough]
case_digest: [0df2a911862ae9e79414998b5ce331a5b836a2f1645ba062ce528791573a56af]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - docs
  - "--"
  - "--exact"
  - the_accept_steps_first_check_runs_through_chough
exit_code: 101
tests_run: 1
duration_seconds: 1
revision: 804bdb506371f0f50b97e76f0c09cc33daf439af
content: 431b305478c9131e54c07bc6bd895e4d5380b9af25bb28dad9e6056aed6a4f09
environment: local-macos-arm64
fingerprint: sha256:553b9d1523e88deb37a51f0786588787d0f7b5b9c56fb6e2e7998cd7badb3323
criteria: [FLOW-18, DRIVE-20]
requirements: [REQ-cli-057]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T21:05:45Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test docs -- --exact the_accept_steps_first_check_runs_through_chough for TC-cli-accept-step-checks-through-chough: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision 804bdb506371, observed 2026-11-12T21:05:45Z. The raw log stays in the machine-local store.

## Key failing lines

- ---- the_accept_steps_first_check_runs_through_chough stdout ----
- thread 'the_accept_steps_first_check_runs_through_chough' (924234752) panicked at tests/docs.rs:1793:9:
- skills/chough-workflow/references/accept.md step 1 does not run the check through Chough: 1. When the delta adds or modifies requirements, run `specsync change check <id>`
-    first, so the requirement text that cases link is materialized (cases already
-    link it from the approved delta while the checkpoint records the change). It
-    refuses while `tasks.md` has an unchecked box, so the cases in step 2 are never a
-    box there.
- note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 16 filtered out; finished in 0.00s
