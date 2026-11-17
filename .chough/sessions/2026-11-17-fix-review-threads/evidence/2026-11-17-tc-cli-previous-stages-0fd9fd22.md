---
kind: evidence
id: 2026-11-17-tc-cli-previous-stages-0fd9fd22
aliases: []
claim: "Ran cargo test --locked --test docs -- --exact stages_from_main_0fd9fd22_update_and_an_edited_copy_stays for TC-cli-previous-stages-0fd9fd22: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-previous-stages-0fd9fd22]
case_digest: [ae1659a4572ce084ac087c454e145b882b1429ee03984a4e97b8b20758a8ec7f]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - docs
  - "--"
  - "--exact"
  - stages_from_main_0fd9fd22_update_and_an_edited_copy_stays
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: ab0d421149f12d9ad2984c833cfe1d9346d3989e
content: 2f9cb23952ea3e465dab57e7278d76cc9821b4c15dea27dc3c87620f01f0d0f7
environment: local-macos-arm64
fingerprint: sha256:782fbdc14a07051b1630dba393e8d542aaa750056e45026fef54acf7c9c8a253
criteria: [FLOW-18]
requirements: [REQ-cli-016]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-17T14:48:10Z
actor:
  - agent: grok-t1w
    model: grok
    terminal: t1w
---
**Ran cargo test --locked --test docs -- --exact stages_from_main_0fd9fd22_update_and_an_edited_copy_stays for TC-cli-previous-stages-0fd9fd22: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision ab0d421149f1, observed 2026-11-17T14:48:10Z. The raw log stays in the machine-local store.
