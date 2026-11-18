---
kind: evidence
id: 2026-11-18-tc-records-sk-after-a-hyphen-is-not-a-key
aliases: []
claim: "Ran cargo test --locked --test leaks -- --exact a_slug_holding_sk_is_not_a_key_and_a_glued_token_still_is for TC-records-sk-after-a-hyphen-is-not-a-key: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-sk-after-a-hyphen-is-not-a-key]
case_digest: [62ead6f185e47a2fdfc424e109756873ab870e33c684b26fda95ff282a72e04c]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - leaks
  - "--"
  - "--exact"
  - a_slug_holding_sk_is_not_a_key_and_a_glued_token_still_is
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: 66543c2bf509627ec2a165414acae0bc8ab59c4a
content: 0f5bd98c1130324705d5103058dd906f17b7e47ec174e9301347f54bd0dc6577
environment: local-macos-arm64
fingerprint: sha256:4457cc377078dbd3500c8c2ba92caffcc6c1984e91809569dc7d5a43abbfb1da
criteria: [LIFE-26]
requirements: [REQ-records-006]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T15:48:07Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4g
---
**Ran cargo test --locked --test leaks -- --exact a_slug_holding_sk_is_not_a_key_and_a_glued_token_still_is for TC-records-sk-after-a-hyphen-is-not-a-key: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 66543c2bf509, observed 2026-11-18T15:48:07Z. The raw log stays in the machine-local store.
