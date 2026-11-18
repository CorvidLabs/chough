---
kind: evidence
id: 2026-11-18-tc-history-a-storys-verified-line-shows-the-entrys-limitations-903080
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
duration_seconds: 1
revision: e86d71c89caaa6b861c00081a9ba7fd57015fd84
content: 7e8250c431538fb9a472e1cafbeebe77b94c0ae6a852af6aff026e6737555100
environment: local-macos-arm64
fingerprint: sha256:0a2cb8dba64b74258da60751f7fe20e2053ed53ac82257d9d571bee1704f148c
criteria: [KNOW-35]
requirements: [REQ-history-006]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T23:08:18Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran cargo test --locked --test evidence -- --exact a_story_receipt_carries_each_field_its_entry_knows for TC-history-a-storys-verified-line-shows-the-entrys-limitations: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision e86d71c89caa, observed 2026-11-18T23:08:18Z. The raw log stays in the machine-local store.
