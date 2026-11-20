---
kind: evidence
id: 2026-11-20-tc-cli-g2-a-session-before-the-first-commit-1d6151
aliases: []
claim: "Ran cargo test --locked --test gates -- --exact a_session_started_before_the_first_commit_counts_from_it for TC-cli-g2-a-session-before-the-first-commit: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-g2-a-session-before-the-first-commit]
case_digest: [6aac022451b58ebc4595b293330f52a91eeb8d0eefdcccbe621685e39891842d]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - gates
  - "--"
  - "--exact"
  - a_session_started_before_the_first_commit_counts_from_it
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 35859e1516e1fa57a436c12188c3b3aa7263b420
content: db21fe4862a1a7deb857a290ea4afff285aad1eecddf04ee171bcd6e10f6fcdb
environment: local-macos-arm64
fingerprint: sha256:27bedee362587b1a42d5f000a6690b800ffd1ee18433f09f6126da3dcb4d48ee
criteria: [DRIVE-24]
requirements: [REQ-cli-061]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T18:43:46Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t85
---
**Ran cargo test --locked --test gates -- --exact a_session_started_before_the_first_commit_counts_from_it for TC-cli-g2-a-session-before-the-first-commit: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 35859e1516e1, observed 2026-11-20T18:43:46Z. The raw log stays in the machine-local store.
