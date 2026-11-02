# Lesson bundle: make-chough-timing-read-each-native-change-at-most-once-skip-archived-history-the-requested-report-does-not-show-bound

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Make chough timing read each native change at most once, skip archived history the requested report does not show, bound total native wait per report with explicit unknown states, and add an opt-in phase trace
- **Kind**: Feature
- **Specs**: cli
- **Paths**: src/timing.rs, src/records.rs, tests/timing.rs, tests/common, specs/cli, docs/cli.md, .chough/cases/cli.cases.toml, CHANGELOG.md
- **Acceptance**: For one timing report each distinct native change ID is passed to `specsync change status` at most once: with the checkpoint change also present in the archive, a counting stub records exactly one read for it and at most ten archived reads in total.
- **Acceptance**: Archived native history is read only when the report shows it (`--cycle all` with no `--stage`, `--case` or `--bug` filter). This is a deliberate change of output semantics: today `native` always carries history regardless of filters. Filtered reports emit `native.history` null with `native.history_selection` state `skipped-by-filter` naming the filters, and the text says so; unfiltered reports emit `selected` with read, reused, skipped and budget counts. Stage estimates come only from transitions and are identical either way.
- **Acceptance**: One whole-report native budget, CHOUGH_TIMING_NATIVE_BUDGET_SECONDS (default 120, whole seconds 1 to 3600, invalid values refused before any native read), caps total native wait in `chough timing` only. A read in flight gets at most the remaining budget; reads not started are reported not-read with reason budget-exhausted and unknown times. Slow, hung, failing and malformed stub responses stay timed-out, failed or invalid-output, and no descendant process outlives the command.
- **Acceptance**: With CHOUGH_TRACE=1 every bounded native read writes one stderr line with program, arguments, elapsed milliseconds and final state, and timing writes per-phase durations. Without it stdout and stderr are byte-identical to the untraced output, and the trace never changes a report, record or exit code.
- **Acceptance**: Stage moves, `chough next` and `chough status` keep their single authoritative native read under the existing per-read deadline, with no budget, filtering or reuse. Missing, timed-out, failed or invalid native evidence still refuses gated moves. This change does not make that single read faster: next and status stay bound by SpecSync's archived-status latency (one read measured at 49.954 s).
- **Acceptance**: Coverage extends the existing TC-cli-0089 and TC-cli-0100 (adding REQ-cli-024) plus named Rust regressions in tests/timing.rs; no new case number is allocated. Failure-first and post-fix observations use explicit IDs that carry the case, the purpose and the base revision. Before and after release-build measurements on the same inputs (cold and warm) cover focused next, full status, filtered and unfiltered timing and prune preview, with native call counts and completion or timeout state. cargo fmt, Clippy with warnings denied, full cargo test and `fledge lanes run verify` pass with CI-matching tools.

## Evidence

- Verification commit: `039e9c05c5a08e5b1b0d9e0f2e0ae5272c5c039c`
- Base commit: `b35acaf0804f84a2c56fcee4b3ca1504cf74c95e`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

Constraints consulted: AGENTS.md, INTENT.md, hi/life.md (LIFE-3, LIFE-6, LIFE-7,
LIFE-10), docs/FRD.md, specs/cli requirements REQ-cli-003, 020, 021, issue #32 and
its siblings #30 and #31, and the open draft PR #29 (Kimi; checkpoint merge; not
touched).

Measured on b35acaf in a temporary full-history clone, macOS arm64, SpecSync 6.1.2,
fresh debug and release builds (sha256 50ae8210… and 44e990d7…); raw results are kept
outside the repository with the handoff report. One sample each on a machine
running other agents' processes, so durations are indicative, not benchmarks.

- `specsync change status <archived checkpoint change> --json` alone completed in
  49.954 s (exit 0, 9,912 bytes). A shimmed run cut off at 25 s had logged one
  SpecSync start and 104 Git starts (history provenance: `git show <commit>:<archived
  record>` for every archived change, `rev-list`, `log --diff-filter=A`). Shim
  per-call durations include two interpreter start-ups each and are inflated.
  `specsync change status --json` with no ID returned in 0.049 s. The slowness is in
  SpecSync's archived-change provenance path; the internal root cause is not
  established.
- Native call counts per Chough command (logging shims): next 1, status 1, prune
  preview 1, timing 11 (checkpoint change plus ten archived, the checkpoint change
  read twice), cases check 0 SpecSync and 52 `hi`.
- With every native read capped at 1 s, release build: next 3.3–4.0 s, status
  3.2–3.4 s, timing (current cycle, test stage) 11.3–11.4 s, prune 1.1–1.5 s. Debug
  build: next 5.1–6.3 s, status 11.4–15.6 s (507 KB output), timing 11.5–11.8 s.
  Timing time is almost entirely its eleven sequential native reads; next and status
  have a 2–3 s local cost (release) not yet split by phase.
- At the default 120 s per-read deadline, timing's worst case is eleven reads × 120 s.
  If every archived read costs about what the one measured read cost, that is roughly
  nine minutes. This is an extrapolation, not a measurement.

## From the change's design.md

# Design

- Request-local memo in `src/timing.rs`: a map from change ID to the `NativeJson`
  result, used only inside `native_section`. Nothing is written to disk or shared
  between commands.
- History selection: `native_section` receives the parsed filters. The new JSON
  fields are `native.history_selection = {state: "selected"|"skipped-by-filter",
  filters: [...], read, reused, skipped_beyond_bound, not_read_budget}`. When the
  selection skips history, `native.history` is `null`. `skipped_archived` keeps its
  current meaning when history is selected and is `null` when it is not. Text output
  gains one line naming the selection.
- Budget: `records.rs` gains `native_json_within(root, program, args, cap)`, which
  runs the existing `native_capture` with `min(per-read deadline, cap)`. The existing
  `native_json` and `native` keep their signatures and behavior, so the stage, next,
  status, evidence and context call sites are untouched. Timing tracks remaining
  budget with a monotonic clock.
- Trace: one `trace()` helper in `records.rs`, checked through `CHOUGH_TRACE` once
  per process, writing to stderr only. It is called from `native_capture` and from
  the timing phases (transitions, tests, validation runs, native, render).
- Rejected alternatives: caching native responses across commands (stale
  approval/verification risk, invalidation scope); parallel native reads (SpecSync
  concurrency on one repository is unqualified and would multiply Git load); changing
  next/status/gates (their single read is already the minimum and must stay
  authoritative).

## From the change's testing.md

# Testing

This is the verification plan, not a record that these checks have run.

| Requirement | Evidence |
|---|---|
| REQ-cli-024, TC-cli-0089 (extended) | `tests/timing.rs`: a stub `specsync` on PATH logs each argv. Unfiltered report: one read of the checkpoint change, at most ten archived reads, identical current and history entries. Each of `--cycle current`, `--stage test`, `--case`, `--bug`: one read, `history` null, `history_selection` `skipped-by-filter` with the filter named, text line present, estimates identical to the unfiltered report. |
| REQ-cli-024, TC-cli-0100 (extended) | Stub modes sleep, hang (with a child holding the pipes), exit 1 and print malformed JSON, with `CHOUGH_TIMING_NATIVE_BUDGET_SECONDS=2`: bounded elapsed time, explicit states, `not-read`/`budget-exhausted` entries with unknown times, no surviving stub descendants, and invalid budget values refused with zero stub invocations. |
| REQ-cli-024, named regressions in tests/timing.rs | Trace on: one stderr line per read and per phase, stdout byte-identical. Trace off: both streams byte-identical. Stage move with a failing/timing-out stub and budget 1 is refused. `next` and `status` each log exactly one stub read. Records byte-identical after timing. |
| Real native | Release build against SpecSync 6.1.2 on a clone of the same base: completed versus timed-out reads reported as observed; no claim beyond the measured inputs. |

Environments: local-macos-arm64 observed; ci-linux and ci-windows only when observed.

No new case number is allocated. Each observation receives a unique ID conforming to the active identity grammar. Coordinate new writes with PR #33: once its OBS slug/random format applies, use that format or the current helper allocation rather than a legacy explicit-ID example. Preserve the actual IDs returned in the retained evidence.

## Where these lessons go

- `specs/cli/context.md`
