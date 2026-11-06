---
change: support-macos-and-linux-with-windows-through-wsl-and-drop-the-native-windows-ci-job-flow-17
artifact: testing
---

# Testing

## tests/package.rs

`supported_platforms_are_macos_and_linux_with_windows_through_wsl` checks the CI matrix, that the Ubuntu `native-gates` job runs the verify lane with the exercise, and the platform statements in README.md, docs/ci.md and docs/integrations.md (see design).

It fails first: the base matrix includes `windows-latest`, and README lists "Hosted CI: Ubuntu, macOS, Windows".

## CRLF safety

The tests that normalize CRLF or write CRLF copies stay unchanged and run on Linux and macOS. They are the CRLF `change.md` freshness test (TC-cli-0070), `library index --check` on a CRLF copy (REQ-cli-047), and the LF-normalized digest checks in tests/package.rs, tests/optional.rs, tests/stages.rs and tests/vault.rs.

## Case

`TC-cli-supported-platforms` (FLOW-17) binds the new test with the `cargo-test` adapter and is run with `chough cases run`.

## Lanes

`fledge lanes run verify` passes locally. Hosted CI is observed at the archived tip before merge, and no result is claimed before it is observed.
