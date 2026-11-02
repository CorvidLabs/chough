---
change: read-report-case-definitions-once-per-request-and-expose-local-phase-costs
artifact: plan
---

# Plan

## Approach

Keep all reuse inside one root-bound request and preserve native authority. Codex owns this branch; other agents retain their declared separate scopes.

## Steps

1. Retain the baseline metadata, raw outputs and corrected interpretation as new
   artifacts, preserving the clean main revision and freshly built binary identity.
2. Add the root-bound lazy case index and thread it through report freshness and
   label readers. Preserve producer validation and public output shapes.
3. Add opt-in local phases/counters and portable regressions for exact read counts,
   no-bound-case laziness, malformed input and next-request invalidation.
4. Compare old and new binaries on identical frozen main inputs. Report first
   observed and repeated runs, native wait and timeouts separately. Do not promise
   a speedup until observed.
5. Update docs/FRD/architecture/roadmap/changelog and executable case definitions.
6. Finish implementation/regression tasks, materialize REQ-cli-028, then run linked
   cases, the full Fledge lane and Rust 1.88 qualification. Commit source and Chough
   evidence, run native committed verification, push one focused PR and observe CI.

## Ownership

Codex owns this branch's declared Rust/report/test/doc paths. Parent
session owns the separate workflow-authority change. Grok owns PR #41's skill
ordering work. PR #37's history/retention paths and recorded proof are preserved.

## Relative estimate

Small-to-medium internal refactor. Main uncertainties are preserving
all failure precedence and making measurements comparable despite dominant native
wait. Timing history is not a reliable effort estimate: native-capped reports are
partial and elapsed lifecycle samples are not labor. No deadline is inferred.

## Out of scope

Persistent caching, parallel native authentication, source hashing
redesign, new runtime/service, record schemas, retention, identity, lifecycle policy,
manual native operation by the human, native acceptance/ship, merge or release.

## Risks

A shared index can accidentally survive a request or change failure precedence. Bind it to the request root, keep invalid definitions visibly invalid, and exercise later-request mutation fixtures. Native latency can dominate an otherwise useful local reduction, so report that remaining cost without a total responsiveness guarantee.

## Constraints consulted

AGENTS.md, INTENT.md, LIFE/FLOW criteria, FRD, canonical cli contract and companions, specs/cli/context.md lessons, source-level read-path audit, raw clean-main diagnostics and the explicit standing human authorization.
