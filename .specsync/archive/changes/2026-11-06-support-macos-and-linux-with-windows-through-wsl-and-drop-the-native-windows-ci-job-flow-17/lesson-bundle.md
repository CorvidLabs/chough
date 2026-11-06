# Lesson bundle: support-macos-and-linux-with-windows-through-wsl-and-drop-the-native-windows-ci-job-flow-17

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Support macOS and Linux with Windows through WSL and drop the native Windows CI job (FLOW-17)
- **Kind**: Operations
- **Specs**: cli
- **Paths**: .github/workflows/ci.yml, README.md, CHANGELOG.md, docs/ci.md, docs/cli.md, docs/integrations.md, docs/DECISIONS.md, docs/ROADMAP.md, docs/FRD.md, specs/cli/testing.md, tests/package.rs, hi/flow.md, INTENT.md, knowledge/
- **Acceptance**: Hosted CI runs the product tests and release build on ubuntu-latest and macos-latest only, and no job runs on Windows; the MSRV, Fledge / HI / SpecSync (the verify lane with the feature-and-bug exercise), Archive, SpecSync Archive and CI gate fixtures jobs stay on Ubuntu, unchanged. README, docs/ci.md and docs/integrations.md say Chough supports macOS and Linux and Windows only through WSL 2 with Ubuntu, and say exactly what the Ubuntu jobs cover for WSL and what they do not. No requirement or criterion promises native Windows behavior, no Linux or macOS requirement changes (REQ-cli-046 keeps its short attachment paths and file modes), and specs/cli/testing.md no longer claims Windows CI. tests/package.rs::supported_platforms_are_macos_and_linux_with_windows_through_wsl fails on the base and passes after; the CRLF-normalizing tests stay; fledge lanes run verify passes on Linux or macOS.

## Evidence

- Verification commit: `1d28c61b5c12a5002e752476dab72f53a42178c1`
- Base commit: `ec830cfb001cc5f71f4f83809ca3294d685f42a4`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

## The decision

Leif, in the orc tab on 2026-11-05 at 8:17 PM MST, chose "Mac + Linux; Windows via WSL", relayed by orc (hc t18). The context he chose from, as orc relayed it: match SpecSync 6.0 (no native Windows); drop native Windows CI from Chough; document and test WSL; keep tests CRLF-safe; Raven Desktop keeps Windows (not this repo). It is captured as FLOW-17. It was first drafted as FLOW-16 and renumbered before any push, because the branch of chough#123 (ch-d7) holds FLOW-16; orc's ID registry assigns FLOW-16 to ch-d7. Orc asked for this change to merge early because other Chough pull requests wait on the Windows job.

## What exists today

- `.github/workflows/ci.yml` runs `Product / windows-latest` (cargo test and a release build) beside Ubuntu and macOS. Branch protection on main requires no status check (read 2026-11-05: empty contexts and checks, rulesets `[]`), so removing the job blocks no merge.
- README "Tested platforms" lists "Hosted CI: Ubuntu, macOS, Windows", and Known limitations has a Windows `taskkill` bullet. docs/ci.md says CI builds on Windows and recommends `Product / windows-latest` as a required check. specs/cli/testing.md says Windows CI exercises `/`-separated keys and CRLF front matter.
- SpecSync 6.2.0 publishes Linux and macOS assets only, and its README says "Windows is not a supported target as of 6.0" and points to WSL.

## Requirements that name Windows

No HI criterion and no requirement promises that Chough runs on native Windows. The only requirement text that names Windows is REQ-cli-046 (#109): attachment folders are short and paths stop at 220 characters "so attachment paths stay short enough to check out on Windows". That is a rule about the tracked tree a repository holds, which any Git client may clone, so it stays, and so does its file-mode rule (a Unix guarantee). Module IDs that refuse Windows reserved names and `<>:"|?*`, and case env names that are equal ignoring ASCII case, stay for the same reason: they keep files portable and match what SpecSync accepts.

## Approval

Leif: "Approve, A on D1-D5", relayed by orc (hc t18) on 2026-11-05, answering the intent and scope gate for this change while it was still numbered FLOW-16. D1 to D5 are the five choices in that gate: no requirement delta; keep the `cfg(windows)` code, documented as unsupported and untested; the Ubuntu jobs cover WSL 2, with no WSL runner; close no issues; leave the `ci-windows` labels on the earlier case definitions.

## Issues

No open issue is about Windows only. #30 mentions a Windows `\\?\` path failure that was fixed long ago; its subject (parallel-safe identities) stays open. #109 is a merged pull request whose Windows items were fixed by the short-folder rule, which stays. Nothing is proposed for closing.

## From the change's design.md

# Design

No visual or interaction design applies.

The product job keeps its name `Product / ${{ matrix.os }}`, so `Product / ubuntu-latest` and `Product / macos-latest` keep the same names. No other job changes.

The new package test reads `.github/workflows/ci.yml`, `fledge.toml`, README.md, docs/ci.md and docs/integrations.md, with CRLF normalized to LF. Each check that fails names its file:

- the product job's matrix is exactly `[ubuntu-latest, macos-latest]`, and no line of ci.yml names Windows;
- the `native-gates` job runs on `ubuntu-latest` and runs `fledge lanes run verify`, the verify lane includes `exercise`, and the exercise task runs `examples/feature-bug-loop.sh`. Together these mean a Linux build runs the exercise on every pull request;
- README's Tested platforms names Ubuntu and macOS for hosted CI and says Windows is supported through WSL 2 with Ubuntu; README, docs/ci.md and docs/integrations.md list no `Product / windows-latest` and claim no Windows CI.

## From the change's testing.md

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

## Where these lessons go

- `specs/cli/context.md`
