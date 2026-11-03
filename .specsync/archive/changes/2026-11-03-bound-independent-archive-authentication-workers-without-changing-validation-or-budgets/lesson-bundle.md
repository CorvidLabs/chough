# Lesson bundle: bound-independent-archive-authentication-workers-without-changing-validation-or-budgets

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Bound independent archive authentication workers without changing validation or budgets
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: .github/scripts/check-specsync-archive.py, .github/scripts/test-specsync-archive.py
- **Acceptance**: Every historical roster snapshot and archive obligation is retained. Independent current-root archive status reads use a fixed maximum of two workers. Every result must match its requested ID, archived state and authenticated-history validity; no success until all finish. Native errors, invalid archives and command deadlines fail closed. SpecSync 6.1.2, 90-second subprocess timeout, output bounds, read-only fingerprints and the 10-minute hosted job budget stay unchanged. Existing native fixtures plus concurrency/failure/drain tests pass. Actual hosted duration is observed before merge.

## Evidence

- Verification commit: `e8cafc4353915f2eaab94f1c5d4dd7cc6e6a0067`
- Base commit: `9af1921a3bb1f7092d364a18489cb3028e97ff26`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# CI archive execution performance

## Human authority and limits

Acting agent: corvid-agent. Human source: 0xLeif in chat on 2026-11-02 authorized ticket changes through PRs ("I already gave it some approval and told it can do any changes. It just needs a PR."), then "Can you finish up all these?" and "Merge approved, green PRs and resynchronize", followed by "get the 3 prs merged?". This concrete scope fixes the #46 archive executor within unchanged validation and budgets. It does not authorize lowering gates, rewriting recorded proof, a longer timeout, release or deployment. Implementation closeout and merge require actual tests, independent review and green exact-head checks.

## Problem

PR46 at 9af1921 twice authenticated 40 archives but exceeded the unchanged 10-minute job limit. Native 6.1.2 starts a whole-archive authentication snapshot for each requested archive. Historical checkout replay must stay serial.

## Approach

Use exactly two workers for independent read-only status requests against the fixed current checkout. Validate every obligation with the existing predicates. Join every submitted request before leaving the executor and before the final fingerprint. Report results in deterministic ID order. Keep native commands, trust inputs, history replay, version, output bounds and all deadlines unchanged.

## Out of scope

Native cache/validator changes, persistent caches, archive or evidence rewrites, gate or timeout changes, product runtime changes, release and deployment.

## Ownership and estimate

Root owns native/checkpoint/qualification/closeout. Worker owns only the two declared Python files. Small code change; uncertainty is hosted wall time under two CPUs. No duration promise is made.

## Constraints consulted

AGENTS.md, INTENT.md, LIFE-1/2/6/7, FRD native closeout boundary, cli contract/companions, workflow lifecycle and context.md lesson about Chough writes preceding final native verification. Existing readonly validation conditions remain authoritative.

## From the change's testing.md

# Verification plan

Run the existing 12 real-native fixture tests unchanged. Add unit tests for bounded overlap, exactly-once complete ID coverage, wrong-ID/invalid archive refusal, native errors/timeouts and draining outstanding reads before return. Keep historical snapshots serial. Run Fledge verify and the dedicated archive fixture task, plus independent source review. Measure the full checker on this branch and require hosted success inside the existing 10-minute job budget. A unit fixture is not native archive authentication. Earlier product evidence retains its original content identity; no old record is relabeled current.

## Where these lessons go

- `specs/cli/context.md`
