---
id: parse-native-status-beyond-the-display-bound-and-apply-checkpoint-clears-first
state: approved
type: bug_fix
base_commit: 1a108d9844e5bc02e5439cb6b3688803d6435a53
---

# Parse native status beyond the display bound and apply checkpoint clears first

## Intent

Parse native status beyond the display bound and apply checkpoint clears first

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- With native status reporting summary.state archived in a JSON response larger than 16384 bytes, chough stage advance leaves accept for done with gate native:archived (BUG-0004, REQ-cli-008).
- Native status responses are parsed from the complete output up to a 4 MiB bound; a larger, failed, unavailable or undecodable response still refuses gated transitions as native-unknown.
- chough status and next report a large valid native response as observed rather than invalid-output, show the parsed summary state and next action, and keep the displayed content bounded with a visible truncation note.
- chough checkpoint applies --clear-awaiting-human and --clear-blockers before additions in the same call, so the newly given entries remain (BUG-0005, REQ-cli-007).
- Regression tests cover both defects; the existing suite, the feature-and-bug exercise and native structural checks still pass; the recorded BUG-0004 reproduction, rerun against the real archived evolve-chough-into-a-resumable-stage-workflow change in a disposable copy of the archive tip, now records accept -> done with gate native:archived, and BUG-0004 and BUG-0005 are verified with passing observations.

## No-spec Rationale

Behavior-only fix: REQ-cli-007 and REQ-cli-008 already require these outcomes; the contract text and requirements are unchanged.
