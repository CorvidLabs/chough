---
id: make-archived-next-and-status-fast-within-explicit-warm-budgets
state: archived
type: feature
base_commit: e99dc7961323e5c4e2aff1a2cf13333b0c47037e
---

# Make archived next and status fast within explicit warm budgets

## Intent

Make archived next and status fast within explicit warm budgets

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- On this macOS arm64 Chough checkout with 121 archives, a release build of next and status for a session naming an archived change each finishes at or below 2000 ms at p95 and 3000 ms for every run across 10 warm runs after one successful priming run. A valid warm archive entry makes zero SpecSync status calls; cold or invalid entries make one live bounded read and preserve observed, failed, timed-out and unknown states. Only a completed authenticated read primes an entry; a timed-out prime writes no entry, leaves no native descendants, and causes another bounded cold attempt rather than a warm claim. Every stage transition, timing report, retention decision and native verification keeps its live authoritative read and existing gate, output and timeout bounds. Cache invalidation covers archive bytes and Git identity, the resolved SpecSync binary and version, native policy/configuration/baseline, and shallow/replace/graft history inputs. Snapshot optimizations preserve byte-identical legacy and content fingerprints and observation freshness. Failing-first regressions, before/after real measurements and fledge lanes run verify are recorded.

## No-spec Rationale

Not applicable
