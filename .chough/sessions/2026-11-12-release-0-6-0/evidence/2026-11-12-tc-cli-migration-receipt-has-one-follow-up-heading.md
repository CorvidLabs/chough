---
kind: evidence
id: 2026-11-12-tc-cli-migration-receipt-has-one-follow-up-heading
aliases: []
claim: "Ran cargo test --locked --test migrate -- --exact a_receipt_has_one_follow_up_heading_for_every_follow_up for TC-cli-migration-receipt-has-one-follow-up-heading: failed (exit 101, tests run 1)."
result: failed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-migration-receipt-has-one-follow-up-heading]
case_digest: [c0b53bced8467534fb0a3f914e0d32e2705e82699ccb81d1d0f19fbdccf76697]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - migrate
  - "--"
  - "--exact"
  - a_receipt_has_one_follow_up_heading_for_every_follow_up
exit_code: 101
tests_run: 1
duration_seconds: 1
revision: a0152789eec8d548d98dbd11f86b0818ad98f494
content: 0ef70fa8fed5b907a0059b2d0f7320b32f187bc45fae18ee147c4deb03536246
environment: local-macos-arm64
fingerprint: sha256:1a74a98964fd1784b6efff30841af766b9ab84a15f4fa253ca6a7522efa056d0
criteria: [DRIVE-27, LIFE-20]
requirements: [REQ-records-009]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T20:25:29Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test migrate -- --exact a_receipt_has_one_follow_up_heading_for_every_follow_up for TC-cli-migration-receipt-has-one-follow-up-heading: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision a0152789eec8, observed 2026-11-12T20:25:29Z. The raw log stays in the machine-local store.

## Key failing lines

- ---- a_receipt_has_one_follow_up_heading_for_every_follow_up stdout ----
- thread 'a_receipt_has_one_follow_up_heading_for_every_follow_up' (922839360) panicked at tests/migrate.rs:1550:5:
- assertion `left == right` failed: ---
- kind: receipt
- id: 2026-11-12-from-0-5
- plan_digest: be39a2f6d71286093636b9aa77e00d5ab467e2c3097fb596f825b8dc9eb6909c
- applied: 2026-11-12T20:25:27Z
- consent: "--consent (a relayed claim): SYNTHETIC-human said migrate it (fixture)"
- removed_at_commit: 77d02b4a46ca45644a3d816fb1fa9808030bf24c
- ---
- # Migration from Chough 0.5.x
- Git history still holds what earlier commits recorded; this migration does not rewrite it.
- ## Before
- - `.chough/history/cycles/CYC-35b47ded993bd490850611ba/EVT-4edf61033f58689200e93314.json` f1bc75e5e8d6ada4f9d9e100c15e419fbdef51809bb5ab25ac1a0fb7f5ceee1c
- - `.chough/history/cycles/CYC-42680cc9b00fa16e09f7b1a5/EVT-0cdfd4d090ee4a3389ad309f.json` acf090d9b33ce7632d06143d384555994b1cd442ed07273403aa8ae4a2d0aa85
- - `.chough/history/cycles/CYC-42680cc9b00fa16e09f7b1a5/EVT-da5fff4240599edcb3d6502e.json` e6545f4ab98ca946dd0741a619cc80330a2ccc38f9988d0c57d0e8ed373b1052
- - `.chough/history/cycles/CYC-5a72dc37cbbdb146702c0e2a/EVT-ce87b276f18c71b7feab3cfb.json` eaace03f5d8643d87968b94e629e85244c44b917a10cb91b246b0c62ef54e406
- - `.chough/history/cycles/CYC-621399035e1dfe1261b5ce4a/EVT-188a807240ab4176f0ec460e.json` f7cab9d85cc08f05a7f23dd6b9a0ad88e43d1ca5499afd46172150608bce63cd
- - `.chough/history/cycles/CYC-621399035e1dfe1261b5ce4a/EVT-281a5600cf62a0bc2dc15ca6.json` 199eacf317750c6e9e45dd88469d64f72f0aa83d4242dc2d24cfc4d7a488fc63
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 23 filtered out; finished in 1.65s
