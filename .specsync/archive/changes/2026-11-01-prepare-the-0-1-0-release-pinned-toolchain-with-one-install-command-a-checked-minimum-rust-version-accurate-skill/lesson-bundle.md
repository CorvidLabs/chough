# Lesson bundle: prepare-the-0-1-0-release-pinned-toolchain-with-one-install-command-a-checked-minimum-rust-version-accurate-skill

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Prepare the 0.1.0 release: pinned toolchain with one install command, a checked minimum Rust version, accurate skill guidance and contract wording, and a change log
- **Kind**: Operations
- **Specs**: cli
- **Paths**: Cargo.toml, .github/workflows/ci.yml, .github/scripts/, AGENTS.md, README.md, CHANGELOG.md, docs/, tests/package.rs, src/skill.rs, skills/chough-workflow/, .agents/skills/chough-workflow/, .claude/skills/chough-workflow/, specs/cli/
- **Acceptance**: Tool pins in one place each, kept in agreement: README.md, AGENTS.md and .github/workflows/ci.yml name the same versions, HI 0.8.0 (crate human-intent), SpecSync 6.1.2 and Fledge 1.8.0. CI installs Fledge with the Fledge GitHub Action at its v1.8.0 release (`version: v1.8.0`, the release tagged on 2026-11-01 at the commit published to crates.io) and HI with `cargo install human-intent --version 0.8.0 --locked` as before; the SpecSync Action steps, including the Archive gate step, are pinned to the v6.1.2 release commit d08d2746f635bc26900601376d7392395d07faa9 with version '6.1.2', and .github/scripts/check-specsync-archive.py and .github/scripts/test-specsync-archive.py expect `specsync 6.1.2`. docs/ci.md and docs/integrations.md say Fledge 1.8.0, and docs/DECISIONS.md records the switch as D-019. Recorded exercises keep the versions they observed (LIFE-6, LIFE-9, REQ-cli-019).
- **Acceptance**: One-command install: README.md has an `Install the toolchain` section whose single command is `cargo install --locked human-intent@0.8.0 fledge@1.8.0 specsync@6.1.2`, says it needs Rust 1.89 or later (SpecSync 6.1.2 declares 1.89), and says how to install chough itself. That exact command was run with the local cargo into a scratch root before approval and installed hi, specsync and fledge at those versions (LIFE-9).
- **Acceptance**: Pins are checked: a test in tests/package.rs fails, naming the file and the tool, when the HI, SpecSync or Fledge version is missing from README.md, AGENTS.md or .github/workflows/ci.yml or disagrees between them, or when the minimum Rust version is missing from Cargo.toml, README.md or the CI MSRV job or disagrees between them (REQ-cli-019).
- **Acceptance**: Minimum Rust version: Cargo.toml declares rust-version = "1.88", README.md says Rust 1.88 or later for chough, and a CI job `MSRV / 1.88.0`, which needs the Archive job like every other job, runs `cargo test --locked` with Rust 1.88.0 on Ubuntu, so the claim stays true (REQ-cli-019).
- **Acceptance**: Skill guidance matches the helper: the nine verified corrections from the 2026-11-01 accuracy review are applied to skills/chough-workflow: SKILL.md (`--host` replaces the openai default, repeat it per host), references/stages.md (resume checks nothing itself; run `chough next`), references/validate.md (a stop request persists until `validate resume` and also halts later runs; pausing pauses a run; `monitoring` only allows entering the stage while `validate run` needs `repeat-validation`), references/testing.md (`timeout_seconds` 1 to 3600, default 600, with a timeout recorded failed; fledge-task and command need an expect_output fragment; `cases record` exits 3 when not passed), templates/cases.toml (the fledge-task, command, timeout_seconds and env fields), references/checkpoint.md (`--clear-awaiting-human` clears every open gate; gates still waiting are passed again and logged closed and opened again) and references/lifecycle.md (refused:no-native-change, ungated:not-configured and ungated:sdd-disabled, and a correction reported invalid). The bundle stays free of U+2013 and U+2014 (LIFE-4, REQ-cli-019).
- **Acceptance**: Installed copies stay current: .agents/skills/chough-workflow and .claude/skills/chough-workflow are byte-identical to the canonical bundle, and KNOWN_PREVIOUS in src/skill.rs gains the 5af3f78 digest of every bundle file this change edits, so `chough skill status` reports an unedited copy from main before this change as known-outdated, `chough skill update` replaces it, and an edited copy stays edited (LIFE-4, REQ-cli-016).
- **Acceptance**: Contract wording is accurate and otherwise unchanged: specs/cli/context.md records the archived changes of PRs #3 to #18 and the current working files and record directories; REQ-cli-016's acceptance no longer speaks of a 0.1.0 bundle and instead names a file byte-identical to a known earlier build; REQ-cli-008's acceptance adds that reaching test or accept is refused for draft, unavailable and approval-invalid states and allowed for a valid approval or an authenticated archive, matching src/stage.rs; REQ-cli-019 adds the minimum Rust version and the toolchain pins. These are MODIFIED deltas only; no requirement is added or removed and no HI criterion is reworded.
- **Acceptance**: Release notes and README after publish: CHANGELOG.md has a 0.1.0 entry marked unreleased that summarizes the delivered work of PRs #2 to #19 from the roadmap and Git history and says this is the first release. README.md stays true both before and after publishing, because crates.io keeps the README as published: it gives `cargo install chough --locked` for released versions and the Git install for unreleased main, and does not claim that a release exists or does not exist. docs/releasing.md lists the human release steps after merge (date the change log, package dry run, tag, publish, GitHub release, a clean install observation).
- **Acceptance**: Nothing is released by this change: no tag, crates.io publish or GitHub release is made, and no feature, command or integration is added. The full suite on Ubuntu, macOS and Windows, the MSRV job, the Fledge verify lane, HI and SpecSync checks pass in CI.

## Evidence

- Verification commit: `25ca8164549a9b35545bd27cf55799bacdeb1fab`
- Base commit: `5af3f78d66b40a5af42607eafd5649c9b957d2ad`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

0xLeif, 2026-11-01: "we need to have solid version numbers for hi, spec-sync, and
fledge.. and make sure locally we are on the latest and all is good.. and it is easy
to install them for chough". Asked how to pin Fledge, 0xLeif chose "Pin 1.8.0,
install via cargo". The direction for this change is release preparation only: no new
features and no new integrations.

What is true at 5af3f78 (origin/main):

- AGENTS.md says to match CI with hi 0.8.0, fledge 1.7.2, specsync 6.1.1. CI installs
  HI with `cargo install human-intent --version 0.8.0 --locked`, Fledge 1.7.2 through
  the Fledge Action at a pinned commit, and SpecSync through its v6.1.1 Action commit.
  Fledge 1.8.0 is on crates.io but has no GitHub release, so the Action cannot fetch
  it. README has no toolchain install instructions.
- Locally (macOS arm64): hi 0.8.0, fledge 1.8.0 and specsync 6.1.1 are on PATH; the
  local specsync was installed from a source path, not crates.io. chough itself is
  not installed on PATH. `cargo install --root <scratch> --locked human-intent@0.8.0
  fledge@1.8.0 specsync@6.1.1` with cargo 1.95.0 installed all three at those versions
  (about six minutes cold; cargo warns that a locked dependency, yoke-derive 0.8.3, is
  yanked). SpecSync 6.1.1 declares rust-version 1.89.
- Cargo.toml declares no rust-version. Rust 1.85.0 fails to build (let-chains, E0658);
  `cargo +1.88.0 build --locked` succeeds and `cargo +1.88.0 test --locked --no-run`
  compiles every test target.
- An accuracy review of the skill bundle against src/ at 5af3f78 found nine verified
  gaps (seven confirmed, two corrected by the verifier), listed in the design. One
  more finding of that review, the missing rust-version, is item 2 here.
- specs/cli/context.md stops its history at the first approved change and lists
  neither timing.rs nor the artifacts, validation and retention records.
  REQ-cli-016's acceptance speaks of "an unmodified 0.1.0 bundle file" although no
  0.1.0 exists; REQ-cli-008's acceptance covers build and done but not test and
  accept, which src/stage.rs gates (`prerequisite`, ApprovedOrArchived) and
  tests/stages.rs already exercises.
- Cargo.toml is at version 0.1.0 with a package include list; nothing is tagged or
  published. There is no CHANGELOG.

Open pull requests that touch the same files: #20 (Gemini host cold start) edits
skills/chough-workflow/SKILL.md, references/checkpoint.md and KNOWN_PREVIOUS (it adds
the same two 5af3f78 digests this change needs); #21 (docs accuracy sweep) edits
README.md, docs/ci.md, docs/integrations.md and docs/ROADMAP.md, and already adds a
Rust 1.88 and `cargo install chough --locked` sentence to README. Whichever merges
later rebases. Case IDs from TC-cli-0110 are free on main, #20 (up to TC-cli-0105) and
#21.

- Update (2026-11-01, later): fledge v1.8.0 was then tagged on GitHub at the commit
  published to crates.io, and 0xLeif chose the Fledge Action at v1.8.0 for CI.
- Update (2026-11-01, later still): SpecSync 6.1.2 is being released with #823, #824,
  #828, #829, #830, #832 and #834 (at this writing crates.io has 6.1.1 and only the
  v6.1.2-rc.1 tag exists). 0xLeif decided that 6.1.2 blocks Chough 0.1.0 and chose
  to pin it instead of 6.1.1 (README, AGENTS.md, CI and the SpecSync Action commit).
  SpecSync 6.1.2 still declares rust-version 1.89 (Cargo.toml on origin/main of
  CorvidLabs/spec-sync). The 6.1.1 observations above are history.
- Update (2026-11-01, publish): SpecSync 6.1.2 is published (GitHub release v6.1.2 at
  d08d2746f635bc26900601376d7392395d07faa9, Latest; crates.io serves 6.1.2). Before
  re-approval the README command `cargo install --locked human-intent@0.8.0
  fledge@1.8.0 specsync@6.1.2` was run with cargo 1.95.0 into an empty scratch root:
  exit 0 in 3m39s, printing `hi 0.8.0`, `fledge 1.8.0` and `specsync 6.1.2`. The v6.1.2
  action.yml carries the archive-gate-only input, so the Archive gate step can move
  to the same commit. 0xLeif chose to add `.github/scripts/` to scope, because both
  archive scripts hard-code `specsync 6.1.1`.

## From the change's design.md

# Design

## 1. Tool pins and one install command (REQ-cli-019, LIFE-6, LIFE-9)

Pins: HI 0.8.0 (crate `human-intent`, binary `hi`), SpecSync 6.1.2 (crate and binary
`specsync`), Fledge 1.8.0 (crate and binary `fledge`). Each place that states a pin
keeps its own wording; the test in section 3 keeps them in agreement, so there is no
generated file or new config to read.

- `.github/workflows/ci.yml`, job `native-gates`: keep the `CorvidLabs/fledge` Action
  step and set `version: v1.8.0` (the GitHub release tagged 2026-11-01 at the commit
  published to crates.io; 0xLeif chose the Action over cargo for CI speed); keep the HI
  cargo install after `Swatinem/rust-cache`. rust-cache v2.8
  caches `~/.cargo/bin`, so a warm cache makes both installs no-ops; a cold run
  compiles them (minutes; measured about six minutes for all three tools locally).
  The SpecSync Action steps move to the v6.1.2 release commit (d08d2746) with
  `version: '6.1.2'` because they also run its checks. The Archive gate step moves from
  the #832 main commit to the same v6.1.2 commit, which carries the archive gate inputs,
  and its comment says so.
- `.github/scripts/check-specsync-archive.py` (`VERSION`) and
  `.github/scripts/test-specsync-archive.py` (the version guard, its message and the
  fake version output) expect `specsync 6.1.2`, so the archive check and its fixture
  test agree with the pinned SpecSync.
- `AGENTS.md`: "Match tool versions to CI: hi 0.8.0, fledge 1.8.0, specsync 6.1.2." and
  a pointer to the README toolchain section.
- `README.md`, new section `Install the toolchain` after `Install`:

  ```sh
  cargo install --locked human-intent@0.8.0 fledge@1.8.0 specsync@6.1.2
  ```

  with: these are the versions CI uses; the command needs Rust 1.89 or later because
  SpecSync 6.1.2 declares it; cargo skips a tool already installed at that version from
  crates.io and replaces a different version (`--force` reinstalls). Verified with
  cargo 1.95.0 into a scratch `--root`.
- `docs/ci.md` and `docs/integrations.md`: Fledge 1.8.0 (CI uses the Fledge Action at v1.8.0).
  `docs/DECISIONS.md` D-019: pin Fledge 1.8.0 (CI uses the Fledge Action at its v1.8.0
  release, local installs use cargo); one README command installs all three. Recorded
  exercises (cold-start.md, specs/cli/testing.md environment lines) keep the versions
  they observed.
- No `fledge run tools` task: Fledge must already be installed to run it, and the
  README command is the bootstrap. See open question 3.

## 2. Minimum Rust version (REQ-cli-019)

`Cargo.toml`: `rust-version = "1.88"` after `edition`. Cargo then refuses older
toolchains with a clear message instead of E0658. New CI job:

```yaml
  msrv:
    name: MSRV / 1.88.0
    needs: archive-gate
    runs-on: ubuntu-latest
    timeout-minutes: 20
    steps:
      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2
        with:
          persist-credentials: false
      - uses: dtolnay/rust-toolchain@b725eda525362e7cf34af9a7243cac1e3f01a7ec # stable
        with:
          toolchain: 1.88.0
      - uses: Swatinem/rust-cache@f13886b937689c021905a6b90929199931d60db1 # v2.8.1
      - run: cargo test --locked
```

The pinned dtolnay commit accepts a `toolchain` input on any ref. Ubuntu only: the
platform matrix already runs on stable. README's Install section says Rust 1.88 or
later for chough (the toolchain command's 1.89 is separate and stated there).

## 3. Pin agreement test (REQ-cli-019)

`tests/package.rs` gains `tool_and_rust_version_pins_agree`, plain string parsing
with no new dependency:

- README.md: the line in the `Install the toolchain` code block starting with
  `cargo install --locked`; tokens `<crate>@<version>` for human-intent, fledge and
  specsync. Also the phrase `Rust <rust-version> or later`.
- AGENTS.md: `hi <v>`, `fledge <v>`, `specsync <v>` in the "Match tool versions to
  CI" sentence.
- ci.yml: `cargo install human-intent --version <v> --locked`, the `CorvidLabs/fledge` step's
  `version: v<v>`, every `version: '<v>'` input of a `CorvidLabs/spec-sync` step
  (all equal), and the msrv job's `toolchain: <v>` whose
  major.minor equals Cargo.toml's `rust-version`.
- Cargo.toml: `rust-version`.

A missing pin or a disagreement fails with `<file>: <tool> pinned <found>, expected
<expected>` (or `not found`). The test lives in package.rs because it is about what
an installer of Chough needs; tests/package.rs is already outside the cli spec's file
list, like the other package tests.

## 4. Skill bundle corrections (LIFE-4, REQ-cli-016, REQ-cli-019)

Apply the nine verified findings of the 2026-11-01 review, with the verifier's wording
where it corrected the proposal. Each was checked against 5af3f78:

| File | Correction | Source of truth |
|---|---|---|
| SKILL.md | `--host` replaces the openai default; repeat it per host | src/main.rs:208-212 |
| references/stages.md | Resume checks nothing itself; run `chough next --environment <env>` | src/stage.rs:738-748 |
| references/validate.md | Stop leaves `.chough/validation.stop`, halting the running and any later run until `validate resume` clears it; pausing the stage pauses a run | src/validate.rs:107-117, 140-144, 294-309 |
| references/validate.md | `monitoring` only allows entering the stage; `validate run` needs `repeat-validation` consent or `always` | src/stage.rs:786-792, src/validate.rs:67-76 |
| references/testing.md | `timeout_seconds` 1 to 3600, default 600, timeout recorded failed; fledge-task and command need an expect_output fragment | src/cases.rs:120, 384-389, 432-447, 845-857 |
| references/testing.md | `cases record` exits 3 when not passed | src/cases.rs:1074, 1116-1121 |
| templates/cases.toml | Commented fledge-task (`task`), command (`argv`), `timeout_seconds` and `env` lines | src/cases.rs:410-447 |
| references/checkpoint.md | `--clear-awaiting-human` clears every open gate; re-pass the gates still waiting; they are logged closed and opened again | src/checkpoint.rs:462-520 |
| references/lifecycle.md | refused:no-native-change, ungated:not-configured, ungated:sdd-disabled (no config, no sdd.json, or enabled false), correction invalid | src/stage.rs:203-216, 286-307, 378-390 |

Wording uses colons or commas, never U+2013 or U+2014 (the existing package test
enforces it). `chough skill update` refreshes `.agents/` and `.claude/` copies.
`src/skill.rs` KNOWN_PREVIOUS gains, under `// Main 5af3f78 (PR #19); superseded by the
0.1.0 release prep.`, these previous digests:

| File | SHA-256 at 5af3f78 |
|---|---|
| SKILL.md | d350cc91271e6d942bef516a2b2ea8a6009697a2e0d034b9ff48c8591be2ee5c |
| references/stages.md | 5127182e148521e9c9df2490eb45055d0c5edb3302f245bc4f7aab114b591a24 |
| references/validate.md | 5b7ea4ec1a12c745e7d5a14625f4f98ab2731bf13b588986e4f0a6b7686b4aed |
| references/testing.md | a056931078182d3bd887cfa96c6bc4bd68a83b955e479bca95266bf1d1c1ee7c |
| references/checkpoint.md | 20daaaa0cec972169abcedd508cd9ad18ff93ef7ec9d1f4e7330fcd18ca80ac0 |
| references/lifecycle.md | a776c2f85d8803d78d24bd124773b73be1b09dd190c5cc9b6dd5a11fe4cc09c3 |
| templates/cases.toml | 8a07e72f5f72668f6b440a14d457a8352850a94413ba7e1b5962abc5cdae234a |

If #20 merges first, its SKILL.md and checkpoint.md entries duplicate two of these
(keep one each) and its new digests of those two files are added too, since this
change then edits them again.

## 5. Contract wording (REQ-cli-008, REQ-cli-016, REQ-cli-019)

- specs/cli/context.md: apply the reviewed patch (history of PRs #3 to #18, timing.rs
  in the module list, artifacts, validation and retention records, the PR #2 squash
  reference). Not a requirement change.
- Delta `deltas/cli.md`, MODIFIED only: REQ-cli-008 acceptance adds the test and
  accept routes; REQ-cli-016 acceptance drops "0.1.0"; REQ-cli-019 statement and
  acceptance add the rust-version and the pins. The statements of REQ-cli-008 and
  REQ-cli-016 are unchanged. Materialized by `specsync change check` after approval.

## 6. CHANGELOG and publish-time README

CHANGELOG.md (Keep a Changelog style, not in the package include list):
`## [0.1.0] - Unreleased`, "First release." then Added/Changed grouped by roadmap row:
resumable stage workflow and checkpoint schema 2 (#2), archive CI gate (#3, #18),
native status parsing and checkpoint clears (#4, #6), fail-closed stage gates (#7),
bounded external commands (#8), evidence retention and installable package (#9),
evidence freshness by content and bug verification order (#17), `chough timing` (#15),
bundle hygiene, binding env, colour-safe matching and actionable errors (#16, including
the expect_output ESC refusal), and this release prep (pins, MSRV, guidance). Known
limitations link README. Each line names the PR; nothing claims a tag or publish.

README.md `Install`: "Chough needs Rust 1.88 or later." then `cargo install chough
--locked` for released versions (listed in CHANGELOG.md), `cargo install --git
https://github.com/CorvidLabs/chough --locked` for unreleased main, and the clone
route. The status paragraph drops "No release exists yet ... in progress" and says
release history is in CHANGELOG.md and that nothing is hosted or deployed. This reads
correctly both before publishing (the change log says Unreleased) and in the copy
crates.io freezes at publish. See open question 1.

## 7. Release steps note

docs/releasing.md, for the human after this change merges; nothing here runs them:
confirm main CI is green on the merge commit; `cargo package --list --locked` matches
the include list; `cargo publish --dry-run --locked`; date the 0.1.0 change log entry
(a small docs pull request); tag `v0.1.0` on that commit and push it; `cargo publish
--locked`; create the GitHub release from the change log section; in a clean
environment run `cargo install chough --locked` and `chough --version` and record an
installation-observation. Each is a separate human gate (Chough's optional release
stage, references/release.md).

## Out of the design

Tagging, publishing, a GitHub release, a version bump, new commands or adapters,
cargo-binstall or other install integrations, a `fledge run tools` task, and edits to
recorded exercises.

## From the change's testing.md

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
  directory, installs `hi 0.8.0`, `fledge 1.8.0` and `specsync 6.1.2`
  (`--version` of each). The same command was already run before approval with cargo
  1.95.0 and passed (about six minutes cold); with specsync 6.1.2 it passed again
  before re-approval (cargo 1.95.0, 3m39s).
- Package: the existing package test
  `the_crate_package_holds_only_the_helper_source_and_canonical_skill` still passes;
  CHANGELOG.md and docs are not packaged.

## Regression

`python3 .github/scripts/test-specsync-archive.py` with SpecSync 6.1.2 installed. Full
`cargo test --locked`, `fledge lanes run verify` with Fledge 1.8.0, HI 0.8.0 and
SpecSync 6.1.2 locally, and CI on Ubuntu, macOS and Windows plus MSRV. In CI, the
`native-gates` job installs Fledge with cargo and runs the verify lane: that run is the
evidence that the Action replacement works.

Case IDs TC-cli-0110 to TC-cli-0114 are reserved for this change (free on main, #20
and #21).

## Where these lessons go

- `specs/cli/context.md`
