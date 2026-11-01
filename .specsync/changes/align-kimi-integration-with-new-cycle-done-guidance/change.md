---
id: align-kimi-integration-with-new-cycle-done-guidance
state: implementing
type: bug_fix
base_commit: b2e2f19d904aec2a5acd73569c8439f5583c01f9
---

# Align Kimi integration with new-cycle done guidance

## Intent

Align Kimi integration with new-cycle done guidance

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- Archived done checkpoints with authenticated native evidence follow REQ-cli-021 by recommending a new cycle with a reason-bearing reset. They never repeat stale native review or ship actions. Pause, human gates, blocking bugs, closed-gate precedence and the unchanged checkpoint action retain their existing behavior.
- TC-cli-0105 is updated to the newer REQ-cli-021 contract. A failure at the integrated pre-fix tip is linked to BUG-0025 before implementation, followed by a later passing observation. TC-cli-0121 and TC-cli-0122 also pass. Original observations remain retained and historical.
- The Kimi records and affected docs distinguish the superseded native-routing implementation from integrated new-cycle behavior. BUG-0024 remains separate from main open BUG-0020 and the new integration defect is BUG-0025. No frozen archive is edited and no fresh post-fix Kimi run is claimed.
- Focused Rust tests and fledge lanes run verify and pre-push pass with HI 0.8.0, Fledge 1.7.2 and SpecSync 6.1.1. The PR remains draft through implementation review. Merge follows only after archive and passing checks under 0xLeif explicit instruction. No release or deployment is claimed.

## No-spec Rationale

REQ-cli-021 already requires new-cycle guidance for archived done checkpoints; this integration follows that contract and records evidence without a canonical delta
