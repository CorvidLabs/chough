---
change: release-0-6-1
artifact: plan
---

# Plan

## Approach

Follow `docs/releasing.md`. Set `version = "0.6.1"` in `Cargo.toml` and let Cargo
update `Cargo.lock`. Assemble the fragments in `changelog.d/` into `CHANGELOG.md`
with `fledge run changelog -- --release 0.6.1 --date <date>`, only for changes
merged to main, on the planning session's word of what is in. Rebuild the
knowledge index with `fledge run release-check` and commit what it changes.

## Out of scope

The tag, `cargo publish`, the GitHub release, the clean-install observation and
the roadmap row. No code, spec or skill change. The one test change is to two tests in
`tests/changelog.rs` that read the live fragments in `changelog.d/` and so fail once a
release assembles them: one also reads the released section, the other reads fixture
copies under `tests/fixtures/changelog/`. No fragment of an unmerged
pull request.

## Steps

1. Bump the version and commit.
2. Run `cargo package --list --locked` and `cargo publish --dry-run --locked`,
   and check the crates.io owner.
3. Merge main, assemble the change log and run the release check.
4. Run the verify lane, open the pull request, and stop for review.

## Risks

A fix that merges after the change log is assembled would be missing from the
0.6.1 section; the branch is updated against main right before it merges. A
dry run is a rehearsal and proves no publication.

## Constraints consulted

`docs/releasing.md`, `skills/chough-workflow/references/release.md`, AGENTS.md
(fragments go in `changelog.d/`; the index is rebuilt at release), LIFE-16 and
LIFE-25.
