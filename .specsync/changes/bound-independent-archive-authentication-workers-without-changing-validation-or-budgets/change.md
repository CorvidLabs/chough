---
id: bound-independent-archive-authentication-workers-without-changing-validation-or-budgets
state: implementing
type: bug_fix
base_commit: 9af1921a3bb1f7092d364a18489cb3028e97ff26
---

# Bound independent archive authentication workers without changing validation or budgets

## Intent

Bound independent archive authentication workers without changing validation or budgets

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- Every historical roster snapshot and archive obligation is retained. Independent current-root archive status reads use a fixed maximum of two workers. Every result must match its requested ID, archived state and authenticated-history validity; no success until all finish. Native errors, invalid archives and command deadlines fail closed. SpecSync 6.1.2, 90-second subprocess timeout, output bounds, read-only fingerprints and the 10-minute hosted job budget stay unchanged. Existing native fixtures plus concurrency/failure/drain tests pass. Actual hosted duration is observed before merge.

## No-spec Rationale

CI execution scheduling only. Every native history inventory and authenticated-history obligation, exact SpecSync version, before/after read-only guard, 90-second command timeout and 10-minute job budget remain unchanged. No runtime contract or evidence rewrite.
