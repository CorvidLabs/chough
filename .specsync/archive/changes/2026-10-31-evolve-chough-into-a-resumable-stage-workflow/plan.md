---
change: evolve-chough-into-a-resumable-stage-workflow
artifact: plan
---

# Plan

## Approach

Deliver the design in eight increments on branch `feat/resumable-stage-workflow`,
each ending with focused checks, a checkpoint update and native companion notes.
Increments 1 to 6 are mostly independent behind the shared records module, so they
can run as parallel ownership areas, but the default is one agent working through them
in order. Increments 7 and 8 integrate and exercise the whole loop. Fledge runs every
check with CI-matched tools (hi 0.8.0, fledge 1.7.2, specsync 6.1.1).

Effort is given as relative size with the main uncertainty, not as hours.

| # | Increment | Size | Main uncertainty |
|---|---|---|---|
| 1 | Records module, checkpoint schema 2, explicit migration, stage routing with native gates, `next` | Large | Exact native status JSON for archived changes; confirm in a disposable clone |
| 2 | Skill router, stage reference, per-stage references, templates, host install and skill status/update | Medium | Amount of guidance needed to stay short yet sufficient for a cold start |
| 3 | `chough-testing/1` parsing, validation, digest, selection, adapters, `cases run/record`, observation schema 2 | Large | Cargo summary parsing across platforms; timeout handling on Windows |
| 4 | Local bug backlog with search, regression rule and report candidate | Medium | Usefulness of token-overlap search; kept deliberately simple |
| 5 | Preferences, run consent, repeated validation with stop/resume, release routing | Medium | Stop/resume semantics remain foreground-only; no daemon |
| 6 | Optional librarian snapshot, index and brief template | Small | None significant |
| 7 | Policy and CI, Chough's own session migration, cases for Chough itself, docs, diagrams, marking NEXT_CHANGE deferred | Medium | Diagram readability across several linked charts |
| 8 | Exercises: isolated feature/bug loop with a failing assertion and retest, visual and nonvisual samples with rendered review, preference fixtures, real cold start by a separate agent CLI, hosted CI | Medium | Behaviour of a fresh external agent session |

If parallel agents are used: increment 3 owns `src/cases.rs`, `tests/cases.rs` and
`.chough/cases/`; increment 4 owns `src/bugs.rs` and `tests/bugs.rs`; increment 6
owns `src/library.rs` and `tests/library.rs`; increment 2 owns `skills/`. Only the
coordinating agent edits `src/main.rs`, `src/records.rs`, `specs/` and
`.specsync/`. Each agent preserves others' edits and receives only its files, the
design and the relevant requirements.

## Out of scope

APS backend or mirror; Magpie, Rune, Trust, Augur, Attest or Atlas integration;
model provider clients, hosted services or custom runtimes; universal slash
commands; a Chough dashboard; real releases, tags, publication, deployment or live
monitoring; posting issues or comments; autonomous remediation; observation reuse
after metadata-only commits from `docs/NEXT_CHANGE.md`; changing the Fledge Action
pins; recording human scope approval, implementation review or archive on anyone's
behalf.

## Steps

1. After real approval: record `specsync change approve` with the human's actor
   label, capture approved FLOW criteria with `hi`, and update INTENT.md's index.
2. Increment 1, then `fledge run test` and `fledge run lint`; update the checkpoint.
3. Increments 2 to 6 in order, each followed by focused tests, the existing regression
   suite, `fledge run spec` and a checkpoint save.
4. Increment 7: policy, CI audit step, docs, diagrams rendered and visually inspected,
   Chough's own migration and case definitions.
5. Increment 8: exercises with artifacts and observations, including a genuine
   fresh-session cold start; failures go to the local backlog and back to build.
6. `fledge lanes run verify`, then `specsync change check <id> --commit` after
   staging new files; push the branch; open a draft PR; observe hosted CI at the tip.
7. Hand back for the human implementation review with native review and archive
   reported as pending. After a real review decision: `change review`, then `change
   ship` or `finalize` with no intervening commit, push the archive, observe CI, merge
   through the normal process.

## Risks

- Scope size: many surfaces in one change. Mitigation: shared records module, small
  per-module APIs, and increments that each leave the suite green.
- Gate coupling to SpecSync JSON: a format change could refuse transitions. Mitigation:
  unknown states refuse with the raw output visible, never pass.
- Executing bound commands changes a bootstrap invariant. Mitigation: explicit
  invocation only, argv without a shell, bounded output, timeout, and vacuous runs
  recorded as not-run.
- Policy enabling `require_change_for_meaningful_files` adds friction to later small
  edits; this is the intended full lifecycle and is reversible by a later change.
- The cold-start exercise depends on another agent CLI and its account; if it cannot
  run, FLOW-13 stays explicitly pending rather than substituted by a same-context replay.
- `change status` showed `approval_valid: true` after a delta edit that `change check`
  rejected; the delta stays frozen after approval and checks rely on `change check`.

## Constraints consulted

- specs/cli/context.md lesson: keep the reviewed Fledge wrapper commit pin separate
  from the v1.7.2 binary pin and verify Action assets before changing it. Honored: no
  Fledge pin change.
- Bootstrap decisions in specs/cli/context.md and docs/DECISIONS.md D-001 to D-007:
  HI owns intent, SpecSync owns lifecycle, Fledge owns execution, no model client,
  runtime or hosted service, adapters preserve native status. Honored throughout.
- Windows CI fix in tests/cli.rs: the oversized-record test stays portable within the
  Windows argument limit.
- Upstream SpecSync adoption guidance: same-PR archive, no commit between review and
  finalize, stage new source files before `check --commit`, build directories ignored.
