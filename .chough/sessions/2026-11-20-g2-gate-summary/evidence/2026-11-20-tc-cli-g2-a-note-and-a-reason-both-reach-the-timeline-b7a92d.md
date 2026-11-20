---
kind: evidence
id: 2026-11-20-tc-cli-g2-a-note-and-a-reason-both-reach-the-timeline-b7a92d
aliases: []
claim: "Ran cargo test --locked --test stages -- --exact a_note_and_a_reason_both_reach_the_timeline for TC-cli-g2-a-note-and-a-reason-both-reach-the-timeline: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-g2-a-note-and-a-reason-both-reach-the-timeline]
case_digest: [9ddeedc00bc2af4e31acf464ca345e745a6eb90be21189316fdfe1e3228ab0b5]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - stages
  - "--"
  - "--exact"
  - a_note_and_a_reason_both_reach_the_timeline
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 01593390f15cb0ca05623b83d99173b1d09e9dba
content: 135121ca601cf4d173f91cd8362805a9b0f4c7a2c2fd10e09b1be2c086170999
environment: local-macos-arm64
fingerprint: sha256:fa7675a941a855ad50481a7bb46a9ace57e4cb472b94942e044652277a2db6f3
criteria: [DRIVE-3]
requirements: [REQ-cli-031]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T21:09:52Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t85
---
**Ran cargo test --locked --test stages -- --exact a_note_and_a_reason_both_reach_the_timeline for TC-cli-g2-a-note-and-a-reason-both-reach-the-timeline: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 01593390f15c, observed 2026-11-20T21:09:52Z. The raw log stays in the machine-local store.
