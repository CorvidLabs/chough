---
change: release-0-6-3
artifact: plan
---

# Plan

## Approach

Follow `docs/releasing.md`, as 0.6.2 did. Set `version = "0.6.3"` in `Cargo.toml` and
let Cargo update `Cargo.lock`. Assemble the fragments in `changelog.d/` into
`CHANGELOG.md` with `fledge run changelog -- --release 0.6.3 --date 2026-11-22`.
Every fragment there is a merged change's. Rebuild the knowledge index with
`fledge run release-check` and commit what it changes.

## Out of scope

The tag, `cargo publish`, the GitHub release, the clean-install observation and the
roadmap row. No code, spec or skill change.

## Steps

1. Bump the version.
2. Assemble the change log.
3. Rebuild the knowledge index and pass the release check.
4. Run `cargo package --list --locked` and `cargo publish --dry-run --locked`.
5. Open the pull request and stop before the tag.

## Risks

The release date in the heading is the preparation date. If the tag lands on a later
day, the heading's date is set to it in this branch before merge.

## Constraints consulted

`docs/releasing.md`; LIFE-25 (a 0.x release ships from main when CI is green) and
LIFE-16 (the change log lists every shipped change under the release that first held
it).
