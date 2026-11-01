---
change: prepare-the-0-1-0-release-pinned-toolchain-with-one-install-command-a-checked-minimum-rust-version-accurate-skill
artifact: context
---

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
