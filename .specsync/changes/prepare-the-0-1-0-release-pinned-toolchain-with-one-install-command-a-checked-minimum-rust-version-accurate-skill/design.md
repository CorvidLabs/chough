---
change: prepare-the-0-1-0-release-pinned-toolchain-with-one-install-command-a-checked-minimum-rust-version-accurate-skill
artifact: design
---

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
