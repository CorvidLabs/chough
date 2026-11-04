---
id: scale-independent-archive-authentication-workers-with-the-runner-s-cpus-so-the-archive-check-fits-its-unchanged-budget
state: archived
type: bug_fix
base_commit: 1ac2ae228f12201fac910bce3a61837761580a89
---

# Scale independent archive authentication workers with the runner's CPUs so the Archive check fits its unchanged budget

## Intent

Scale independent archive authentication workers with the runner's CPUs so the Archive check fits its unchanged budget

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- Independent current-root archive status reads use one worker per CPU that os.cpu_count reports with at least two and at most eight workers; Every historical roster snapshot and archive obligation is still read and every result must still match its requested ID and archived state and authenticated-history validity with no success until all reads finish; Native errors and invalid archives and command deadlines still fail closed; SpecSync 6.1.2 and the 90-second subprocess timeout and the output bounds and the read-only fingerprints and the 10-minute hosted job budget stay unchanged; The worker-count tests fail first and the existing native fixtures and concurrency and failure and drain tests pass; Local wall time of the check on the #71 head is measured before and after and the hosted Archive job on this PR passes within its budget; DRIVE-10 records the t813u speed direction in hi/drive.md and hi check passes

## No-spec Rationale

CI execution scheduling only. Every historical roster snapshot and archive obligation and every authenticated-history check stay; the exact SpecSync version and the before and after read-only fingerprints and the 90-second command timeout and the 10-minute job budget are unchanged. No runtime contract or evidence rewrite.
