---
kind: evidence
id: 2026-11-20-tc-cli-g2-a-note-and-a-reason-both-reach-the-timeline
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
revision: b0006dd3acf176c6e05b870c0213084d93116a71
content: f8d2d03c6db5bb7203d64fcc49105b7ed4cb88919b89d4f43253279be8c49fa7
environment: local-macos-arm64
fingerprint: sha256:6db6a50ec044a267f5a488499b760123a1732a2b2642355f6b394e93b20bfbe2
criteria: [DRIVE-3]
requirements: [REQ-cli-031]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T19:57:11Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t85
---
**Ran cargo test --locked --test stages -- --exact a_note_and_a_reason_both_reach_the_timeline for TC-cli-g2-a-note-and-a-reason-both-reach-the-timeline: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision b0006dd3acf1, observed 2026-11-20T19:57:11Z. The raw log stays in the machine-local store.
