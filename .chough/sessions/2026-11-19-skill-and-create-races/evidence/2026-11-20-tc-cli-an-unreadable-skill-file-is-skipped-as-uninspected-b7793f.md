---
kind: evidence
id: 2026-11-20-tc-cli-an-unreadable-skill-file-is-skipped-as-uninspected-b7793f
aliases: []
claim: "Ran cargo test --locked --test cli -- --exact an_unreadable_skill_file_is_skipped_as_uninspected for TC-cli-an-unreadable-skill-file-is-skipped-as-uninspected: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-an-unreadable-skill-file-is-skipped-as-uninspected]
case_digest: [086bcec2745fd2bf5bacaac4374db24720b2860179bb63f9fb10168a7c418e49]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - cli
  - "--"
  - "--exact"
  - an_unreadable_skill_file_is_skipped_as_uninspected
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: f38abb556cee50e587a4c2446a91c7f2fa140997
content: 22f079e0c52edd4b8e261a363b63b124fc70d687f3ee31cf0865838d130d7502
environment: local-macos-arm64
fingerprint: sha256:96cb6beeed59130fc6241076a18542677b436445661a4c0259154c631d64a89c
criteria: [FLOW-1, LIFE-4]
requirements: [REQ-cli-016]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T14:59:55Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test cli -- --exact an_unreadable_skill_file_is_skipped_as_uninspected for TC-cli-an-unreadable-skill-file-is-skipped-as-uninspected: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision f38abb556cee, observed 2026-11-20T14:59:55Z. The raw log stays in the machine-local store.
