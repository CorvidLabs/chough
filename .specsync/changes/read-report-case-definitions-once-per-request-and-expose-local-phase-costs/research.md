---
change: read-report-case-definitions-once-per-request-and-expose-local-phase-costs
artifact: research
---

# Research and observed baseline

The fresh main debug binary has SHA-256
b93d3d0060c0986a99c0f177cf0b5b9b5e49277d0d3fdc91fd2fe95f61ff6680.
Raw diagnostics are in /tmp/chough-ticket32-profile and will be retained as new
Chough artifacts with original metadata. First-process means a new process, not a
purged operating-system cache. No benchmark ran concurrently with audit test builds.

| Command | First process s | Warm s | Native outcome |
|---|---:|---:|---|
| next | 12.347 | 12.023 | one read timed out at 10 s |
| status | 14.978 | 14.936 | one read timed out at 10 s |
| filtered timing | 10.069 | 10.101 | one read timed out at 10 s |
| full timing | 30.142 | 30.098 | three reads started, no completion, 30 s budget |
| retention preview | 10.106 | 10.103 | one read timed out at 10 s |

The raw recorder's native_seconds field is the sum of completed calls. With zero
completed calls its value is an empty sum, not zero native cost. Corrected summaries
must use unknown/null for completed-native cost and retain explicit timeout traces.
The direct unwrapped archived-status command completed in 22.912 seconds, exit zero,
and reported archived authenticated-history. That is separate successful native
integration; the capped Chough reports did not obtain it.

Source pointers: evidence::freshness invokes cases::current_digest per binding;
current_digest structurally reloads the suite. Status checks all observations and
then referenced observations. Cases are bounded but repeated parsing is avoidable.
Existing phase tracing distinguishes timing's major sections and native reads, but
not next/status local Git/hash/case/artifact work. Request-local reuse is the smallest
option without persistent cache invalidation or relaxed gates.

## Implemented comparison

The exact paired results, unchanged-input proof, binary identities and limitations are retained in .chough/artifacts/report-reads-comparison.md and the referenced JSON/tar artifacts. With identical 1-second native / 3-second timing caps, status changed from 7.111 to 2.780 seconds first-process and 6.285 to 2.187 repeated; next from 3.889 to 2.912 and 3.096 to 1.851. New observed suite-load counters are one load for 432 status lookups and 138 next lookups. Every native attempt remained timed out, so this is local-work evidence and no native speedup or successful native qualification is claimed.

The final typed-error-preserving binary was measured again on a new unchanged main fixture. See .chough/artifacts/report-reads-final-comparison.md and its immutable raw bundle for the authoritative final paired figures and binary/patch identities. Earlier paired results are preserved, rather than relabeled as final-code measurements.
