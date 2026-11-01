---
change: fresh-agent-handoff-from-existing-records-chough-next-shows-recent-decisions-the-last-failure-and-a-lessons-pointer
artifact: plan
---

# Plan

## Approach

Prove the protocol first, then change the view, then the wording. The cold-start
fixture and rubric are frozen and hashed, and the mandatory baseline runs on main,
before any implementation commit, so the slice cannot be tuned to the fixture. Then
the archived-not-done branch and the three blocks go into the existing `next` view in
`src/main.rs` and the shared Action chain in `src/stage.rs`, test first in
`tests/next.rs`. The skill wording follows, with host copies and known digests. One
agent builds; if split, ownership is: renderer, Action chain and tests/next.rs (owner
A); skill bundle, host copies, src/skill.rs, tests/package.rs and the skill fixture
(owner B); docs, CHANGELOG, specs/cli/context.md and the cold-start exercise files
(owner C).

| Increment | Relative size | Main uncertainty |
|---|---|---|
| Fixture seed script and rubric, hashed observation | small to medium | choosing a failure that is realistic without mirroring the slice |
| Baseline run on main (different host) | small, wall-clock bound | host start-up and model availability on this machine |
| Archived-not-done branch and shared step constants | small | precedence against the gate rule at plan |
| Decisions block with prefix parsing | small | none |
| Last failure block with linked bugs | small | cost of parsing every observation on large records |
| Lessons pointer and archived bundle listing | small | none |
| Cap and determinism fixture | small to medium | building 4 KiB records fast enough for CI |
| Skill wording, KNOWN_PREVIOUS, fixture, host copies | small to medium | rebase against PR #26 if it edits the same bundle files |
| Docs, CHANGELOG, REQ-cli-002 heading, cases | small | none |
| Change run, real-repository run, grading | small, wall-clock bound | a sample of one per run; the grader's availability |

History from `chough timing` is not used: during the recon it did not finish within
three minutes on this repository (unexplained, not investigated here), so estimates
are relative size only.

## Out of scope

Everything listed under Deferred in the design: cross-project reuse; independent
workstreams and multiple checkpoint writers; a readiness gate at accept; a snapshot
dependency manifest; adopting `verification_ignored_paths` and relocating the
checkpoint; corrections for the 7 artifact hash mismatches; bug fix-commit fields
(after PR #26); BUG-0017; BUG-0020; the ROADMAP Kimi row and other housekeeping (a
separate pull request). Also: any checkpoint schema change, any new command, store,
ledger or record kind, any hi/ edit, REQ-cli-012, TC-cli-0125, BUG-0026, and any edit
to SpecSync-written archives.

## Steps

1. Write `docs/exercises/handoff-cold-start/seed.sh` and `rubric.md`; run the seed
   once into a scratch directory to confirm it builds; record one observation whose
   artifacts are the two files (their SHA-256 bound by `chough evidence`); commit.
2. Confirm the chosen host starts (Kimi Code CLI 0.41.0 at
   /Users/leif/.kimi-code/bin/kimi, else Codex CLI 0.153.4); build the main binary at
   fb6e487; seed, install the skill from that binary, run the fixed prompt; keep the
   transcript and command log as observation artifacts. 0xLeif grades the baseline.
3. Tests first in `tests/next.rs` for archived-not-done, decisions, last failure,
   lessons pointer, cap and determinism; record their failing run.
4. Implement in `src/stage.rs` (shared step constants, new rule 6) and `src/main.rs`
   (three blocks, `LISTS`, text lines), with bounded readers from `src/records.rs`,
   `src/evidence.rs` and `src/bugs.rs` as needed (`pub(super)` only).
5. Skill wording; KNOWN_PREVIOUS digests; `tests/fixtures/skill-fb6e487/`; package
   test; `chough skill update` for both host copies.
6. Normalize the canonical REQ-cli-002 heading exactly as requirements.md shows; run
   the scoped `specsync change check <id>`; add TC-cli-0126 to TC-cli-0133 to
   `.chough/cases/cli.cases.toml`; run the automated cases.
7. docs/cli.md, CHANGELOG.md, docs/ROADMAP.md (only to list the slice),
   specs/cli/context.md (lessons from this change, labelled, through the closeout pass).
8. Change run on the same host and model with the implementation binary; one run on
   this repository at the implementation tip; 0xLeif grades both.
9. `fledge lanes run verify`; then the accept order in references/accept.md.

## Risks

- The fixture could still be tuned to the slice. Mitigation: frozen and hashed before
  code, the baseline is mandatory, and 0xLeif, not the author, grades.
- A baseline pass would mean the runs show no difference; the pull request must say so
  rather than claim the criterion.
- Parsing every observation in `next` adds work proportional to the evidence count
  (about 250 records here); the existing record bounds cap each read.
- The prefix convention is best-effort; agents may not write it. The fallback keeps
  the block useful but less precise.
- PR #26 conflicts on `.chough/session.json`; if it also edits bundle files,
  whichever merges second rebases its KNOWN_PREVIOUS lines.
- Writes to `.chough/` stale SpecSync verification until finalize; the accept order
  already handles this, and closeout runs before `check --commit`.
- External hosts change behavior between versions; the run records the exact version.

## Constraints consulted

specs/cli/context.md Lessons (plain-text gates in tasks.md; frozen deltas; REQ text
validated after check; exact `### REQ-cli-NNN` heading for MODIFIED; do not list
`.chough/` in affected paths; `pub(super)`; local Git identity for disposable
repositories; confirm the external agent starts before relying on a cold-start run);
AGENTS.md (no new integration, store, model client or duplicate lifecycle state; no
unobserved claims); REQ-cli-007 (no schema change without migration); REQ-cli-015
(librarian optional); REQ-cli-016 and REQ-cli-019 (known earlier digests only; no
dashes in the bundle); the recon critique of this slice (2026-11-01).
