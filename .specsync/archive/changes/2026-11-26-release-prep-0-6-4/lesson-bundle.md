# Lesson bundle: release-prep-0-6-4

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Release-prep-0-6-4
- **Kind**: Feature
- **Paths**: Cargo.toml, Cargo.lock, CHANGELOG.md, changelog.d, knowledge/index.md, knowledge/journal/feed.xml
- **Acceptance**: Cargo.toml and Cargo.lock say 0.6.4; CHANGELOG.md has a 0.6.4 section dated 2026-11-26 with every merged fragment from changelog.d, which is then empty, and its release link; fledge run release-check passes; cargo build --locked and cargo test --locked pass. Nothing is tagged or published.

## Evidence

- Verification commit: `fe8207e6e32c6999452c292bb3e0a4c710ee985d`
- Base commit: `2577da8e23a07a308511a21d992c83419eac74e2`
- Verified by: `specsync check (no spec in scope)`

## From the change's context.md

# Context

Three structurally important changes are unreleased on main since 0.6.3 (2026-11-22):
the six overnight record/gate fixes (#448), `chough change approve` relay passthrough
(#450, REQ-cli-071), and chough-first skill routing (#451, REQ-cli-072). CI is green at
merge commit 2577da8e; tests verified on a fresh clone 2026-11-26 (14/14 unit + 24/24
gates). Per LIFE-25 a weekly 0.x release ships from main when CI is green; the agent
prepares changelog + version bump + green checks, Leif gives a one-line go ("Yes let's
do that", pi chat t14z 2026-11-26), then it publishes. The tag/publish/GitHub-release/
install steps in docs/RELEASING.md are the human's and follow after this merges.

## From the change's testing.md

# Testing

Structural checks (no product behavior change — version bump + record assembly):

- `cargo test --locked` at 0.6.4 — all suites pass.
- `chough library index --check` — exit 0, index current.
- `fledge run release-check` — exit 0.

## Where these lessons go

This change declared no affected specs, so there is no module context to fold into.
