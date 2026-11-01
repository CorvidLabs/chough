---
change: prepare-the-0-1-0-release-pinned-toolchain-with-one-install-command-a-checked-minimum-rust-version-accurate-skill
artifact: tasks
---

# Tasks

- [x] Add the pin agreement test to tests/package.rs and record its failing run at the unfixed tip.
- [x] Set the Fledge Action in ci.yml to `version: v1.8.0`.
- [x] Add the `MSRV / 1.88.0` CI job needing the Archive job.
- [x] Add `rust-version = "1.88"` to Cargo.toml.
- [x] Update the AGENTS.md tool version line to hi 0.8.0, fledge 1.8.0, specsync 6.1.2.
- [x] Move the SpecSync Action steps and the Archive gate step in ci.yml to the v6.1.2 commit with version '6.1.2'.
- [x] Update .github/scripts/check-specsync-archive.py and test-specsync-archive.py to expect specsync 6.1.2.
- [x] Update the README toolchain command and the tests/package.rs pins to specsync 6.1.2.
- [x] Apply the nine skill bundle corrections.
- [x] Add the 5af3f78 digests of the seven edited bundle files to KNOWN_PREVIOUS.
- [x] Refresh the .agents/ and .claude/ copies with `chough skill update` and confirm byte identity.
- [x] Apply the specs/cli/context.md history and working-files update.
- [x] Rewrite README Install and add Install the toolchain.
- [x] Add CHANGELOG.md with the unreleased 0.1.0 entry.
- [x] Add docs/releasing.md; update docs/ci.md, docs/integrations.md and add D-019 to docs/DECISIONS.md.

Human gates and post-verification steps (not tasks): the human's scope approval of
this change; `specsync change check` to materialize the delta; rerunning
TC-cli-0110 to TC-cli-0114 at the implementation tip; `fledge lanes run verify`
locally; push and CI (Ubuntu, macOS, Windows, MSRV); the human implementation review;
native review and finalize. Tag, publish and GitHub release are separate human steps
after merge, listed in docs/releasing.md.
