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

- [ ] Capture approved FLOW criteria with `hi` and run `hi check`.
- [ ] Add `src/records.rs` with bounded reads, atomic writes and ID validation; keep existing helpers' behavior.
- [ ] Add checkpoint schema 2, `checkpoint` editing and `migrate` with backup preservation (REQ-cli-007).
- [ ] Add stage table, transitions log and native gate reading; extend `next` and `context` (REQ-cli-008).
- [ ] Tests for every transition kind, gate states, migration and schema 1 compatibility.

## Increment 2 — skill and hosts

- [ ] Rewrite `SKILL.md` as a short router; add `references/stages.md` and per-stage references.
- [ ] Add templates (FRD, review findings, prototype HTML/CSS/companion, cases, bug report, brief).
- [ ] Add `init --host`, `skill status` and `skill update` with 0.1.0 digests (REQ-cli-016).
- [ ] Test that helper stage names match `references/stages.md`.

## Increment 3 — test definitions and execution

- [ ] Add `toml` dependency, `chough-testing/1` validation and digests (REQ-cli-009).
- [ ] Add `cases select` with tracks and omissions (REQ-cli-010).
- [ ] Add adapters, `cases run`, `cases record` and observation schema 2 (REQ-cli-011, REQ-cli-017).
- [ ] Tests: reorder invariance, rejection paths, zero-test not-run, failing assertion stays failed, stale-case-definition.

## Increment 4 — bugs

- [ ] Add bug records, search, update, regression rule and report candidate (REQ-cli-012).
- [ ] Tests: reproduce and classify, duplicate search with appended evidence, regression refusal, out-of-scope visibility.

## Increment 5 — preferences, validation and release

- [ ] Add preferences and run consent with objective clearing (REQ-cli-013).
- [ ] Add repeated validation with stop and resume and failure capture (REQ-cli-011, REQ-cli-013).
- [ ] Add release routing and delivery kinds (REQ-cli-014).

## Increment 6 — librarian

- [ ] Add `library init`, `snapshot` and `index`, and the brief template (REQ-cli-015).

## Increment 7 — policy, docs and self-use

- [ ] Set `require_change_for_meaningful_files` and add `skills/` to meaningful paths.
- [ ] Add `specsync change audit` with `fetch-depth: 0` to CI.
- [ ] Migrate Chough's own session; point it at this change; mark `docs/NEXT_CHANGE.md` superseded.
- [ ] Write `.chough/cases/cli.cases.toml` for Chough's own requirements.
- [ ] Update README, FRD, decisions, roadmap, architecture, lifecycle, CLI reference, walkthrough, integrations and module companions.
- [ ] Update and add Mermaid diagrams; render; inspect images; update docs/verification.md.

## Increment 8 — exercises and verification

- [ ] Isolated feature and bug loop with a deliberately failing assertion, local bug, fix and retest.
- [ ] Nonvisual design skip and visual sample with rendered review artifacts.
- [ ] Preference and consent fixtures; release routing fixture without any real release.
- [ ] Genuine cold start by a separate agent CLI given only repository files; record transcript and assessment.
- [ ] `fledge lanes run verify`, `specsync change check <id> --commit`, push, draft PR, hosted CI at the tip.
