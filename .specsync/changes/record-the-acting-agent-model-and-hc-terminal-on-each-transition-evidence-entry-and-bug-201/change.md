---
id: record-the-acting-agent-model-and-hc-terminal-on-each-transition-evidence-entry-and-bug-201
state: draft
type: feature
base_commit: d613b415e0660869e82f25a669fc1bc4c8fd82be
---

# Record the acting agent, model and hc terminal on each transition, evidence entry and bug (201)

## Intent

Record the acting agent, model and hc terminal on each transition, evidence entry and bug (201)

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- With CHOUGH_ACTOR, CHOUGH_ACTOR_MODEL and CHOUGH_HC_TERMINAL set, a stage transition, an evidence add and a bug add each record actor {agent, model, terminal}; with CLAUDECODE and HC_TERMINAL_ID set they record agent claude-code and the hc terminal; with nothing known every command succeeds and no record carries an actor, so existing record bytes are unchanged. chough next (text and JSON) and chough status name who made the latest transition (last_actor), and next names who recorded each listed observation (recorded_by); status shows each observation record with its actor. Bug IDs do not depend on the actor. Test fixtures clear the detection variables so the suite is environment-independent.

## No-spec Rationale

Not applicable
