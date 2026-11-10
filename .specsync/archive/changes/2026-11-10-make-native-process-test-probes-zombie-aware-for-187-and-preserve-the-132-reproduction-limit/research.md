---
change: make-native-process-test-probes-zombie-aware-for-187-and-preserve-the-132-reproduction-limit
artifact: research
---

# Research

The handoff's loaded macOS reproduction failed 2 of 25 runs; Linux failed 0 of 30 under load and the test passed 5 of 5 alone. In both macOS failures, `kill -0` reported the stub's `sleep 60` grandchild present but `ps -o stat=,ppid=,comm= -p <pid>` immediately afterwards printed nothing. `stop_tree` SIGKILLs the whole stub process group, including that descendant. On macOS, launchd reaps the orphan later and `kill -0` succeeds for a zombie. The evidence therefore rejects the surviving-child/product-kill theory.

The current probes are `tests/timing.rs:146`, `tests/native.rs:306` and `tests/change.rs:352`. All rely on `kill -0`; none checks process state. A failing-first regression can create the exact condition deterministically by killing a child and not calling `wait` until after the probe.

For #132, the issue comment hypothesizes inherited lock descriptors but has no stderr or reproduction proving it. The handoff's 0/40 ordinary and 0/30 loaded Linux rounds do not justify changing lock creation or adding a bounded retry. Capturing more diagnostics is a possible later human decision, not part of this scope.
