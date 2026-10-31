---
change: evolve-chough-into-a-resumable-stage-workflow
artifact: requirements
---

# Requirements

## Human intent: proposed HI family FLOW

The 2026-10-31 evolution brief is product direction from the human. The sentences
below are first-person normalizations of its sections, not verbatim quotes, except
where quotation marks appear. They become permanent HI IDs only after the human
approves this definition; then they are captured with `hi FLOW-<n> "<text>"` into
`hi/flow.md` and existing LIFE-1 through LIFE-7 stay unchanged. IDs FLOW-1 to
FLOW-13 are unused today (`hi check`: families LIFE only).

| Proposed ID | Criterion text | Brief source |
|---|---|---|
| FLOW-1 | I can use the agent I am already talking to, through natural language, a supported skill or the CLI, and reach the same stage records without a mandatory provider API or runtime. | §2 current-agent-first |
| FLOW-2 | Every stage can iterate, go back, reset, pause, resume, or be skipped with a recorded reason, and backtracking keeps history while naming what must be refreshed. | §3 |
| FLOW-3 | I approve the intent stage as a whole after useful review findings are synthesized, not criterion by criterion, and later discoveries add new criteria instead of rewriting what I wanted. | §4.1 |
| FLOW-4 | When a change affects visuals or interactions, one canonical evolving local HTML and CSS prototype with companion notes carries the design, and I approve the rendered prototype and the notes together. | §4.2 |
| FLOW-5 | I give one coherent approval of the technical design and task plan before implementation, over a concrete definition rather than a promise to write one. | §4.3 |
| FLOW-6 | Verification focuses on the affected acceptance criteria and impacted behavior, always includes functional checks, adds copy and design checks when relevant, and does not retest every unrelated criterion. | §4.5 |
| FLOW-7 | Test definitions stay separate from execution observations, with stable case IDs, explicit expected outcomes and real assertions, and a failing case stays failed even when others pass. | §4.5 |
| FLOW-8 | Bugs are reproduced, classified, deduplicated and kept locally with their evidence, and nothing is published externally unless I ask. | §4.6 |
| FLOW-9 | Optional repeated validation and monitoring respect the difference between one-time consent and a standing project preference, honor my pause or stop, and keep every finding locally. | §4.7 |
| FLOW-10 | I review the implementation at acceptance before native archival, and scope approval, agent review, command success or an actor label never stands in for that review. | §4.8 |
| FLOW-11 | The workflow "must not naturally proceed to release every time"; it enters release only when I ask or a project setting covers it, and publication, installation, deployment and live health stay distinct. | §4.9 |
| FLOW-12 | An optional project-local librarian keeps source-linked history, a current index and focused briefs, callable from any stage, without becoming a competing source of truth. | §4.10 |
| FLOW-13 | "Essential state must not exist only in model context": a fresh agent with none of the conversation can find the checkpoint and resume under the same scope, preferences and human gates. | §4.11 |

## Derived functional requirements

`docs/FRD.md` gains a cross-module section for this evolution and links to the
native requirements rather than forking them. Native requirements REQ-cli-007 to
REQ-cli-017 are defined in `deltas/cli.md`; REQ-cli-001 to REQ-cli-006 are unchanged
and remain regression obligations.

| Requirement | Summary | Criteria |
|---|---|---|
| REQ-cli-007 | Canonical checkpoint schema 2 and explicit migration | FLOW-13, LIFE-3 |
| REQ-cli-008 | Stage routing bounded by native gates | FLOW-2, FLOW-5, FLOW-10, LIFE-1 |
| REQ-cli-009 | Validated `chough-testing/1` definitions | FLOW-7, LIFE-2 |
| REQ-cli-010 | Focused case selection | FLOW-6 |
| REQ-cli-011 | Honest case execution and repeated runs | FLOW-7, FLOW-9, LIFE-7 |
| REQ-cli-012 | Local bug backlog | FLOW-8, LIFE-5 |
| REQ-cli-013 | Preferences and run consent | FLOW-9, FLOW-11 |
| REQ-cli-014 | Optional release routing | FLOW-11, LIFE-7 |
| REQ-cli-015 | Optional librarian | FLOW-12 |
| REQ-cli-016 | Host skill installation | FLOW-1, LIFE-4 |
| REQ-cli-017 | Observation schema 2 | LIFE-2, LIFE-7, FLOW-7 |

Skill-led behavior with no helper requirement: intent interviewing, FRD synthesis,
role review passes (FLOW-3), the design stage (FLOW-4) and scope presentation
(FLOW-5) live in the skill references and templates. Their acceptance evidence comes
from documented exercises, not from helper tests alone.

## Brief rubric mapping

| Brief ID | Criteria and requirements |
|---|---|
| EV-01 | FLOW-1, REQ-cli-016, router and skill references |
| EV-02 | FLOW-3, intent and roles references, intent exercise |
| EV-03 | FLOW-4, design reference and templates, visual and nonvisual samples |
| EV-04 | FLOW-5, this native change |
| EV-05 | FLOW-2, REQ-cli-008 |
| EV-06 | FLOW-6, REQ-cli-010 |
| EV-07 | FLOW-7, REQ-cli-009, REQ-cli-011, REQ-cli-017 |
| EV-08 | FLOW-8, REQ-cli-012 |
| EV-09 | FLOW-9, REQ-cli-011, REQ-cli-013 |
| EV-10 | FLOW-10, REQ-cli-008 |
| EV-11 | FLOW-11, REQ-cli-014 |
| EV-12 | FLOW-12, REQ-cli-015 |
| EV-13 | FLOW-13, REQ-cli-007, cold-start exercise |
| EV-14 | Design decision D-010 (APS deferred); no HI criterion |
| EV-15 | LIFE-2, LIFE-7, REQ-cli-001 to REQ-cli-006 |
| EV-16 | LIFE-1, LIFE-4, documentation and diagram verification |

## Non-goals

No provider client, hosted service, custom agent runtime, universal slash-command
engine, Rune session coordination, APS backend, Magpie adapter, Trust/Augur/Attest
gate integration, real release, deployment, live monitoring, external issue
publication, or reuse of observations after metadata-only commits.
