---
kind: evidence
id: 2026-11-23-tc-records-inbox-add-without-hard-links
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
duration_seconds: 3
revision: 91a38bf5fe28054bb1d65cf15634816fe14c50bd
content: d1c29382f51796055bf349b68788331ebec19dd9a633cb69a9499038c7db8433
environment: local-macos-arm64
fingerprint: sha256:368460f1101ff8cd6b0de832e1d4ba0ec44da0cab8ca3566bfb9feea2bd80fca
criteria: [FLOW-8]
requirements: [REQ-records-001]
contracts: [specs/records/records.spec.md]
limitations:
  - outcome rests on the exit code and the declared output fragments; the command's own assertions are not inspected
  - inherited environment variables were not recorded
observed: 2026-11-23T14:42:09Z
actor:
  - terminal: t14z
---
**Ran cargo test --locked --test no_hard_links -- --ignored --exact new_records_are_written_on_a_volume_without_hard_links for TC-records-inbox-add-without-hard-links: passed (exit 0).**

Recorded as passed in local-macos-arm64 at revision 91a38bf5fe28, observed 2026-11-23T14:42:09Z. The raw log stays in the machine-local store.
