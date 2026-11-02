# Lesson bundle: new-project-setup-path-and-sdd-off-hint-document-hi-specsync-init-scaffold-and-specsync-change-adopt-for-a-new-project

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: New-project setup path and SDD-off hint: document HI, specsync init/scaffold and specsync change adopt for a new project, and have chough next say to run specsync change adopt when SpecSync is configured but SDD is off (issue 40)
- **Kind**: Feature
- **Specs**: cli
- **Paths**: CHANGELOG.md, README.md, docs/walkthrough.md, docs/exercises/feature-bug-loop.md, docs/cli.md, specs/cli/, src/stage.rs, tests/stages.rs
- **Acceptance**: When SpecSync is configured but SDD is disabled or not adopted, chough next reports the approved-delta line as `not checked: native SDD is disabled; run specsync change adopt to gate stages`, and the ungated:sdd-disabled gate detail (shown by stage advance and status) names `specsync change adopt`; not checked: no native lifecycle stays when SpecSync is not configured, and a malformed policy is unchanged
- **Acceptance**: docs/walkthrough.md starts with a New project section covering a minimal hi/<family>.md, specsync init and scaffold with files filled in, specsync change adopt and what ungated:sdd-disabled means; README Start here gains only a one-line pointer to it
- **Acceptance**: docs/walkthrough.md shows the current BUG-<slug>-<24 hex> ID form, and docs/exercises/feature-bug-loop.md states the SpecSync version only as observed
- **Acceptance**: A Linux reproduction of issue 40 and the new tests fail before the change and pass after it; fledge lanes run verify passes; hosted CI is reported only as observed

## Evidence

- Verification commit: `6a6f0bd1f2dd720f3b51a80b4bef63eceaca2d09`
- Base commit: `825a3805530456005e361dfa6d96d93eee52cd09`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

Issue #40 came from a Linux x86_64 dogfood run of a new sample project. Versions were chough from main 825a380, hi 0.8.0, specsync 6.1.2 and fledge 1.8.0. `specsync init` writes `.specsync/sdd.json` with `enabled: false` and prints "SDD change workflow is off. Enable with `specsync change adopt`". After that, `chough next` said only "Approved deltas: not checked: no native lifecycle", and plan -> build passed as `ungated:sdd-disabled`. Nothing in Chough's output or docs pointed at `specsync change adopt`. Once adopted, the same view said "not checked: no native change recorded". The reproduction is in `.chough/artifacts/bug40-repro.log`.

The docs also lag. The walkthrough shows the legacy `BUG-0001` form, and the exercise notes record specsync 6.1.1 while the README pins 6.1.2.

Constraints: PR #44 (0xLeif) owns `SKILL.md`, `references/lifecycle.md` and the README rewrite, so README gets one pointer line only. Chough reports native state and never runs `adopt` for the user.

## From the change's design.md

# Design

- `src/stage.rs` `delta_summary`: split the `NotConfigured | Disabled` arm. `NotConfigured` keeps "not checked: no native lifecycle", and `Disabled` gets the adopt hint.
- `src/stage.rs` Disabled gate detail: append "Run `specsync change adopt` to gate stages." The label stays `ungated:sdd-disabled` and the move stays allowed, so this changes reporting only.
- Docs: add a "New project" section at the top of `docs/walkthrough.md`, replace the legacy bug IDs with the `BUG-<slug>-<24 hex>` form, add a version note in `docs/exercises/feature-bug-loop.md` stating only what was observed, add a `docs/cli.md` note, add one README pointer line under Start here, and add a CHANGELOG entry.

## From the change's testing.md

# Testing

- The new test runs in a fixture with stubbed `hi` and `specsync`. It checks an unconfigured project (old line, no hint), a configured project with no policy, `enabled: false` (hint in the JSON and text `next`, gate detail, and `status`) and an invalid policy (unchanged, no hint). It fails before the change.
- Case `TC-cli-sdd-off-names-specsync-change-adopt` (cargo-test, target `stages`) is linked to REQ-cli-031.
- The Linux reproduction is `.chough/artifacts/bug40-repro.log`. `fledge lanes run verify` covers the full suite, lint, the walkthrough-backed exercise and audit. Hosted CI is reported only as observed.

## Where these lessons go

- `specs/cli/context.md`
