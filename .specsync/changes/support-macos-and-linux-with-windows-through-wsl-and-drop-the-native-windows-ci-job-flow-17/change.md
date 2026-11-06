---
id: support-macos-and-linux-with-windows-through-wsl-and-drop-the-native-windows-ci-job-flow-17
state: implementing
type: operations
base_commit: ec830cfb001cc5f71f4f83809ca3294d685f42a4
---

# Support macOS and Linux with Windows through WSL and drop the native Windows CI job (FLOW-17)

## Intent

Support macOS and Linux with Windows through WSL and drop the native Windows CI job (FLOW-17)

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- Hosted CI runs the product tests and release build on ubuntu-latest and macos-latest only, and no job runs on Windows; the MSRV, Fledge / HI / SpecSync (the verify lane with the feature-and-bug exercise), Archive, SpecSync Archive and CI gate fixtures jobs stay on Ubuntu, unchanged. README, docs/ci.md and docs/integrations.md say Chough supports macOS and Linux and Windows only through WSL 2 with Ubuntu, and say exactly what the Ubuntu jobs cover for WSL and what they do not. No requirement or criterion promises native Windows behavior, no Linux or macOS requirement changes (REQ-cli-046 keeps its short attachment paths and file modes), and specs/cli/testing.md no longer claims Windows CI. tests/package.rs::supported_platforms_are_macos_and_linux_with_windows_through_wsl fails on the base and passes after; the CRLF-normalizing tests stay; fledge lanes run verify passes on Linux or macOS.

## No-spec Rationale

No requirement text changes: no REQ promises that Chough runs on native Windows. The CI matrix, the platform statements in README and docs, the cli testing companion's Windows CI lines and a package test that keeps them in agreement change; REQ-cli-046's short attachment paths stay because they keep a repository's tracked tree checkable out by any Git client.
