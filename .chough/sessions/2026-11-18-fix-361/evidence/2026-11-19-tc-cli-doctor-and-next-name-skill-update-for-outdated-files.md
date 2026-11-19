---
kind: evidence
id: 2026-11-19-tc-cli-doctor-and-next-name-skill-update-for-outdated-files
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
duration_seconds: 2
revision: fcd5cebe2edb0d83d8e0d50c7c588e8be4ec2ee4
content: cf5a0802237706027088f0777cefb3b052dadff11286a38866c3543c30ccdc23
environment: local-macos-arm64
fingerprint: sha256:6c73953808866efb6bfe7b38af752304e88fd112bea5e38f8b3216200da86757
criteria: [LIFE-12]
requirements: [REQ-cli-016]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T02:01:32Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran cargo test --locked --test package -- --exact doctor_and_next_say_when_skill_files_are_known_outdated for TC-cli-doctor-and-next-name-skill-update-for-outdated-files: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision fcd5cebe2edb, observed 2026-11-19T02:01:32Z. The raw log stays in the machine-local store.
