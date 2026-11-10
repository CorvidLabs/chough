---
change: record-the-acting-agent-model-and-hc-terminal-on-each-transition-evidence-entry-and-bug-201
artifact: plan
---

# Plan

## Approach

Add the actor module and the optional field on the three record kinds, set it at each creation site, show it in next and status, isolate the test fixture from the detection variables, and cover the behaviour with one regression bound to a change-specific case.

## Out of scope

Handoff, ownership of stages, dispatch, authentication of the actor, recording the actor on bug updates or notes, and detecting hosts whose environment markers have not been observed.

## Steps

1. `src/actor.rs`: the `Actor` record and `current()` detection with overrides and bounds.
2. Add `actor` to `Transition`, `Evidence` and `Bug`; set it where each is created (stage moves and their direct-write events, `evidence add`, `cases run` and `cases record`, `bug add` and case-failure bugs); probes and unit fixtures use none.
3. `next` and `status`: `last_actor`; `next` observation items: `recorded_by`; text lines.
4. Test fixture clears the detection variables; one regression and its case.
5. docs/cli.md and CHANGELOG.

## Risks

- Older Chough builds refuse records holding `actor` (deny_unknown_fields). Mitigated by naming the first reader in the CHANGELOG; mixed-version teams must upgrade first.
- Tests that spawn the binary outside the shared fixture inherit the host's markers; the full suite is run under Claude Code and hc to catch byte-exact comparisons.
- The actor is an unauthenticated claim; docs and the requirement say so.

## Constraints consulted

Leif's #201 decision; LIFE-17 (PR #212); FLOW-16 (recorded claims never stand in for a human decision); REQ-cli-017 observation schema compatibility.
