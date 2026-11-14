---
kind: evidence
id: 2026-11-14-tc-records-an-scp-style-host-path-is-checked-like-the-path-it
aliases: []
claim: "Ran cargo test --locked --bin chough -- --exact leaks::tests::an_scp_style_remote_path_is_checked_like_the_path_it_carries for TC-records-an-scp-style-host-path-is-checked-like-the-path-it-carries: passed (exit 0)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-an-scp-style-host-path-is-checked-like-the-path-it-carries]
case_digest: [46ef70d858aab44624ae827b572a9c0f3e08ee2939b7dfbf7edb819a5195eba7]
command:
  - cargo
  - test
  - "--locked"
  - "--bin"
  - chough
  - "--"
  - "--exact"
  - leaks::tests::an_scp_style_remote_path_is_checked_like_the_path_it_carries
exit_code: 0
duration_seconds: 0
revision: 2b617926f0f0d16dab31e83623bba0ad526a4773
content: 3abfa6fa0222e7654ea3c25247b83fe6b460cd02631bba63670194b705ef85d6
environment: local-macos-arm64
fingerprint: sha256:25c01a074a444969a0082e02b4090a8d04604af7debfd8e9feb585b595013bab
criteria: [LIFE-26, LIFE-17, FLOW-22]
requirements: [REQ-records-006]
contracts: [specs/records/records.spec.md]
limitations:
  - outcome rests on the exit code and the declared output fragments; the command's own assertions are not inspected
  - inherited environment variables were not recorded
observed: 2026-11-14T17:36:32Z
actor:
  - agent: claude-code
---
**Ran cargo test --locked --bin chough -- --exact leaks::tests::an_scp_style_remote_path_is_checked_like_the_path_it_carries for TC-records-an-scp-style-host-path-is-checked-like-the-path-it-carries: passed (exit 0).**

Recorded as passed in local-macos-arm64 at revision 2b617926f0f0, observed 2026-11-14T17:36:32Z. The raw log stays in the machine-local store.
