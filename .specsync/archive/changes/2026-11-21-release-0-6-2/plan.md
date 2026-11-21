---
change: release-0-6-2
artifact: plan
---

# Plan

## Approach

Follow `docs/releasing.md`, as 0.6.1 did. Set `version = "0.6.2"` in `Cargo.toml` and let
Cargo update `Cargo.lock`. Assemble the fragments in `changelog.d/` into `CHANGELOG.md`
with `fledge run changelog -- --release 0.6.2 --date <date>`; every fragment there is a
merged change's. Rebuild the knowledge index with `fledge run release-check` and commit
what it changes.

## Out of scope

The tag, `cargo publish`, the GitHub release, the clean-install observation, the
installed binaries on this Mac and the roadmap row. No code, spec or skill change. A test
in `tests/changelog.rs` that reads a live fragment the release removes is changed only so
it holds after the release, as 0.6.1 did, reading the released section or a fixture copy
under `tests/fixtures/changelog/`.

## Steps

1. Bump the version and commit.
2. Run `cargo package --list --locked` and `cargo publish --dry-run --locked`, and check
   the crates.io owner.
3. Assemble the change log and run the release check.
4. Run the verify lane, open the pull request, and stop for review.

## Risks

- A changelog test that reads a live fragment fails once the release removes it; it is
  changed as 0.6.1 changed its two.
- The release date in the heading is the preparation date; if the tag lands on a later
  day, the heading's date is set to it in the same branch before merge.

## Constraints consulted

`docs/releasing.md`; LIFE-25 (a 0.x release ships from main when CI is green) and LIFE-16
(the change log lists every shipped change under the release that first held it).
