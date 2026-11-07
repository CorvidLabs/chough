---
change: make-archived-next-and-status-fast-within-explicit-warm-budgets
artifact: testing
---

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
