# Lesson bundle: case-recording-order-the-shipped-skill-keeps-running-and-recording-chough-cases-out-of-tasks-md-checkboxes-and-runs-the

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Case recording order: the shipped skill keeps running and recording Chough cases out of tasks.md checkboxes and runs the scoped specsync change check first, so a case can link a requirement an approved change adds (BUG-0017, issue 11)
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: skills/chough-workflow/references/plan.md, skills/chough-workflow/references/testing.md, skills/chough-workflow/references/accept.md, .agents/skills/chough-workflow/references/, .claude/skills/chough-workflow/references/, src/skill.rs, tests/package.rs, CHANGELOG.md, docs/ROADMAP.md, specs/cli/
- **Acceptance**: The shipped plan, testing and accept references say a requirement an approved change adds is linkable only after the scoped specsync change check materializes it, so that check runs before cases that link it are added, run or recorded
- **Acceptance**: The same references say running or recording Chough cases is not a tasks.md checkbox, because specsync change check refuses while a box is unchecked
- **Acceptance**: REQ-cli-005 and the chough cases check refusal are unchanged; Chough reads no native deltas
- **Acceptance**: Unedited copies of the plan, testing and accept references from main 825a380 read known-outdated and update; edited copies stay edited; both host copies in this repository read current
- **Acceptance**: A Linux run with SpecSync 6.1.2 shows the refusal with an open case-recording box and the passing order without it
- **Acceptance**: fledge lanes run verify passes locally; hosted CI is reported only as observed

## Evidence

- Verification commit: `4902615a3b19b4bfc8d95691555593c38b521ec1`
- Base commit: `825a3805530456005e361dfa6d96d93eee52cd09`
- Verified by: `specsync check --spec cli`

## From the change's context.md

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

## From the change's design.md

# Design

Skill text only, plus the earlier digests that let unedited host copies update. No
helper behavior, output or record format changes.

## references/plan.md

The lessons paragraph after "One approval" becomes (proposed text):

```markdown
Lessons learned from rehearsing SpecSync 6.1: the delta is frozen once approved and
`change check` refuses an edited delta; `change check` also refuses while `tasks.md`
has unchecked boxes, so list human gates, and running or recording Chough cases, as
plain text rather than checkboxes. A requirement the delta adds reaches the
companion only when `change check` materializes it, and `chough cases check` refuses
a case linking it until then, so a box for those cases would block the very check
that unblocks them.
```

## references/testing.md

After the `chough cases check` paragraph in "Definitions are not observations", one
new paragraph (proposed text):

```markdown
A requirement an approved native change adds is not in the companion until the
scoped `specsync change check <id>` materializes it, so `cases check` refuses a case
that links it before then. Finish the implementation tasks, run that check, then add,
run and record those cases. Running or recording cases is never a `tasks.md`
checkbox: `change check` refuses while any box is unchecked.
```

## references/accept.md

Step 1 of "Prepare the review" keeps its text and gains one sentence (proposed):

```markdown
1. When the delta adds or modifies requirements, run `specsync change check <id>`
   first, so the requirement text that cases link is materialized. It refuses while
   `tasks.md` has an unchecked box, so the cases in step 2 are never a box there.
```

## Installed copies and earlier digests

`chough skill update` refreshes `.agents/` and `.claude/`. KNOWN_PREVIOUS in
`src/skill.rs` gains the main 825a380 digests of the three references:

- references/plan.md `1022db57d0c114effc8669ede9ebb453f932109f608f883712725e57ba53551d`
- references/testing.md `426c2df68e8d25bfbdee4bf81b4deaa06eac4525b6dfad3c771311e0d6a4ac99`
- references/accept.md `80f71366ad81e1720bdc5f91d51c5428cd6a60729c9d1675977e284f7b08e398`

The entries are appended as one commented block, separate from PR #37's added
checkpoint.md entry, so the two merge in either order. `tests/package.rs` keeps the
825a380 bytes inline as consts, as earlier changes did, because a fixture directory
would sit outside the affected paths.

## Not changed

`src/cases.rs`, REQ-cli-005 and the `cases check` hint. Chough still reads no native
delta, keeps no lifecycle state of its own and does not ask SpecSync to change.

## Estimate

Small: three short skill paragraphs, three digests and two package tests. Main
uncertainty: the inline 825a380 reference consts in `tests/package.rs` and a textual
merge with PR #37 in `src/skill.rs`. `chough timing` on this repository (estimates
from recorded history, elapsed wall-clock, not targets): build median 26m 37s
(12 samples, 1s to 1h 30m 36s), test median 48s (11 samples), accept median 20m 37s
(12 samples). Human gates are not included.

## From the change's testing.md

# Testing

## Cases

| Case | Requirement | Criteria | Method | What it shows |
|---|---|---|---|---|
| TC-cli-skill-orders-the-change-check-before-new-requirement-cases | REQ-cli-009 | FLOW-7, LIFE-6, LIFE-2 | agent-assisted, reviewer-note | At the implementation tip the canonical plan, testing and accept references say the scoped check precedes cases linking a requirement the change adds and that running or recording cases is not a `tasks.md` checkbox; no dash; `chough skill status` reports both host copies current |
| TC-cli-0094 (existing) | REQ-cli-005 | as defined | automated | The refusal and check-first hint are unchanged |

The new case is added only after the scoped check materializes the delta, which is
the order this change documents. The package tests (automated, in the full suite)
cover the known-outdated updates and the guidance being present.

## Reproduction

A disposable copy of `examples/shipping`, driven like `examples/feature-bug-loop.sh`
up to build, with one extra unchecked "Record Chough cases" box: `cases check` and
`change check` are expected to exit 1; with the box removed, `change check` exits 0,
the requirement is materialized and `cases check` exits 0. The log is kept as an
artifact of the observation linked to BUG-0017; BUG-0017 is verified only by a pass
observed after its failure.

## Regression

The full suite, `chough cases check` and `fledge lanes run verify` on Linux with
Fledge 1.8.0, SpecSync 6.1.2 and hi 0.8.0. Hosted CI (Ubuntu, macOS, Windows, MSRV)
is reported only as observed; the Archive job blocks while this change is open.

## Where these lessons go

- `specs/cli/context.md`
