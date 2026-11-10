# Lesson bundle: make-native-process-test-probes-zombie-aware-for-187-and-preserve-the-132-reproduction-limit

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Make native process test probes zombie-aware for #187 and preserve the #132 reproduction limit
- **Kind**: BugFix
- **Specs**: cli, reporting
- **Paths**: tests/common/mod.rs, tests/native.rs, tests/change.rs, tests/timing.rs, .chough/cases/cli.native-process-liveness.cases.toml, .chough/cases/reporting.native-process-liveness.cases.toml
- **Acceptance**: A new regression first fails when the shared probe uses only kill -0 on a killed, unreaped child. After the fix, the bounded common::alive probe reports that zombie as not alive, still reports a live sleep 60 child as alive, and is used by tests/timing.rs, tests/native.rs and tests/change.rs. Focused regressions and stressed repetitions pass on macOS and Linux, and fledge run linux-verify passes. src/checkpoint.rs and its lock test stay unchanged: #132 remains open with the handoff's 0/40 Linux rounds and 0/30 loaded rounds, with no speculative diagnostics or fix. No product behavior, HI wording or requirement text changes.

## Evidence

- Verification commit: `0f7144b77861974367a47c9dd1951eb7e87d1649`
- Base commit: `86ae8dda94aba981094e92d5395dc085ebe65dbd`
- Verified by: `specsync check --spec cli --spec reporting`

## From the change's context.md

# Context

Issue #187 reproduced twice in 25 loaded macOS runs of `tests/timing.rs::a_timed_out_or_unconfigured_native_read_leaves_native_times_unknown`; isolated runs passed. Instrumentation found that `kill -0` succeeded while the immediately following `ps` found no process. The stub's `sleep` descendant had received the process-group SIGKILL and was a killed, not-yet-reaped zombie, not a survivor. The temporary instrumentation was reverted and no product code changed.

Issue #132's ETXTBSY half was fixed by #145. Its remaining checkpoint-lock failure is unconfirmed: the handoff records 0 failures in 40 Linux rounds of `cargo test --bin chough -- --test-threads=32` and 0 in 30 loaded rounds. This change leaves `src/checkpoint.rs` and that test unchanged and leaves #132 open with those limits rather than adding speculative retries or diagnostics.

Leif's existing DRIVE-19 covers this test-reliability work. The lane handoff explicitly assigns #187 and #132 as one theme group. Only the orc records human approvals, reviews and gate decisions.

## From the change's design.md

# Design

Add `common::alive(pid: &str) -> bool` for Unix integration tests. Begin with the existing `kill -0` behavior and a regression that spawns `sleep 60`, confirms the running child is detected, SIGKILLs it without waiting, and asserts that the killed, unreaped child is not alive. Reap the child before asserting so the failing-first run leaves no zombie behind.

Then bound the helper to one second with 20 ms polls. Each poll returns false when `kill -0` fails, when `ps -o stat= -p <pid>` returns no process, or when the state starts with `Z`. A process still in a non-zombie state after the deadline returns true. This preserves detection of a real survivor while allowing a killed process to exit or be reaped.

Use the helper in the three existing probes: `surviving_children` in `tests/timing.rs`, the timed-out native process check in `tests/native.rs`, and the SIGTERM check in `tests/change.rs`. Do not change `src/records.rs`, `stop_tree`, checkpoint locking, assertions' intended behavior, or any public interface.

## From the change's testing.md

# Testing

Failing first: run the new native integration regression against a helper that only calls `kill -0`. The killed, unreaped child must produce a failed assertion, with the child reaped before the assertion. Record that commit, observation and a local bug before implementing the fix.

Focused passing checks: the new zombie/live-child regression; `a_timed_out_prime_writes_nothing_kills_children_and_retries_cold`; `sigterm_to_change_check_stops_specsync`; and `a_timed_out_or_unconfigured_native_read_leaves_native_times_unknown`. Add one cli case for the new helper and affected cli probes, and one reporting case for the original timing probe.

Stress: repeat the original timing regression 25 times under CPU load on macOS and 30 times under CPU load on Linux. Keep all results, including any failure; do not report only a final pass. Run `fledge run linux-verify` on the implementation tip and hosted archive-first CI on the archived tip.

#132 remains a stated omission: no new run is substituted for the handoff's 0/40 ordinary and 0/30 loaded Linux rounds, no cause is claimed, `src/checkpoint.rs` is untouched, and the issue stays open. Any new diagnostics require a later orc/Leif decision.

## Where these lessons go

- `specs/cli/cli.3md`
- `specs/reporting/context.md`
