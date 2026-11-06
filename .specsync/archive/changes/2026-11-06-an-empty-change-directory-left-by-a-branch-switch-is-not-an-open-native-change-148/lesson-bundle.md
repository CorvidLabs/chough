# Lesson bundle: an-empty-change-directory-left-by-a-branch-switch-is-not-an-open-native-change-148

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: An empty change directory left by a branch switch is not an open native change (148)
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: src/evidence.rs, src/library.rs, tests/evidence.rs, CHANGELOG.md
- **Acceptance**: A directory under .specsync/changes with no file at any depth is not an open change: the knowledge index does not list it and the trim does not refuse on it, while a directory with a change file still counts as open; the new test fails first and fledge lanes run verify passes on Linux.

## Evidence

- Verification commit: `fcf014e842776c77ff67c31a5257c38bd3d012ab`
- Base commit: `64aa77e01d2763395d8af7566e62743ede37cc32`
- Verified by: `specsync check --spec cli --spec history`

## From the change's context.md

# Context

Issue #148, filed by ch-c456 during the C5 closeout (#137, PR #146).

A branch switch removes a change's tracked files but leaves its empty directories, because Git does not track directories. Chough reads every directory under `.specsync/changes/` as an open change, in two places:
- the knowledge index's Active list (`library.rs`, `sorted_entries`);
- the trim's open-change checks (`evidence.rs`, `change_dirs`).

So the index written in that worktree differs from a fresh clone, and `library index --check` fails in CI. The trim may also refuse with a false "a native change is open".

The new test leaves `.specsync/changes/left-behind/deltas/` empty. It reproduces the failure: the index lists `left-behind` as Active.

## From the change's design.md

# Design

## Approach

A shared helper `has_change_files(path)` walks the directory, bounded, and is true at the first regular file. Two places use it:
- `evidence::change_dirs` for `.specsync/changes`. Archive listings are unchanged.
- `library.rs`, for the Active list.

Symlinks are still refused, as before.

## Risks

A truly open change always has `change.md` and `state.json`, so it is never skipped.

## Constraints consulted

REQ-cli-043, KNOW-1, issue #148.

## From the change's testing.md

# Testing

tests/evidence.rs `an_empty_change_directory_left_by_a_branch_switch_is_not_an_open_change` checks four things:
- with an empty `left-behind/deltas/`, `library index` does not list it;
- `library archive --apply` does not refuse on it or name it;
- a directory with `change.md` still refuses and is named;
- the existing open-change refusal test still passes.

It fails first: the index lists `left-behind` as Active.

`fledge lanes run verify` passes on Linux.

## Where these lessons go

- `specs/cli/context.md`
