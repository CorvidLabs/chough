---
change: release-prep-0-6-4
artifact: plan
---

# Plan

## Approach

Mechanical release preparation per docs/RELEASING.md, same shape as the 0.6.1/0.6.2/0.6.3
release-prep changes: bump the version, assemble the changelog, rebuild generated
knowledge, prove the checks green.

## Out of scope

The human release steps (tag v0.6.4, cargo publish, GitHub release, clean-install
observation, roadmap row) — LIFE-25 reserves those for Leif's one-line gos after merge.

## Steps

1. Bump Cargo.toml version to 0.6.4; cargo build --locked refreshes Cargo.lock.
2. Assemble the changelog with fledge run changelog -- --release 0.6.4 --date 2026-11-26; verify the [0.6.4] heading, every fragment folded in, and the tag link.
3. Rebuild generated knowledge: chough library index.
4. Run fledge run release-check until it exits 0.
5. cargo test --locked at 0.6.4.
6. Commit, push release-prep-0-6-4, open the PR to main.

## Risks

- The stray 2026-11-22-docs-0-6-3-release.md fragment may predate 0.6.3's assembly;
  the changelog tool refuses if [0.6.3] already covers it — then it is handled as a
  0.6.4 docs entry or removed with a note in the change record, never silently.
- Release-check requires a clean tree; commits happen before the check.

## Constraints consulted

LIFE-25 (weekly 0.x release, agent preps, Leif's one-line go); docs/RELEASING.md
steps 1-4 are the agent-side portion.
