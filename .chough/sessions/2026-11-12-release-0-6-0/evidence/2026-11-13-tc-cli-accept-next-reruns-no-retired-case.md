---
kind: evidence
id: 2026-11-13-tc-cli-accept-next-reruns-no-retired-case
aliases: []
claim: "Ran cargo test --locked --test next -- --exact at_accept_next_names_no_rerun_of_a_retired_case for TC-cli-accept-next-reruns-no-retired-case: failed (exit 101, tests run 1)."
result: failed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-accept-next-reruns-no-retired-case]
case_digest: [033ba8d0f10d1b171066bd46228fbb171623e904567f76602dc2be2b6689d3c2]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - next
  - "--"
  - "--exact"
  - at_accept_next_names_no_rerun_of_a_retired_case
exit_code: 101
tests_run: 1
duration_seconds: 4
revision: fc14a104e2c76a2a133cf3e3ef932f6b1f318509
content: f060dec5e9d73dd1f3936ba47c56074be4c863313e27f0d65f4013350c47501e
environment: local-macos-arm64
fingerprint: sha256:d7e14b0cacf7e4a1ba93c114e5dcfc6ad006de6eea2503c741b802dc972d8e34
criteria: [FLOW-13, FLOW-23]
requirements: [REQ-cli-021]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T00:29:04Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test next -- --exact at_accept_next_names_no_rerun_of_a_retired_case for TC-cli-accept-next-reruns-no-retired-case: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision fc14a104e2c7, observed 2026-11-13T00:29:04Z. The raw log stays in the machine-local store.

## Key failing lines

- ---- at_accept_next_names_no_rerun_of_a_retired_case stdout ----
- thread 'at_accept_next_names_no_rerun_of_a_retired_case' (929244000) panicked at tests/next.rs:3023:5:
- a retired case is named for a rerun: Prepare the human's review, in order: rerun each referenced case whose latest run is not current, `chough cases run TC-cli-f4-rerun --environment local`; open the review gate with `chough checkpoint --awaiting-human "Implementation review of f4-retired" --holds accept --drives review`; commit the source with Chough's records; then run `chough change check f4-retired`. The human reviews after that.
- note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 34 filtered out; finished in 1.59s
