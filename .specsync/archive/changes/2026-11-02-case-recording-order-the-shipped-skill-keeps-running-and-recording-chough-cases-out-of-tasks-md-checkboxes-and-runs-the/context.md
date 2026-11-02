---
change: case-recording-order-the-shipped-skill-keeps-running-and-recording-chough-cases-out-of-tasks-md-checkboxes-and-runs-the
artifact: context
---

# Context

CorvidLabs/chough#11, found in the Raven run (CorvidLabs/raven#644) and recorded
locally as BUG-0017. A case could not link REQ-home-8, which existed only in an
approved change's delta: `chough cases check` refused it, and `specsync change check`,
which materializes the delta, refused while `tasks.md` still had the unchecked box
"record Chough cases". Each tool waited on the other, and the fallback doubled the
case runs.

PR #16 (8b30943) fixed the error half: the refusal now names the files searched and
the check-first fix (REQ-cli-005, TC-cli-0094). The ordering half stays open.

Decisions binding this change (0xLeif in chat, 2026-11-02, "I approve all", relayed
as the go-ahead on the options proposed in #11; this is not the native scope
approval of this definition):

- Keep REQ-cli-005 unchanged. Chough does not resolve links against native deltas.
- Fix the order in the shipped skill only: running or recording cases is not a
  `tasks.md` checkbox, and the scoped check runs first.
- Trace to existing HI criteria (LIFE-6, FLOW-7); no HI wording changes.

Rejected for this change: resolving links against approved deltas (it would change
REQ-cli-005, which says the hint SHALL NOT read native deltas) and asking SpecSync to
materialize with open tasks (an upstream SpecSync change).

Reproduced on Linux (box, x86_64) with chough 825a380, SpecSync 6.1.2, Fledge 1.8.0
and hi 0.8.0 using the shipping sample: with an open "Record Chough cases" box,
`cases check` exits 1 with the check-first hint and `change check` exits 1 with
"tasks.md contains incomplete tasks"; with the box removed, `change check` exits 0,
REQ-shipping-002 appears in `specs/shipping/requirements.md`, and `cases check` exits
0. This informal run is the planning basis; the recorded reproduction is a task.

For a session picking this up: PR #37 (0xLeif, draft) also edits `src/skill.rs`
KNOWN_PREVIOUS, `tests/package.rs`, `specs/cli/`, `CHANGELOG.md`, `docs/ROADMAP.md`
and the checkpoint; keep edits here additive so either merge order works. #37 adds
REQ-cli-025 to 027, so this change adds no requirement and modifies REQ-cli-009 only.
