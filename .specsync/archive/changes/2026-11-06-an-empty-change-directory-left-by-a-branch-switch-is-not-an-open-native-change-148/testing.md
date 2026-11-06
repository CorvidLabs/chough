---
change: an-empty-change-directory-left-by-a-branch-switch-is-not-an-open-native-change-148
artifact: testing
---

# Testing

tests/evidence.rs `an_empty_change_directory_left_by_a_branch_switch_is_not_an_open_change` checks four things:
- with an empty `left-behind/deltas/`, `library index` does not list it;
- `library archive --apply` does not refuse on it or name it;
- a directory with `change.md` still refuses and is named;
- the existing open-change refusal test still passes.

It fails first: the index lists `left-behind` as Active.

`fledge lanes run verify` passes on Linux.
