---
kind: evidence
id: 2026-11-12-tc-cli-gate-summary-counts-from-the-changes-own-base
aliases: []
claim: "Ran cargo test --locked --test gates -- --exact a_second_change_on_one_branch_counts_from_its_own_base_commit for TC-cli-gate-summary-counts-from-the-changes-own-base: failed (exit 101, tests run 1)."
result: failed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-gate-summary-counts-from-the-changes-own-base]
case_digest: [96733866e58287900d082a4267d823512be8f83e3aad9c16f5f3885888e91ad8]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - gates
  - "--"
  - "--exact"
  - a_second_change_on_one_branch_counts_from_its_own_base_commit
exit_code: 101
tests_run: 1
duration_seconds: 2
revision: acd2b0b0a7a1c7abb78ff8af1e260c4f8aaaa412
content: 2d6250eed8dd794966658bf74e5293ab41d4660a58c5226068705266141c63b6
environment: local-macos-arm64
fingerprint: sha256:d07fed67730e6e225b2fde27d630dbf619dbb45503aef353074e796e6e4b33ce
criteria: [FLOW-16, FLOW-23]
requirements: [REQ-cli-061]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T15:27:42Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test gates -- --exact a_second_change_on_one_branch_counts_from_its_own_base_commit for TC-cli-gate-summary-counts-from-the-changes-own-base: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision acd2b0b0a7a1, observed 2026-11-12T15:27:42Z. The raw log stays in the machine-local store.

## Key failing lines

- ---- a_second_change_on_one_branch_counts_from_its_own_base_commit stdout ----
- thread 'a_second_change_on_one_branch_counts_from_its_own_base_commit' (917804066) panicked at tests/gates.rs:543:5:
- Gate: Scope approval of g1
- Change: g1 — Free shipping at the threshold (approved)
- What changed: 4 files, +4 −0 since 96ec3069
-   lib/first-change-0.txt +1 −0
-   lib/first-change-1.txt +1 −0
-   lib/first-change-2.txt +1 −0
-   src/second.txt +1 −0
- Tests: 1 passed, 1 failed, 0 not run
-   Failing: TC-cli-g1-fail
- Risks:
-   - A rounding edge at exactly fifty dollars.
- Diff: git diff 96ec3069a65c1bb76f81d9206ede313936090bd4..11da2273d19260321be580068cbeeb84ff68afc6
- note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 8 filtered out; finished in 1.75s
