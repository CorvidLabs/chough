---
id: integrate-merged-main-into-pr-46-while-preserving-report-reads-and-lifecycle-guidance
state: implementing
type: operations
base_commit: 7b3d4fcab3b64088a1268eb11c5823cf06ac98f3
---

# Integrate merged main into PR 46 while preserving report reads and lifecycle guidance

## Intent

Integrate merged main into PR 46 while preserving report reads and lifecycle guidance

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- Merged main retains case-order guidance and unlinked-failure behavior together with request-local report reads and typed error chains; all immutable proof bytes and additive checkpoint collections and transition occurrences survive; canonical version and changelog are monotonic; mandatory lane and Rust 1.88 tests and the three report cases pass; native verification is current before sole-writer closeout

## No-spec Rationale

Reconcile already approved and archived PR 46 report reads with merged main case-order and unlinked-failure behavior. Preserve all semantic requirements and source behavior; only integration, canonical version/changelog reconciliation and qualification records are new.
