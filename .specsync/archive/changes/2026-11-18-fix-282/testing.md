---
change: fix-282
artifact: testing
---

# Testing

`tests/checkpoint.rs::checkpoint_pr_sets_and_clears_the_pull_request`, red on main because `--pr` is not a flag:
- `--pr 150` and then a pull request URL each set `pr`, read back by `chough status`.
- The Timeline gains no entry.
- An empty value clears the field.
- A line break and a home path are each refused with the session file byte-identical.

Case `TC-cli-checkpoint-pr-sets-the-pull-request` binds it.
