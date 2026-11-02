---
change: read-report-case-definitions-once-per-request-and-expose-local-phase-costs
artifact: context
---

# Context

Ticket #32 asks for measured reporting work without weakening native gates. Main
825a380 is the baseline. PR #34 already bounded and deduplicated native timing
reads. PR #37 owns immutable history/retention and reserves REQ-cli-025 to 027;
this independent change reserves REQ-cli-028. Grok's PR #41 owns the case-ordering
skill fix. This branch changes neither skill policy nor those storage contracts.

A freshly built debug main binary was profiled on macOS arm64 with Rust 1.95.0,
HI 0.8.0, Fledge 1.8.0 and SpecSync 6.1.2. First-process/warm wall times were
12.35/12.02 seconds for next and 14.98/14.94 for status, with 10-second native
read deadlines. Those native reads timed out; exit zero was not native success.
Filtered timing took about 10.1 seconds; full timing exhausted its 30-second
native budget. An unwrapped native archived-status request completed in 22.912
seconds and reported authenticated history. OS caches were not evicted.

The source graph predicts repeated case parsing: every bound-case freshness check
calls current_digest, which loads all definitions. The baseline has 323
observations and 294 case bindings, plus 138 bindings in checkpoint/bug references.
The predicted minimum for status is 432 suite loads and for next 138. These are
source-derived counts, not instrumented file-open measurements. The implementation
will expose exact request-local load and lookup counts and preserve separate native
wait evidence.

Human authorization: on 2026-11-02 0xLeif explicitly said, "And I already gave it
some approval and told it can do any changes. It just needs a PR." He directed
Chough to operate the native tools internally. This applies to this concrete
implementation scope and its native scope approval, with the human named as actor.
It does not substitute for implementation acceptance, native review/ship or merge.
Source: current chat and the relay on issue #11, comment 6581702299.

Constraints: one ephemeral read context per request; no persistent cache or cached
native authorization; unchanged case digests, freshness precedence and report
schemas; unchanged artifact, environment and source validation; no live adoption,
cleanup, archive or cold-start grade. The baseline diagnostics preceded this
change directory and must retain their original revision and binary identity.

Lessons consulted: specs/cli/context.md records requirement materialization before
linked cases, complete implementation tasks before native check, and committing
source plus Chough records before final native verification. Post-materialization
case executions and human gates are not task checkboxes.

## Implementation lesson

Observation: the first trace-parity regression joined diagnostic lines and thereby dropped a final newline before comparing them. The assertion failed while the other three read-context regressions passed. Source: OBS-product-test-72b596ad74a5bf0901e6aa92 and .chough/artifacts/report-reads-trace-test-before.log. Future use: remove trace lines with a byte-preserving split-inclusive filter when checking exact stderr parity; do not normalize away error changes. Limits: this was a new test-harness comparison failure, not a native lifecycle defect.

Verified follow-up: all five focused report regressions pass after the diagnostic filter was changed to preserve exact newline bytes. Source: .chough/artifacts/report-reads-focused-after.log. The paired debug comparison on frozen main inputs is retained in .chough/artifacts/report-reads-comparison.md and its raw bundles; it shows local next/status reductions while every capped native read remains timed out.

Final review caught a separate cache compatibility defect after the initial full lane: converting loader errors to strings flattened the existing cause chain. OBS-product-test-553ec9b0ffe80fd46a96145d and BUG-report-case-cache-flattens-existing-loader-error-chains-ba1e469eeb3970b370b6456b retain the failure. The strengthened regression failed before the fix and OBS-product-test-4e2d19e426e48d18085f0b50 records the later pass, including exact original-loader error parity. The cache now preserves typed shared errors. The final-binary comparison is .chough/artifacts/report-reads-final-comparison.md; earlier measurements and full201-test logs remain intermediate historical evidence.
