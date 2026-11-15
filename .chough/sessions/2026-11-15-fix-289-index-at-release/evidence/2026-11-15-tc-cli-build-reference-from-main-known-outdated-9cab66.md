---
kind: evidence
id: 2026-11-15-tc-cli-build-reference-from-main-known-outdated-9cab66
aliases: []
claim: "Ran cargo test --locked --test cli -- --exact build_reference_from_main_4daf861_is_known_outdated for TC-cli-build-reference-from-main-known-outdated: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-build-reference-from-main-known-outdated]
case_digest: [e87e897bdf1ab06ffab9894326cd28ecb9b9faed7c463e3dc6c369087343f52f]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - cli
  - "--"
  - "--exact"
  - build_reference_from_main_4daf861_is_known_outdated
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 8b5bd53cab53ec98725db6feb53ee0f6570e3451
content: 12c5104737c421d6e72a751786ffabf99172508dabf9ee642dda94f38685a761
environment: local-macos-arm64
fingerprint: sha256:6968532466df4484bc80cb442dd592b8b1f8756e03674c0064e8464916cde0ec
criteria: [LIFE-11, LIFE-4]
requirements: [REQ-cli-034]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-15T21:09:10Z
actor:
  - agent: claude-code
    terminal: t28
---
**Ran cargo test --locked --test cli -- --exact build_reference_from_main_4daf861_is_known_outdated for TC-cli-build-reference-from-main-known-outdated: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 8b5bd53cab53, observed 2026-11-15T21:09:10Z. The raw log stays in the machine-local store.
