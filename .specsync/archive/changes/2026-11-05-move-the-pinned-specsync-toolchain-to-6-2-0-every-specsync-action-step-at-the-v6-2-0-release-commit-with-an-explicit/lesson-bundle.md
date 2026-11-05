# Lesson bundle: move-the-pinned-specsync-toolchain-to-6-2-0-every-specsync-action-step-at-the-v6-2-0-release-commit-with-an-explicit

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Move the pinned SpecSync toolchain to 6.2.0: every SpecSync Action step at the v6.2.0 release commit with an explicit version 6.2.0, and every pin in README, AGENTS, CI scripts and docs kept in agreement
- **Kind**: Operations
- **Specs**: cli
- **Paths**: README.md, AGENTS.md, .github/workflows/ci.yml, .github/scripts/check-specsync-archive.py, .github/scripts/test-specsync-archive.py, docs/ci.md, docs/integrations.md, docs/DECISIONS.md, tests/package.rs, .specsync/config.toml, CHANGELOG.md
- **Acceptance**: Every SpecSync Action step in .github/workflows/ci.yml (the Archive gate and the Fledge / HI / SpecSync and SpecSync Archive and CI gate fixtures jobs) uses CorvidLabs/spec-sync@40dcf33a6b0f0b55dcb7f17e8e98be9d55fa2c56 # v6.2.0 with version '6.2.0' passed explicitly; README.md (install command), AGENTS.md, .github/scripts/check-specsync-archive.py, .github/scripts/test-specsync-archive.py, docs/ci.md, docs/integrations.md and .specsync/config.toml name SpecSync 6.2.0 and docs/DECISIONS.md records the move; no other current 6.1.x pin remains while recorded exercises keep the versions they observed; tests/package.rs tool_and_rust_version_pins_agree passes on 6.2.0; fledge lanes run verify and the CI gate fixtures pass locally with SpecSync 6.2.0; and the hosted PR checks pass, with SpecSync step durations compared against the latest main run

## Evidence

- Verification commit: `22f2e49b4616434c473d3de019f9c51d7538f342`
- Base commit: `ded0e9ef4ec27f634d5175c4601f65b7e5f5b694`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

SpecSync 6.2.0 was released (tag v6.2.0, commit
40dcf33a6b0f0b55dcb7f17e8e98be9d55fa2c56; crate version 6.2.0, rust-version 1.89).
Chough's toolchain pins, kept in agreement by REQ-cli-019 and D-019, still name 6.1.2
with the Action at the v6.1.2 commit d08d2746. The local `specsync --version` already
reports 6.2.0. The v6.2.0 tag's action.yml still defaults its `version` input to
6.1.2, so every Action step must pass `version: '6.2.0'` explicitly.

The task came from session t18 on 2026-11-05: move to 6.2.0 through a SpecSync change,
nothing else. Release notes list read-cache and finalize speed-ups (#840, #842, #858)
whose effect on the CI SpecSync steps is to be measured on the PR run against the
latest main run.

Out of scope: any other Chough behavior; HI and Fledge pins; historical lessons, test
comments and archived records that name the version they observed.

## Scope authority

Human authority: Leif (0xLeif) replied "approve scope" in the orc tab at 2026-11-05 08:46 MST, relayed by session t18. It covers intent coverage by LIFE-9, REQ-cli-019 and D-019 with no new HI criterion, and exactly the scope in this change (pins, docs, D-021, package test, CHANGELOG). Limits: no merge, tag or release; implementation review stays a separate human decision. Native actor: claude, the acting agent account.

## From the change's design.md

# Design

No product design change. Each pin keeps its current place and form; only the version
and the Action commit change. `tests/package.rs::tool_and_rust_version_pins_agree`
continues to enforce agreement, with its mutation cases moved to the 6.2.0 strings.

## From the change's testing.md

# Testing

- `cargo test --locked --test package tool_and_rust_version_pins_agree` with the pins
  moved, and the full `fledge lanes run verify` on macOS with HI 0.8.0, Fledge 1.8.0 and
  SpecSync 6.2.0.
- `python3 .github/scripts/test-specsync-archive.py` (CI gate fixtures; needs real
  published SpecSync 6.2.0).
- A grep shows no current 6.1.x pin outside recorded history.
- Hosted PR checks are observed only after they run; SpecSync step durations are read
  with `gh run view --json jobs` for the PR run and the most recent main run.

## Where these lessons go

- `specs/cli/context.md`
