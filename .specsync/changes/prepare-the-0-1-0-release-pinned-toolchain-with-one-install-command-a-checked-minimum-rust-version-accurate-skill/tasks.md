---
change: prepare-the-0-1-0-release-pinned-toolchain-with-one-install-command-a-checked-minimum-rust-version-accurate-skill
artifact: tasks
---

# Tasks

- [ ] Add the pin agreement test to tests/package.rs and record its failing run at the unfixed tip.
- [ ] Replace the Fledge Action in ci.yml with `cargo install fledge --version 1.8.0 --locked`.
- [ ] Add the `MSRV / 1.88.0` CI job needing the Archive job.
- [ ] Add `rust-version = "1.88"` to Cargo.toml.
- [ ] Update the AGENTS.md tool version line to hi 0.8.0, fledge 1.8.0, specsync 6.1.1.
- [ ] Apply the nine skill bundle corrections.
- [ ] Add the 5af3f78 digests of the seven edited bundle files to KNOWN_PREVIOUS.
- [ ] Refresh the .agents/ and .claude/ copies with `chough skill update` and confirm byte identity.
- [ ] Apply the specs/cli/context.md history and working-files update.
- [ ] Rewrite README Install and add Install the toolchain.
- [ ] Add CHANGELOG.md with the unreleased 0.1.0 entry.
- [ ] Add docs/releasing.md; update docs/ci.md, docs/integrations.md and add D-019 to docs/DECISIONS.md.

Human gates and post-verification steps (not tasks): the human's scope approval of
this change; `specsync change check` to materialize the delta; rerunning
TC-cli-0110 to TC-cli-0114 at the implementation tip; `fledge lanes run verify`
locally; push and CI (Ubuntu, macOS, Windows, MSRV); the human implementation review;
native review and finalize. Tag, publish and GitHub release are separate human steps
after merge, listed in docs/releasing.md.
