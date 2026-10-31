---
change: evolve-chough-into-a-resumable-stage-workflow
artifact: context
---

# Context

## What led here

The bootstrap (HEAD d38aa19) delivered `init`, `context`, `status`, `next` and
`evidence add` with native structural checks while SpecSync SDD was disabled. It made
no claim of scope approval, implementation review or archive. A human brief of
2026-10-31 redirected the next work from the observation-reuse proposal in
`docs/NEXT_CHANGE.md`, which was never approved, to a complete resumable development
workflow: stage routing, conditional design, one scope approval, focused three-track
testing, a local bug backlog, optional validation and release, an optional
librarian, and mandatory checkpoints with cold resume. This change is the first use
of the full native lifecycle in this repository.

## What a session picking this up needs to know

- Native lifecycle was adopted in commit c582a10 on `feat/resumable-stage-workflow`;
  SpecSync writers and CI use 6.1.1. Use a `PATH` that selects hi 0.8.0, fledge 1.7.2
  and specsync 6.1.1; the default `PATH` on the authoring machine picks hi 0.5.0 and an
  unreleased fledge 1.8.0.
- The delta is frozen once approved. `change status` may still show
  `approval_valid: true` after a delta edit; `change check` refuses it.
- `.chough/session.json` still points to `docs/NEXT_CHANGE.md`. That pointer is stale
  planning, not authority, and is replaced during increment 7.
- APS and 3md were evaluated and not selected; see research.md. Do not reintroduce
  them without a new decision.
- Human gates pending: scope approval of this definition, then implementation review
  at acceptance. No agent records either.

## Ruled out

Reusing observations after metadata-only commits; APS as checkpoint backend or mirror;
3md as the test-definition source; Magpie generic mode as an oracle; a dashboard UI.
