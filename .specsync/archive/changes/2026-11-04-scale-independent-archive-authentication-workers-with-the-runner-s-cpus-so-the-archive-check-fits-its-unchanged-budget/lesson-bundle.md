# Lesson bundle: scale-independent-archive-authentication-workers-with-the-runner-s-cpus-so-the-archive-check-fits-its-unchanged-budget

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Scale independent archive authentication workers with the runner's CPUs so the Archive check fits its unchanged budget
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: .github/scripts/check-specsync-archive.py, .github/scripts/test-specsync-archive.py, hi/drive.md, INTENT.md, CHANGELOG.md
- **Acceptance**: Independent current-root archive status reads use one worker per CPU that os.cpu_count reports with at least two and at most eight workers; Every historical roster snapshot and archive obligation is still read and every result must still match its requested ID and archived state and authenticated-history validity with no success until all reads finish; Native errors and invalid archives and command deadlines still fail closed; SpecSync 6.1.2 and the 90-second subprocess timeout and the output bounds and the read-only fingerprints and the 10-minute hosted job budget stay unchanged; The worker-count tests fail first and the existing native fixtures and concurrency and failure and drain tests pass; Local wall time of the check on the #71 head is measured before and after and the hosted Archive job on this PR passes within its budget; DRIVE-10 records the t813u speed direction in hi/drive.md and hi check passes

## Evidence

- Verification commit: `ee7318db0c67a3af4d6d0ae89d2ce7ac5794384d`
- Base commit: `1ac2ae228f12201fac910bce3a61837761580a89`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

## Human authority and limits

Acting agent: corvid-agent. Intent: Leif in chat, 2026-11-03 6:30 PM MST (t813u), "I've had reports that sometimes SpecSync archive takes like three minutes to run" and "make sure that the whole suite runs fast and is good", recorded as DRIVE-10. The parent agent relayed the decision for this PR at 9:31 PM MST: make the check itself faster without weakening it, and raise no timeout. This does not authorize lowering a gate, a longer timeout, rewriting recorded proof, release or deployment. Closeout and merge need tests, independent review and green exact-head checks.

## Problem

The hosted SpecSync Archive job on #71 (head f646bc5, 63 archives) was cancelled at its unchanged 10-minute limit twice. On main, #68's run took 8 min 39 s. Locally the same script passes on the #71 head in 9 min 56 s with two workers. Each native 6.1.2 `change status <archived id>` takes about 19 s (user 9.5 s, sys 5.7 s), and the check reads every archive with a fixed two workers (from #46), so the wall time grows with the archive count. ubuntu-latest runners for public repositories have 4 vCPUs.

## Approach

Use one worker per CPU that `os.cpu_count()` reports, at least 2 and at most 8, for the independent read-only status reads of the fixed current checkout. Everything else is unchanged: history replay stays serial, every obligation and predicate stays, and every submitted read is joined before the final fingerprint.

## Ruled out

- A SpecSync build with spec-sync#840's fast archived reads: #840 is still open and 6.1.2 is the latest release; building from an unmerged PR is not allowed.
- Raising the job timeout.
- Skipping archives already authenticated on the base: deferred, since it changes what this run authenticates.

## From the change's design.md

# Design

- `archive_workers()` returns `max(2, min(os.cpu_count() or 2, 8))`.
- `authenticate_archives` uses `ThreadPoolExecutor(max_workers=archive_workers())`. Nothing else in the executor changes: it prechecks presence, submits in sorted ID order, reports errors in ID order and joins every read before leaving the context.
- No other function changes.

## From the change's testing.md

# Testing

- `python3 .github/scripts/test-specsync-archive.py`:
  - A new worker-count test maps CPU counts 1, 3, 4, 64 and None to 2, 3, 4, 8 and 2.
  - The overlap test runs with 3 CPUs: exactly three reads overlap, no fourth starts until one is released, every ID is read once and the peak is 3.
  - The new tests fail against main 1ac2ae2.
  - The existing error-order, drain and real native 6.1.2 fixture tests pass unchanged.
- The check is timed on the #71 head (base 1ac2ae2) with 2, 4 and 8 workers.
- `fledge lanes run verify` passes.

## Where these lessons go

- `specs/cli/context.md`
