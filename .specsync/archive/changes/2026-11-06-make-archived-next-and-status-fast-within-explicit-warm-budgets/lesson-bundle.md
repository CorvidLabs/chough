# Lesson bundle: make-archived-next-and-status-fast-within-explicit-warm-budgets

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Make archived next and status fast within explicit warm budgets
- **Kind**: Feature
- **Specs**: cli
- **Paths**: src/native.rs, src/main.rs, src/stage.rs, src/evidence.rs, tests/native.rs, tests/next.rs, tests/evidence.rs, tests/common/mod.rs, docs/cli.md, docs/FRD.md, hi/drive.md, CHANGELOG.md, specs/cli/cli.spec.md, specs/cli/testing.md
- **Acceptance**: On this macOS arm64 Chough checkout with 121 archives, a release build of next and status for a session naming an archived change each finishes at or below 2000 ms at p95 and 3000 ms for every run across 10 warm runs after one successful priming run. A valid warm archive entry makes zero SpecSync status calls; cold or invalid entries make one live bounded read and preserve observed, failed, timed-out and unknown states. Only a completed authenticated read primes an entry; a timed-out prime writes no entry, leaves no native descendants, and causes another bounded cold attempt rather than a warm claim. Every stage transition, timing report, retention decision and native verification keeps its live authoritative read and existing gate, output and timeout bounds. Cache invalidation covers archive bytes and Git identity, the resolved SpecSync binary and version, native policy/configuration/baseline, and shallow/replace/graft history inputs. Snapshot optimizations preserve byte-identical legacy and content fingerprints and observation freshness. Failing-first regressions, before/after real measurements and fledge lanes run verify are recorded.

## Evidence

- Verification commit: `18af4851adae0dc3daebebe2ff32beda2cc3fdbf`
- Base commit: `e99dc7961323e5c4e2aff1a2cf13333b0c47037e`
- Verified by: `specsync check --spec cli --spec history`

## From the change's context.md

# Context

Issue #32 / plan C8 reports 3-6 s `next` and `status` on Chough's own repository. DRIVE-2 already asks for archived changes to use committed files or a digest-keyed cache, at most one native status read per command, and a `next` response within a few seconds. Leif approved the numeric budget and narrow implementation, then re-approved the addition of exactly `specs/cli/cli.spec.md` and `specs/cli/testing.md` to `affected_paths`; REQ-cli-053 is materialized.

Profiling at `e99dc7961323e5c4e2aff1a2cf13333b0c47037e` on macOS arm64 with SpecSync 6.2.0 and 121 archives found:

- Direct archived status: 4.36-4.84 s warm, 261 Git children in an instrumented run.
- Direct active-draft status: 1.80-2.25 s, 18 Git children.
- No-active-change status: 0.006-0.020 s; Chough adds four Git calls and substantial local hashing.
- Fresh release `next`/`status` without an active change: 1.39-4.20 s across warm runs, dominated by Git enumeration and two snapshot fingerprint passes.

The full measurements and limitations are in `.chough/artifacts/issue-32-next-status-latency/PROFILE.md`.

PR #69 previously proposed authenticated archive reuse. It is stale/conflicting, reviewer evidence showed that shallow-history changes were absent from its key, and it reused verdicts for stage mutations. The coordinated split assigns its archive cache to this #32 change and batched `stage advance --to` to a separate small PR; #69 closes pointing to both replacements. This proposal retains SpecSync as the only authenticator, adds the missing history inputs, and limits reuse to read-only `next`/`status`. Every stage transition, timing report, retention decision and native verification stays live. No cached active-change status is allowed.

ch-pr69's independent SpecSync 6.2.0 measurements found the same warm native/snapshot split and an additional cold boundary: on Raven under load average 42-117, cold archived reads took 130.7-131.8 s, beyond Chough's 120 s native timeout, while warm native reads still took 11.4-11.5 s. Through Chough, two cold `next` runs timed out without SpecSync writing its read cache. This change therefore requires a successful prime, preserves the timeout and descendant kill, writes no entry on timeout, and never claims that a cold or killed read warmed the cache.

The ID registry audit covered origin/main `260ecc949ea01214f4bec43b30ea4226ce8fae59` and every remote branch. REQ-cli-051, REQ-history-011, FLOW-17 and DRIVE-14 were the highest respective claims at that point. ch-pr69 then claimed REQ-cli-052 for the split stage-advance PR, so this change takes REQ-cli-053 and proposes DRIVE-15; it claims no history or FLOW ID.

The checkpoint-cap owner retains `src/checkpoint.rs`, `src/sessions.rs`, `tests/cli.rs`, `tests/sessions.rs`, and session/capacity records. PR #133 also touches `src/evidence.rs`, `tests/next.rs`, `tests/common/mod.rs`, `docs/cli.md`, and `hi/drive.md`; merge main after it lands and preserve those edits. No rebase or force push is permitted.

Rejected: parsing archive files and deciding terminal validity in Chough, because that creates a second native authority. Rejected: weakening a gate, dropping an invalidation input, increasing a timeout, or caching active native state to meet the number. Cold archived status and active-change native latency are measured but are not given a new latency promise in this change.

## Closeout lessons

- Observation, source OBS-tc-cli-archived-report-reuse-20f0b488f31ec8fa7fbbff14: at `cfa2d104` on the 147-archive macOS arm64 repository, ten warm release runs per command after one authenticated prime gave next p95/max 0.66/0.66 s and status 0.67/0.67 s. Representative snapshot hashes were 247-258 ms and separate wrapper runs made zero warm SpecSync calls. This is limited to that host and input tree; Raven and CI Fledge 1.8.0 were not measured. Future changes to report, snapshot or native-input code must retain the full identity guards and rerun the real budget rather than infer it from unit tests.
- Verified, sources BUG-archived-reports-lack-bounded-authenticated-reuse-0536fdd42da0bab4c25202cf and BUG-specsync-symlink-cache-key-uses-unresolved-path-9205c0afb5fb77ec63d04630: preserved failing-first receipts preceded the cache and resolved-path corrections, and all five focused cases passed afterward. Cache identity must use the resolved SpecSync executable path and bytes, not the PATH symlink spelling.
- Observation, source `.chough/artifacts/issue-32-next-status-latency/verify-integrated.log`: after a native send-back with `canonical_applied: false`, audit refused already-approved paths until `specsync change check` materialized the delta. A later lane also became stale when its own new log under `.chough/` changed the verification digest. Future closeout should materialize first, keep required post-materialization steps as plain task text, run the lane without writing into the verified tree, and refresh native verification after all Chough writes.

## From the change's design.md

# Design

## Native archive verdict reuse

Add a crate-private `src/native.rs` component used by the report-only native read in `src/main.rs`.

1. On a live response, accept for storage only an archived response whose change ID matches and whose terminal validity is `authenticated-history`. Require a single self-contained archive and its native acceptance records. Strip any cache marker supplied by the live process.
2. Store the native JSON and proof metadata under the Git common directory, outside tracked inputs and outside `.chough` session/history records. Use create-only/atomic bounded writes and reject symlinks, malformed entries and oversized records.
3. Key reuse to the change ID, archive tree and complete file bytes, archive-carrying commit and relevant ancestry/object availability, resolved SpecSync executable bytes and recorded version, native policy/configuration/baseline bytes, and shallow/replace/graft/alternates state. Recompute the cheap identity checks on every report. A mismatch or inability to prove identity invokes the existing live bounded read. Only a completed authenticated live response can prime an entry. A timeout writes nothing, retains the existing descendant-kill behavior and is reported as unknown; the next invocation performs another bounded cold read rather than claiming the cache warmed.
4. Reuse only the exact native JSON previously supplied by SpecSync. Mark the Chough read state `cached`; never accept that marker from a live response. Report output keeps its existing size and field rules.
5. Do not call this component from `stage`, `timing`, retention or verification. Their existing `native_json` calls remain live, including after a warm report. Existing native timeouts and output bounds stay unchanged.

The cache is local trusted state like the Git directory, not human approval, a second lifecycle ledger, or proof that a later live check succeeded. Documentation will explain removal and cold fallback.

## Exact snapshot work reduction

Run the existing ordered legacy and content fingerprint passes as independent scoped workers. Each worker retains its current iteration order, `hash_input` transformations and digest bytes. Join both workers before constructing the snapshot; propagate failures visibly and preserve the existing Git status check. This changes no fingerprint algorithm, freshness predicate, persistent record or bound. Measure this implementation against the sequential baseline; do not replace it with an unbounded in-memory file cache or metadata-only reuse.

## Ownership

This change owns `src/native.rs`, the report call seam in `src/main.rs`, minimal live-path assertions around `src/stage.rs`, snapshot workers in `src/evidence.rs`, and focused native/next/evidence tests and docs. It does not edit checkpoint/session implementation or tests. Merge main after PR #133 and retain its overlapping evidence, next-test, common-fixture, documentation and HI changes.

## From the change's testing.md

# Testing

## Failing first

Use a counting SpecSync stand-in and real Git fixtures. Before product implementation, demonstrate failures for:

- first live archived authentication records a reusable result and the second `next`/`status` makes zero status calls;
- change ID, archive byte/tree/carrying history, SpecSync binary/version, native config/policy/baseline, shallow boundary, replace ref, graft and alternates mismatches force a live read;
- malformed, oversized, symlinked, spoofed, non-archived and non-authenticated entries never reuse;
- failed, timed-out and malformed live replies preserve existing unknown/failed behavior and write no authenticated entry;
- warm reports do not serve stage transitions, timing, retention or verification, whose status-call counts remain live;
- snapshot known answers and current/historical/mismatched observation labels remain byte-identical under the worker implementation.

Retain the failing commands and logs. Do not delete or weaken an existing assertion to make a test portable.

## Focused and regression checks

Run the new native/next/evidence tests, existing stage/native timeout and evidence freshness tests, and mutation checks for the cache marker, history key and live-gate call count. Use locked shell stubs where concurrent processes count calls, per the existing lesson. Run `fledge lanes run verify` at the final integrated head and record actual platform/tool versions.

## Real budget evidence

Build with `fledge run build -- --release`. On the same 121-archive macOS arm64 input tree, run one successful priming archived read and then ten warm runs each of `next --environment local` and `status --environment local`. Require p95 <= 2000 ms and every run <= 3000 ms for each command. Report all ten times, p95, maximum, binary/source identity, input counts, native call counts and `CHOUGH_TRACE=1` phases. A valid warm run makes zero SpecSync status calls. Also retain a separate cold live archived status measurement and its Git-call count; do not call that cold result warm or hide a timeout behind a zero exit. A controlled timeout test proves that a killed priming read writes no authenticated entry, leaves no native descendants, and causes the next invocation to attempt a fresh bounded live read. If the real cold read times out, report that it did not prime and do not proceed as though the warm budget were demonstrated.

No wall-clock unit test replaces this real benchmark, and a structural SpecSync pass is not product acceptance. If the budget fails, record the failure and return to build or the human scope gate; never increase the approved numbers in implementation.

## Where these lessons go

- `specs/cli/context.md`
