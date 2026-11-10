---
change: make-native-process-test-probes-zombie-aware-for-187-and-preserve-the-132-reproduction-limit
artifact: context
---

# Context

Issue #187 reproduced twice in 25 loaded macOS runs of `tests/timing.rs::a_timed_out_or_unconfigured_native_read_leaves_native_times_unknown`; isolated runs passed. Instrumentation found that `kill -0` succeeded while the immediately following `ps` found no process. The stub's `sleep` descendant had received the process-group SIGKILL and was a killed, not-yet-reaped zombie, not a survivor. The temporary instrumentation was reverted and no product code changed.

Issue #132's ETXTBSY half was fixed by #145. Its remaining checkpoint-lock failure is unconfirmed: the handoff records 0 failures in 40 Linux rounds of `cargo test --bin chough -- --test-threads=32` and 0 in 30 loaded rounds. This change leaves `src/checkpoint.rs` and that test unchanged and leaves #132 open with those limits rather than adding speculative retries or diagnostics.

Leif's existing DRIVE-19 covers this test-reliability work. The lane handoff explicitly assigns #187 and #132 as one theme group. Only the orc records human approvals, reviews and gate decisions.
