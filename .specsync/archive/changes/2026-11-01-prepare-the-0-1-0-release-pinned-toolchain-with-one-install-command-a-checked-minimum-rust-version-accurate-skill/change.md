---
id: prepare-the-0-1-0-release-pinned-toolchain-with-one-install-command-a-checked-minimum-rust-version-accurate-skill
state: archived
type: operations
base_commit: 5af3f78d66b40a5af42607eafd5649c9b957d2ad
---

# Prepare the 0.1.0 release: pinned toolchain with one install command, a checked minimum Rust version, accurate skill guidance and contract wording, and a change log

## Intent

Prepare the 0.1.0 release: pinned toolchain with one install command, a checked minimum Rust version, accurate skill guidance and contract wording, and a change log

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- Tool pins in one place each, kept in agreement: README.md, AGENTS.md and .github/workflows/ci.yml name the same versions, HI 0.8.0 (crate human-intent), SpecSync 6.1.2 and Fledge 1.8.0. CI installs Fledge with the Fledge GitHub Action at its v1.8.0 release (`version: v1.8.0`, the release tagged on 2026-11-01 at the commit published to crates.io) and HI with `cargo install human-intent --version 0.8.0 --locked` as before; the SpecSync Action steps, including the Archive gate step, are pinned to the v6.1.2 release commit d08d2746f635bc26900601376d7392395d07faa9 with version '6.1.2', and .github/scripts/check-specsync-archive.py and .github/scripts/test-specsync-archive.py expect `specsync 6.1.2`. docs/ci.md and docs/integrations.md say Fledge 1.8.0, and docs/DECISIONS.md records the switch as D-019. Recorded exercises keep the versions they observed (LIFE-6, LIFE-9, REQ-cli-019).
- One-command install: README.md has an `Install the toolchain` section whose single command is `cargo install --locked human-intent@0.8.0 fledge@1.8.0 specsync@6.1.2`, says it needs Rust 1.89 or later (SpecSync 6.1.2 declares 1.89), and says how to install chough itself. That exact command was run with the local cargo into a scratch root before approval and installed hi, specsync and fledge at those versions (LIFE-9).
- Pins are checked: a test in tests/package.rs fails, naming the file and the tool, when the HI, SpecSync or Fledge version is missing from README.md, AGENTS.md or .github/workflows/ci.yml or disagrees between them, or when the minimum Rust version is missing from Cargo.toml, README.md or the CI MSRV job or disagrees between them (REQ-cli-019).
- Minimum Rust version: Cargo.toml declares rust-version = "1.88", README.md says Rust 1.88 or later for chough, and a CI job `MSRV / 1.88.0`, which needs the Archive job like every other job, runs `cargo test --locked` with Rust 1.88.0 on Ubuntu, so the claim stays true (REQ-cli-019).
- Skill guidance matches the helper: the nine verified corrections from the 2026-11-01 accuracy review are applied to skills/chough-workflow: SKILL.md (`--host` replaces the openai default, repeat it per host), references/stages.md (resume checks nothing itself; run `chough next`), references/validate.md (a stop request persists until `validate resume` and also halts later runs; pausing pauses a run; `monitoring` only allows entering the stage while `validate run` needs `repeat-validation`), references/testing.md (`timeout_seconds` 1 to 3600, default 600, with a timeout recorded failed; fledge-task and command need an expect_output fragment; `cases record` exits 3 when not passed), templates/cases.toml (the fledge-task, command, timeout_seconds and env fields), references/checkpoint.md (`--clear-awaiting-human` clears every open gate; gates still waiting are passed again and logged closed and opened again) and references/lifecycle.md (refused:no-native-change, ungated:not-configured and ungated:sdd-disabled, and a correction reported invalid). The bundle stays free of U+2013 and U+2014 (LIFE-4, REQ-cli-019).
- Installed copies stay current: .agents/skills/chough-workflow and .claude/skills/chough-workflow are byte-identical to the canonical bundle, and KNOWN_PREVIOUS in src/skill.rs gains the 5af3f78 digest of every bundle file this change edits, so `chough skill status` reports an unedited copy from main before this change as known-outdated, `chough skill update` replaces it, and an edited copy stays edited (LIFE-4, REQ-cli-016).
- Contract wording is accurate and otherwise unchanged: specs/cli/context.md records the archived changes of PRs #3 to #18 and the current working files and record directories; REQ-cli-016's acceptance no longer speaks of a 0.1.0 bundle and instead names a file byte-identical to a known earlier build; REQ-cli-008's acceptance adds that reaching test or accept is refused for draft, unavailable and approval-invalid states and allowed for a valid approval or an authenticated archive, matching src/stage.rs; REQ-cli-019 adds the minimum Rust version and the toolchain pins. These are MODIFIED deltas only; no requirement is added or removed and no HI criterion is reworded.
- Release notes and README after publish: CHANGELOG.md has a 0.1.0 entry marked unreleased that summarizes the delivered work of PRs #2 to #19 from the roadmap and Git history and says this is the first release. README.md stays true both before and after publishing, because crates.io keeps the README as published: it gives `cargo install chough --locked` for released versions and the Git install for unreleased main, and does not claim that a release exists or does not exist. docs/releasing.md lists the human release steps after merge (date the change log, package dry run, tag, publish, GitHub release, a clean install observation).
- Nothing is released by this change: no tag, crates.io publish or GitHub release is made, and no feature, command or integration is added. The full suite on Ubuntu, macOS and Windows, the MSRV job, the Fledge verify lane, HI and SpecSync checks pass in CI.

## No-spec Rationale

Not applicable
