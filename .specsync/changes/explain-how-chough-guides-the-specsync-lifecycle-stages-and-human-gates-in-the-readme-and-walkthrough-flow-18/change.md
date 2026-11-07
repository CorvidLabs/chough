---
id: explain-how-chough-guides-the-specsync-lifecycle-stages-and-human-gates-in-the-readme-and-walkthrough-flow-18
state: implementing
type: documentation
base_commit: e2d20dad504ee21fd304150632a21aa71d074d37
---

# Explain how Chough guides the SpecSync lifecycle, stages and human gates in the README and walkthrough (FLOW-18)

## Intent

Explain how Chough guides the SpecSync lifecycle, stages and human gates in the README and walkthrough (FLOW-18)

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- tests/docs.rs and fledge lanes run verify pass with the updated pages; the README states, with the loop shown as goal to agent work to human decision to tested PR, who acts and who decides at each stage, that chough change new/answer/check/status pass SpecSync output through unchanged and only change ship also closes the review gate and moves the session to done, and that chough next is routing guidance rather than an unattended runner; walkthrough and lifecycle match that wording; no helper behavior, requirement text, stage semantics, retention or SpecSync changes, and the archive-stage proposal is not described as current behavior.

## No-spec Rationale

No requirement text changes: the README, walkthrough and lifecycle docs are corrected to describe what the helper and the workflow skill already do (which SpecSync steps chough change wraps, that next is routing guidance, that the agent advances stages, and which human gates remain). No helper behavior, stage semantics, retention or SpecSync changes.
