---
change: prepare-the-0-1-0-release-pinned-toolchain-with-one-install-command-a-checked-minimum-rust-version-accurate-skill
artifact: plan
---

# Plan

## Approach

Small, independent edits with one new regression test; no product behavior changes.
One agent builds. If split, ownership is: CI, Cargo.toml, AGENTS.md and the pin test
(owner A); skill bundle, host copies and src/skill.rs (owner B); README, CHANGELOG,
docs and specs/cli/context.md (owner C). The delta is already drafted.

| Increment | Relative size | Main uncertainty |
|---|---|---|
| CI: Fledge via cargo, MSRV job | small | cold-cache CI time for compiling Fledge |
| Cargo.toml rust-version, AGENTS.md pins | small | none |
| Pin agreement test | small | parsing stays simple as wording evolves |
| Nine skill corrections, KNOWN_PREVIOUS, host refresh | small to medium | rebase with #20 on the same files |
| specs/cli/context.md patch | small | rebase with later merges |
| README install sections, CHANGELOG, docs/releasing.md, docs pins, D-019 | small to medium | rebase with #21 on README and docs |

History from `chough timing` is not used for hours: this is mostly documentation and
the earlier changes were feature work, so the ranges do not transfer; relative size
only.

## Out of scope

Tagging v0.1.0, `cargo publish`, a GitHub release, a version bump; new commands,
adapters or install integrations (cargo-binstall, prebuilt binaries); a `fledge run
tools` task (open question 3); rewriting recorded exercises; `.github/scripts/`;
PR #20 and #21 content; any native SpecSync record edit.

## Steps

1. Pin agreement test first, run at the unfixed tip: it fails on AGENTS.md (fledge
   1.7.2), ci.yml (Fledge Action, no msrv job), Cargo.toml (no rust-version) and
   README (no toolchain section). Keep that failing run as the red observation of
   TC-cli-0110.
2. ci.yml: set the Fledge Action's `version: v1.8.0`; add the `msrv` job needing `archive-gate`.
3. Cargo.toml `rust-version = "1.88"`; AGENTS.md version line.
4. Skill bundle corrections; KNOWN_PREVIOUS entries; `chough skill update` for the host
   copies; check byte identity.
5. specs/cli/context.md patch.
6. README Install and Install the toolchain sections; CHANGELOG.md; docs/releasing.md;
   docs/ci.md, docs/integrations.md and D-019.
7. `fledge lanes run verify` locally with the pinned tools; `specsync change check`
   after approval materializes the delta; push; CI including MSRV.

## Risks

- Cold CI cache compiles Fledge in `native-gates` (minutes). rust-cache keeps
  `~/.cargo/bin`, so later runs skip it. A prebuilt-binary installer would be a new
  integration and is out of scope.
- Fledge 1.8.0 runs the same lanes as 1.7.2 locally (`fledge lanes list` reads
  fledge.toml); the verify lane under 1.8.0 in CI is confirmed only by the CI run.
- `cargo install --locked` warns that SpecSync's or Fledge's lock holds a yanked
  yoke-derive 0.8.3; it still installs. Upstream note, not fixed here.
- The README wording for after publish (open question 1) is a judgment call.
- Overlap with #20 (SKILL.md, checkpoint.md, KNOWN_PREVIOUS) and #21 (README, docs):
  whichever merges second rebases; KNOWN_PREVIOUS must then hold the digests of the
  bundle as it was just before this change's edit.
- The MSRV job adds one Ubuntu job per run.

## Constraints consulted

specs/cli/context.md lessons (plain-text gates; frozen deltas; Fledge Action ref
history); Invariant 1 and REQ-cli-016 (known earlier digests only); REQ-cli-019
(package contents, no dashes); AGENTS.md (no new integration, no unobserved claims,
nothing published as a side effect); the archive-first CI rule.
