---
kind: evidence
id: 2026-11-18-tc-cli-a-busy-evidence-add-leaves-nothing-583391
aliases: []
claim: "Ran cargo test --locked --test evidence -- --exact a_busy_evidence_add_writes_nothing_and_refuses_with_nothing_saved for TC-cli-a-busy-evidence-add-leaves-nothing: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-a-busy-evidence-add-leaves-nothing]
case_digest: [171db9f5113ca1ed54e860a2ae631c98649b907269783242a47990e24116adb9]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - evidence
  - "--"
  - "--exact"
  - a_busy_evidence_add_writes_nothing_and_refuses_with_nothing_saved
exit_code: 0
tests_run: 1
duration_seconds: 30
revision: 6de7742e1c283c58406b202bd5b3294001778083
content: 52a5a34cff8b408de4f6326410ff8dbecc6b96f3264e8be3b086076ef9a66e6b
environment: local-macos-arm64
fingerprint: sha256:9a882a04d39929b6b74753d99337f7dbe97899bc1a9fe52892ebc2eed384b6b2
criteria: [DRIVE-11]
requirements: [REQ-cli-040]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T19:33:47Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test evidence -- --exact a_busy_evidence_add_writes_nothing_and_refuses_with_nothing_saved for TC-cli-a-busy-evidence-add-leaves-nothing: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 6de7742e1c28, observed 2026-11-18T19:33:47Z. The raw log stays in the machine-local store.
