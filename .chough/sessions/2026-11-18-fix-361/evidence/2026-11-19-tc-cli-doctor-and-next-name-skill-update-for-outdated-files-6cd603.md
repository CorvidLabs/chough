---
kind: evidence
id: 2026-11-19-tc-cli-doctor-and-next-name-skill-update-for-outdated-files-6cd603
aliases: []
claim: "Ran cargo test --locked --test package -- --exact doctor_and_next_say_when_skill_files_are_known_outdated for TC-cli-doctor-and-next-name-skill-update-for-outdated-files: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-doctor-and-next-name-skill-update-for-outdated-files]
case_digest: [03bea407285321899ad293896668fd3e25bbae52519388dea6c38ca520147dd7]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - package
  - "--"
  - "--exact"
  - doctor_and_next_say_when_skill_files_are_known_outdated
exit_code: 0
tests_run: 1
duration_seconds: 9
revision: c5c59657462d947bc8d3efba8b7cffcc48646071
content: fc3e96d5c75a330a1fda8f6ae611bff30cef3d363563886e00612efbb5b8b002
environment: local-macos-arm64
fingerprint: sha256:43001eb6d3f2167f6c33b6aa608b1b45d544603f2beba6edd1efa09d86f0beb0
criteria: [LIFE-12]
requirements: [REQ-cli-016]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T02:15:15Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran cargo test --locked --test package -- --exact doctor_and_next_say_when_skill_files_are_known_outdated for TC-cli-doctor-and-next-name-skill-update-for-outdated-files: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision c5c59657462d, observed 2026-11-19T02:15:15Z. The raw log stays in the machine-local store.
