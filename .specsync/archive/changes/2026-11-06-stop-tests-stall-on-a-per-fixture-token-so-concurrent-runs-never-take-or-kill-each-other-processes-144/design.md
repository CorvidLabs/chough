---
change: stop-tests-stall-on-a-per-fixture-token-so-concurrent-runs-never-take-or-kill-each-other-processes-144
artifact: design
---

# Design

## Approach

Each scenario computes `let token = identity(<n>)` and uses `sleep {token}` in its script, its `still_running` pattern and its `Reap`. `identity` makes the token unique by process ID, a counter and nanoseconds. `stall_git` and `hold_git` take the token, not a `u32`.

## Risks

None to the product: this changes only how tests isolate themselves.

## Constraints consulted

The #95 review's `identity()`. The rule against changing assertions or timeouts.
