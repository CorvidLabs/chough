---
change: support-macos-and-linux-with-windows-through-wsl-and-drop-the-native-windows-ci-job-flow-17
artifact: plan
---

# Plan

## Approach

Remove `windows-latest` from the product matrix and say, everywhere the repository states platforms, that Chough supports macOS and Linux and Windows through WSL 2 with Ubuntu. Say precisely what the Ubuntu jobs cover for WSL. Add one package test, failing first, that keeps the CI matrix, the Linux exercise and these statements in agreement.

## Out of scope

- Removing the `cfg(windows)` code in `src/` and `tests/`. It stays as it is, untested, and the docs say so.
- A CI job inside WSL on a Windows runner (for example with setup-wsl). It would bring back a Windows runner that pull requests wait on.
- Any requirement text, including REQ-cli-046, and any Linux or macOS behavior.
- Closing issues, and changing branch protection or repository settings.
- Raven Desktop, which keeps Windows.

## Steps

1. Add the failing test `supported_platforms_are_macos_and_linux_with_windows_through_wsl` in tests/package.rs and see it fail on the base.
2. Drop `windows-latest` from the product matrix in `.github/workflows/ci.yml`.
3. Update README (Tested platforms, Known limitations), docs/ci.md (the job summary and the required-check list), docs/integrations.md (a Platforms section), docs/cli.md (the `taskkill` lines say native Windows is unsupported and untested), docs/ROADMAP.md (APS no longer needs Windows builds), docs/DECISIONS.md (D-022) and CHANGELOG.md.
4. Update specs/cli/testing.md so it no longer claims Windows CI.
5. Add case `TC-cli-supported-platforms` in `.chough/cases/cli.supported-platforms.cases.toml`, linked to FLOW-17, and run it.
6. Run `fledge lanes run verify` locally.

## Risks

- Windows-only code can now break unnoticed. That is accepted: native Windows is unsupported.
- WSL-specific setups (WSL 1, a checkout on `/mnt/c`, Git for Windows) are not exercised. The docs name them as not covered.
- The package test reads YAML and Markdown as text, so a reworded doc can fail it. Its messages name the file and the missing statement.

## Constraints consulted

FLOW-17; the 2026-11-05 lesson "generated paths must check out on Windows" (the 220-character rule stays); the 2026-11-04 lesson on CRLF checkouts (the LF-normalized digests and CRLF tests stay); AGENTS.md (no CI result is claimed before it is observed); the CI pins kept by `tool_and_rust_version_pins_agree`.
