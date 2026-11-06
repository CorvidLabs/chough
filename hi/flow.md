---
hi: 1
families: [FLOW]
---

# Flow

## Intent

I want "a dependable, portable workflow that remembers what was wanted, proves what was actually checked, and lets another agent continue from project files under the same human decisions."

Captured from the human's 2026-10-31 evolution brief and approved with native change evolve-chough-into-a-resumable-stage-workflow. Quoted phrases are the brief's words; the other sentences are first-person normalizations of its sections, not a claim that the workflow is built.

## Criteria

- **FLOW-1**  I can use the agent I am already talking to, through natural language, a supported skill or the CLI, and reach the same stage records without a mandatory provider API or runtime.
- **FLOW-2**  Every stage can iterate, go back, reset, pause, resume, or be skipped with a recorded reason, and backtracking keeps history while naming what must be refreshed.
- **FLOW-3**  I approve the intent stage as a whole after useful review findings are synthesized, not criterion by criterion, and later discoveries add new criteria instead of rewriting what I wanted.
- **FLOW-4**  When a change affects visuals or interactions, one canonical evolving local HTML and CSS prototype with companion notes carries the design, and I approve the rendered prototype and the notes together.
- **FLOW-5**  I give one coherent approval of the technical design and task plan before implementation, over a concrete definition rather than a promise to write one.
- **FLOW-6**  Verification focuses on the affected acceptance criteria and impacted behavior, always includes functional checks, adds copy and design checks when relevant, and does not retest every unrelated criterion.
- **FLOW-7**  Test definitions stay separate from execution observations, with stable case IDs, explicit expected outcomes and real assertions, and a failing case stays failed even when others pass.
- **FLOW-8**  Bugs are reproduced, classified, deduplicated and kept locally with their evidence, and nothing is published externally unless I ask.
- **FLOW-9**  Optional repeated validation and monitoring respect the difference between one-time consent and a standing project preference, honor my pause or stop, and keep every finding locally.
- **FLOW-10**  I review the implementation at acceptance before native archival, and scope approval, agent review, command success or an actor label never stands in for that review.
- **FLOW-11**  The workflow "must not naturally proceed to release every time"; it enters release only when I ask or a project setting covers it, and publication, installation, deployment and live health stay distinct.
- **FLOW-12**  An optional project-local librarian keeps source-linked history, a current index and focused briefs, callable from any stage, without becoming a competing source of truth.
- **FLOW-13**  "Essential state must not exist only in model context": a fresh agent with none of the conversation can find the checkpoint and resume under the same scope, preferences and human gates.
- **FLOW-14**  When I stop Chough with Ctrl-C or a normal kill (SIGINT, SIGTERM or SIGHUP), everything it started stops too and nothing partial is recorded; a hard kill (SIGKILL) that Chough cannot catch is not covered. Source: Leif, card t819s9328, 2026-11-03 7:07 PM MT, "Yes, when I stop Chough everything it started stops too and nothing partial is recorded", answering the card "When you stop Chough (Ctrl-C or a normal kill), everything it started stops too, and nothing half-done gets saved." A SIGKILL watchdog was offered and not chosen.
- **FLOW-15**  When native verification runs none of my project's commands, Chough tells me so, and at the test stage it points me to recorded test evidence before accept. Source: Leif in a Claude Code session, 2026-11-04 about 7:20 AM MST, choosing "Warn and point" for issue #85 from the offered options, whose wording was "When native verification runs none of my project's commands, Chough tells me so, and at the test stage it points me to recorded test evidence before accept." Warning only: a gate that refuses test to accept without a passed observation was offered and not chosen.
- **FLOW-17**  Chough and SpecSync support macOS and Linux; Windows through WSL. Source: Leif in the orc tab, 2026-11-05 8:17 PM MST, his verbatim choice "Mac + Linux; Windows via WSL", relayed by orc (hc t18); the sentence above is the wording orc relayed for that choice, not his verbatim text. The context he chose from, as orc relayed it: "match SpecSync 6.0 (no native Windows); drop native Windows CI from Chough; document and test WSL; keep tests CRLF-safe; Raven Desktop keeps Windows (not this repo)". First captured on this branch as FLOW-16 and renumbered before merge, because the branch of chough#123 holds FLOW-16.
