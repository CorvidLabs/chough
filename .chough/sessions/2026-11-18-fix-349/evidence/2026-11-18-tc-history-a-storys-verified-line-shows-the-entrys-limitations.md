---
kind: evidence
id: 2026-11-18-tc-history-a-storys-verified-line-shows-the-entrys-limitations
aliases: []
claim: "Ran cargo test --locked --test evidence -- --exact a_story_receipt_carries_each_field_its_entry_knows for TC-history-a-storys-verified-line-shows-the-entrys-limitations: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-history-a-storys-verified-line-shows-the-entrys-limitations]
case_digest: [18a398b09c6239b8e00a8f462c48175d172acebc4dd3d749a5d29f92d4fdcfc9]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - evidence
  - "--"
  - "--exact"
  - a_story_receipt_carries_each_field_its_entry_knows
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 3fcecacf966fe8577a81df66fa6534ffa574c4ca
content: 4bee6d8ed8896c676c800a45b378fc42ea2c071d7e6d298166b91731d59f24e0
environment: local-macos-arm64
fingerprint: sha256:0892524d94ed9cc9aa3f966d3be835b4203c40713bab703f3d6db4fb296291c8
criteria: [KNOW-35]
requirements: [REQ-history-006]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T22:51:26Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran cargo test --locked --test evidence -- --exact a_story_receipt_carries_each_field_its_entry_knows for TC-history-a-storys-verified-line-shows-the-entrys-limitations: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 3fcecacf966f, observed 2026-11-18T22:51:26Z. The raw log stays in the machine-local store.
