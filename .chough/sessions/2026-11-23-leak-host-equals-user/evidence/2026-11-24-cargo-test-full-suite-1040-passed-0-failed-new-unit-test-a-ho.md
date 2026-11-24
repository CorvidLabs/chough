---
kind: evidence
id: 2026-11-24-cargo-test-full-suite-1040-passed-0-failed-new-unit-test-a-ho
aliases: []
claim: "cargo test: full suite 1040 passed, 0 failed; new unit test a_host_label_equal_to_the_user_name_is_not_a_leak_as_a_path_component red on main, green with the fix; the built binary's doctor --leaks on the tob repository reports 0 findings (6 host-name findings before the fix)"
result: passed
evidence_kind: product-test
revision: 6e00b813a0d52be20a361b625cb96db03be171d6
content: 806d82a7ba15318126f7fac49c60562c943c05f02f7ecfd61b6ee7aa9bc16f21
environment: macos-arm64
criteria: [LIFE-26]
requirements: [REQ-records-006]
contracts: []
limitations:
  - "recorded on a toolchain that differs from CI: differs: SpecSync 6.4.1 (CI 6.4.0); Fledge 1.8.1 (CI 1.8.0)\n2026-11-24T00:27:07Z\nHI: installed 0.8.0, CI pins 0.8.0\nSpecSync: installed 6.4.1, CI pins 6.4.0\nFledge: installed 1.8.1, CI pins 1.8.0\nRust"
observed: 2026-11-24T00:33:54Z
actor:
  - terminal: t14g
---
**cargo test: full suite 1040 passed, 0 failed; new unit test a_host_label_equal_to_the_user_name_is_not_a_leak_as_a_path_component red on main, green with the fix; the built binary's doctor --leaks on the tob repository reports 0 findings (6 host-name findings before the fix).**

Recorded as passed in macos-arm64 at revision 6e00b813a0d5, observed 2026-11-24T00:33:54Z.
