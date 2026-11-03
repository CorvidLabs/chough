---
change: identity-acceptance-evidence-for-issue-30-a-pinned-clock-same-second-run-in-two-git-roots-and-a-real-git-integration
artifact: testing
---

# Testing

- `same_second_runs_in_two_git_roots_keep_distinct_identities_after_merge` (unit, bin target) runs on every CI host and needs only `git`.
- `independently_added_records_merge_without_conflict_or_replacement` (integration, `tests/cases.rs`) uses the built binary, a `hi` stand-in and real `git`.
- Cases `TC-cli-same-second-runs-in-two-git-roots-stay-distinct` and `TC-cli-independently-added-records-merge-cleanly` link REQ-cli-023.
- These are acceptance tests for behavior already on main. Each assertion is checked against a deliberately broken variant: the test must fail if the IDs or paths collide, or if a record is replaced.
