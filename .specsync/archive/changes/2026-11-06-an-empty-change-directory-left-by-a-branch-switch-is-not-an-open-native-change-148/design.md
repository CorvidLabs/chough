---
change: an-empty-change-directory-left-by-a-branch-switch-is-not-an-open-native-change-148
artifact: design
---

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
