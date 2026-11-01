---
change: prepare-the-0-1-0-release-pinned-toolchain-with-one-install-command-a-checked-minimum-rust-version-accurate-skill
artifact: testing
---

# Testing

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-cli-019 | TC-cli-0110 (new, automated): `tests/package.rs::tool_and_rust_version_pins_agree` fails at the unfixed tip and passes after; the same checker given copies with one pin changed fails naming that file and tool |
| REQ-cli-019 | TC-cli-0111 (new, automated, command adapter): `cargo +1.88.0 test --locked` on macOS arm64 passes; the CI job `MSRV / 1.88.0` passes on the pull request |
| REQ-cli-019 | TC-cli-0065 (existing): bundle files from the latest main build are never reported edited, still passes with the new bundle; the existing no-dash test passes |
| REQ-cli-016 | TC-cli-0112 (new, automated): `tests/package.rs::a_skill_copy_from_main_before_the_release_prep_is_known_outdated_and_updated` writes the 5af3f78 bytes of the seven edited files, expects known-outdated, updates them to current, and leaves an edited copy edited; TC-cli-0064 still passes |
| REQ-cli-008 | TC-cli-0040 and TC-cli-0041 (existing): every route into test and accept is refused for an invalid approval and allowed after approval or an authenticated archive; the modified acceptance bullet documents this tested behavior, no code changes |

## Guidance and install checks

- TC-cli-0113 (new, agent-assisted): at the implementation tip, each of the nine skill
  corrections is read against the source lines cited in the design and states what
  the code does; `.agents/` and `.claude/` copies are byte-identical to
  skills/chough-workflow (`chough skill status` reports current for both hosts).
- TC-cli-0114 (new, agent-assisted): the README `Install the toolchain` command, copied
  from README.md at the implementation tip, run with `--root` set to an empty scratch
  directory, installs `hi 0.8.0`, `fledge 1.8.0` and `specsync 6.1.1`
  (`--version` of each). The same command was already run before approval with cargo
  1.95.0 and passed (about six minutes cold).
- Package: the existing package test
  `the_crate_package_holds_only_the_helper_source_and_canonical_skill` still passes;
  CHANGELOG.md and docs are not packaged.

## Regression

Full `cargo test --locked`, `fledge lanes run verify` with Fledge 1.8.0, HI 0.8.0 and
SpecSync 6.1.1 locally, and CI on Ubuntu, macOS and Windows plus MSRV. In CI, the
`native-gates` job installs Fledge with cargo and runs the verify lane: that run is the
evidence that the Action replacement works.

Case IDs TC-cli-0110 to TC-cli-0114 are reserved for this change (free on main, #20
and #21).
