---
kind: evidence
id: 2026-11-18-tc-cli-doctor-and-next-name-skill-update-for-outdated-files
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
duration_seconds: 1
revision: ac1c3dd118bbb4d69e3b132184a2fa844fa2c3e4
content: 7f0227ff76a0faeb9d954aa7517a56f899edd9d0941741e2dd735ba4a9c9e031
environment: local-macos-arm64
fingerprint: sha256:de685549cb873be61398f888c36d53d65f2f1b2bb30e02612d1bd1ea770a2018
criteria: [LIFE-12]
requirements: [REQ-cli-016]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T23:43:11Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran cargo test --locked --test package -- --exact doctor_and_next_say_when_skill_files_are_known_outdated for TC-cli-doctor-and-next-name-skill-update-for-outdated-files: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision ac1c3dd118bb, observed 2026-11-18T23:43:11Z. The raw log stays in the machine-local store.
