---
change: each-change-adds-a-changelog-fragment-that-the-release-task-assembles-into-changelog-md-290
artifact: plan
---

# Plan

## Approach

Replace shared edits of `CHANGELOG.md` with one fragment per change, assembled at release by a deterministic task, and tell agents to write the fragment.

## Out of scope

The index and feed on main (#289), SpecSync's merge driver (#292), and changing `.gitattributes`.

## Steps

1. Fragments, the assembler and the fledge task (Flint), with tests in `tests/changelog.rs`.
2. Convert main's Unreleased bullets once into fragments.
3. `AGENTS.md`, `docs/releasing.md`, and the workflow skill's fast path (`stages.md`, both copies) say to write a fragment; the old `stages.md` digest joins `KNOWN_PREVIOUS` and the release corpus pins the new bytes.

## Risks

- Every open PR still edits `CHANGELOG.md` until it is updated from main; after this merges, each one moves its bullets into a fragment when it is next updated.
- The ranked names of the converted fragments sort ahead only by their leading digits; they are removed at the next release.

## Constraints consulted

Leif's decision on #290 ("2: fragments"); LIFE-16, unchanged; the skill bundle rules (KNOWN_PREVIOUS, release corpus, SKILL.md size).
