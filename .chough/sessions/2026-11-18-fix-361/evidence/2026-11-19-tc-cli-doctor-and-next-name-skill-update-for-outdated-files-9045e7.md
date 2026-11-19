---
kind: evidence
id: 2026-11-19-tc-cli-doctor-and-next-name-skill-update-for-outdated-files-9045e7
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
revision: e8bd047b79fed58402ca1bb47198e0ba7cddaa45
content: bf9424fbe897384f08798ea4a75962fdb3aa145cf873253ceb049a243db9bb77
environment: local-macos-arm64
fingerprint: sha256:4d06019fe0091bbd9eb2ffb01c843cb9a86d7b899c1ec5136d9fe4a51bbb03df
criteria: [LIFE-12]
requirements: [REQ-cli-016]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T02:33:24Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran cargo test --locked --test package -- --exact doctor_and_next_say_when_skill_files_are_known_outdated for TC-cli-doctor-and-next-name-skill-update-for-outdated-files: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision e8bd047b79fe, observed 2026-11-19T02:33:24Z. The raw log stays in the machine-local store.
