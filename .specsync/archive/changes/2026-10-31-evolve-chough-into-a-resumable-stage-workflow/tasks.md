---
change: evolve-chough-into-a-resumable-stage-workflow
artifact: tasks
---

# Tasks

Human gates are decisions, not tasks, so they are not checkboxes: an agent never
checks them off, and `change check` requires every listed task to be complete.

Human gates for this change:

1. Scope approval of this definition, recorded with `specsync change approve` only
   after the human decides.
2. Implementation review at acceptance, after verification and hosted CI, recorded
   with `specsync change review` only after the human decides.

## Increment 1 — checkpoint and stages

- [x] Capture approved FLOW criteria with `hi` and run `hi check`.
- [x] Add `src/records.rs` with bounded reads, atomic writes and ID validation; keep existing helpers' behavior.
- [x] Add checkpoint schema 2, `checkpoint` editing and `migrate` with backup preservation (REQ-cli-007).
- [x] Add stage table, transitions log and native gate reading; extend `next` and `context` (REQ-cli-008).
- [x] Tests for every transition kind, gate states, migration and schema 1 compatibility.

## Increment 2 — skill and hosts

- [x] Rewrite `SKILL.md` as a short router; add `references/stages.md` and per-stage references.
- [x] Add templates (FRD, review findings, prototype HTML/CSS/companion, cases, bug report, brief).
- [x] Add `init --host`, `skill status` and `skill update` with 0.1.0 digests (REQ-cli-016).
- [x] Test that helper stage names match `references/stages.md`.

## Increment 3 — test definitions and execution

- [x] Add `toml` dependency, `chough-testing/1` validation and digests (REQ-cli-009).
- [x] Add `cases select` with tracks and omissions (REQ-cli-010).
- [x] Add adapters, `cases run`, `cases record` and observation schema 2 (REQ-cli-011, REQ-cli-017).
- [x] Tests: reorder invariance, rejection paths, zero-test not-run, failing assertion stays failed, stale-case-definition.

## Increment 4 — bugs

- [x] Add bug records, search, update, regression rule and report candidate (REQ-cli-012).
- [x] Tests: reproduce and classify, duplicate search with appended evidence, regression refusal, out-of-scope visibility.

## Increment 5 — preferences, validation and release

- [x] Add preferences and run consent with objective clearing (REQ-cli-013).
- [x] Add repeated validation with stop and resume and failure capture (REQ-cli-011, REQ-cli-013).
- [x] Add release routing and delivery kinds (REQ-cli-014).

## Increment 6 — librarian

- [x] Add `library init`, `snapshot` and `index`, and the brief template (REQ-cli-015).

## Increment 7 — policy, docs and self-use

- [x] Set `require_change_for_meaningful_files` and add `skills/` to meaningful paths.
- [x] Add `specsync change audit` with `fetch-depth: 0` to CI.
- [x] Migrate Chough's own session; point it at this change; mark `docs/NEXT_CHANGE.md` deferred (unapproved, not rejected).
- [x] Write `.chough/cases/cli.cases.toml` for Chough's own requirements.
- [x] Update README, FRD, decisions, roadmap, architecture, lifecycle, CLI reference, walkthrough, integrations and module companions.
- [x] Update and add Mermaid diagrams; render; inspect images; update docs/verification.md.

## Increment 8 — exercises and verification

- [x] Isolated feature and bug loop with a deliberately failing assertion, local bug, fix and retest (examples/feature-bug-loop.sh; fledge run exercise).
- [x] Nonvisual design skip and visual sample with rendered review artifacts (docs/exercises/visual-design.md).
- [x] Preference and consent fixtures; release routing fixture without any real release (tests/stages.rs, tests/optional.rs).
- [x] Genuine cold start by a separate agent CLI given only repository files; transcript and assessment recorded (docs/exercises/cold-start.md).
- [x] Local verification lanes before the scoped native check.

Closeout steps after the scoped check (plain text, not tasks): run `specsync change
check <id> --commit`; record Chough's own case observations at the product tip;
push; update the draft PR; observe hosted CI at the real tip; hand back for the human
implementation review. Native review and archive happen only after that decision.
