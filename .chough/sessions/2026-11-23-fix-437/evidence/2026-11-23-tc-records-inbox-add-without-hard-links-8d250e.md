---
kind: evidence
id: 2026-11-23-tc-records-inbox-add-without-hard-links-8d250e
aliases: []
claim: "Ran cargo test --locked --test no_hard_links -- --ignored --exact new_records_are_written_on_a_volume_without_hard_links for TC-records-inbox-add-without-hard-links: passed (exit 0)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-inbox-add-without-hard-links]
case_digest: [49447b4ab0d79717def9eb696041b65756061b03682e880848b23e1bdf42b43b]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - no_hard_links
  - "--"
  - "--ignored"
  - "--exact"
  - new_records_are_written_on_a_volume_without_hard_links
exit_code: 0
duration_seconds: 4
revision: 35828a1df3d308677a38b138e7aebe0e7b6770ef
content: 4d47b7100bc892032ff8590379612f01dcdfc1da77293fdfae595482355808d1
environment: local-macos-arm64
fingerprint: sha256:d1e520c0afb96a3e2761e1b002099ea7217416fba3b39ded4bfb58cac1ee9d99
criteria: [FLOW-8]
requirements: [REQ-records-001]
contracts: [specs/records/records.spec.md]
limitations:
  - outcome rests on the exit code and the declared output fragments; the command's own assertions are not inspected
  - inherited environment variables were not recorded
observed: 2026-11-23T14:55:31Z
actor:
  - terminal: t14z
---
**Ran cargo test --locked --test no_hard_links -- --ignored --exact new_records_are_written_on_a_volume_without_hard_links for TC-records-inbox-add-without-hard-links: passed (exit 0).**

Recorded as passed in local-macos-arm64 at revision 35828a1df3d3, observed 2026-11-23T14:55:31Z. The raw log stays in the machine-local store.
