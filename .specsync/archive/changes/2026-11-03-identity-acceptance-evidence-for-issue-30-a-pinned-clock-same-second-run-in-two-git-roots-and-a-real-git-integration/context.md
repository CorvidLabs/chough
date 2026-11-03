---
change: identity-acceptance-evidence-for-issue-30-a-pinned-clock-same-second-run-in-two-git-roots-and-a-real-git-integration
artifact: context
---

# Context

Issue #30 still has unchecked acceptance evidence after PR #33 (slug case IDs and random OBS/VAL/SNAP IDs) merged. The two items picked up here are:

- the deterministic same-second two-root test;
- the real Git fixture.

`records::now()` cannot be injected (`tests/common` says so), and the Product CI jobs do not install `hi`. So the same-second test runs inside the crate, with a test-only clock pin and a test-only `hi` export stand-in. Neither exists outside `cfg(test)`, so no runtime switch can forge observation times.

Plan comment: https://github.com/CorvidLabs/chough/issues/30#issuecomment-6593220605. Criteria 2, 4, 5 and 6 of the issue are out of scope here.
