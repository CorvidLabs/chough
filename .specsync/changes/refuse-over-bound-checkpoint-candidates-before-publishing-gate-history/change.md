---
id: refuse-over-bound-checkpoint-candidates-before-publishing-gate-history
state: approved
type: bug_fix
base_commit: 9bd3d506b86cbb312a5a3e195697cd3138920039
---

# Refuse over-bound checkpoint candidates before publishing gate history

## Intent

Refuse over-bound checkpoint candidates before publishing gate history

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- Observation-count and serialized-byte overflow through the checkpoint writer with gate changes refuse before any managed record changes; exact bounds and successful gate history still work; all old evidence and unrelated limits remain byte-identical; focused and required gates pass with current proof.

## No-spec Rationale

Repair existing REQ-cli-032 byte-identical refusal and REQ-cli-007 bounded checkpoint invariants; no new product contract, bound, schema or history policy.
