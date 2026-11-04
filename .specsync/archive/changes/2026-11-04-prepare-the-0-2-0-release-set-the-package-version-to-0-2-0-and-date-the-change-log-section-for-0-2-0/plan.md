---
change: prepare-the-0-2-0-release-set-the-package-version-to-0-2-0-and-date-the-change-log-section-for-0-2-0
artifact: plan
---

# Plan

## Approach

Set `version = "0.2.0"` in Cargo.toml, let cargo update the chough entry in Cargo.lock,
and rename the change log's Unreleased heading to `[0.2.0] - 2026-11-03` with a new
empty Unreleased heading above it and a release link below the 0.1.0 one. Add
entries for #29, #33 and #26, merged since v0.1.0 without a change log entry, written
from those pull requests and the current docs (found in review 5761319339).

## Out of scope

Tagging, publishing to crates.io, creating the GitHub release, the roadmap row (marked
after publication, as #28 did for 0.1.0), and any wording change inside the moved entries.

## Steps

1. Edit Cargo.toml and run `cargo check --locked` after `cargo update -p chough` to
   refresh Cargo.lock.
2. Edit CHANGELOG.md headings and links, and add the #29, #33 and #26 entries.
3. Run `fledge lanes run verify` and `cargo package --list --locked`.

## Risks

None beyond a reader mistaking the merge for a release; the change log heading carries
the date and the release link resolves only once the GitHub release exists.

## Constraints consulted

AGENTS.md: never claim unobserved release results; docs/releasing.md for the order of
release steps.
