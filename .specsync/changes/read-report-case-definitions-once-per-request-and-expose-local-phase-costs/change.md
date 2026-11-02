---
id: read-report-case-definitions-once-per-request-and-expose-local-phase-costs
state: implementing
type: refactor
base_commit: 825a3805530456005e361dfa6d96d93eee52cd09
---

# Read report case definitions once per request and expose local phase costs

## Intent

Read report case definitions once per request and expose local phase costs

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- One request reads and validates the case suite at most once while preserving legacy/current case digests, report output, artifact and environment checks, native read counts and genuine lifecycle refusal; a later request observes changed source/case/artifact/native inputs and never reuses stale results; opt-in local phase traces preserve stdout and exit behavior; before/after first-process and repeated measurements record the exact revision, freshly built binary, input sizes, toolchain and censored versus completed native work with no unobserved speedup claim.

## No-spec Rationale

Not applicable
