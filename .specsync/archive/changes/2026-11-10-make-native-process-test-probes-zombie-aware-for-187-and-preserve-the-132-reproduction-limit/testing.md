---
change: make-native-process-test-probes-zombie-aware-for-187-and-preserve-the-132-reproduction-limit
artifact: testing
---

# Testing

Failing first: run the new native integration regression against a helper that only calls `kill -0`. The killed, unreaped child must produce a failed assertion, with the child reaped before the assertion. Record that commit, observation and a local bug before implementing the fix.

Focused passing checks: the new zombie/live-child regression; `a_timed_out_prime_writes_nothing_kills_children_and_retries_cold`; `sigterm_to_change_check_stops_specsync`; and `a_timed_out_or_unconfigured_native_read_leaves_native_times_unknown`. Add one cli case for the new helper and affected cli probes, and one reporting case for the original timing probe.

Stress: repeat the original timing regression 25 times under CPU load on macOS and 30 times under CPU load on Linux. Keep all results, including any failure; do not report only a final pass. Run `fledge run linux-verify` on the implementation tip and hosted archive-first CI on the archived tip.

#132 remains a stated omission: no new run is substituted for the handoff's 0/40 ordinary and 0/30 loaded Linux rounds, no cause is claimed, `src/checkpoint.rs` is untouched, and the issue stays open. Any new diagnostics require a later orc/Leif decision.
