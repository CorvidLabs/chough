---
id: finish-the-bounded-capacity-proposal-with-named-refusals-and-accurate-provenance
state: approved
type: bug_fix
base_commit: dbb66d979e2c34e41926401641d58971c0d65f65
---

# Finish the bounded capacity proposal with named refusals and accurate provenance

## Intent

Finish the bounded capacity proposal with named refusals and accurate provenance

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- The401-observation refusal explicitly names400 and preserves record bytes; strengthened test fails before repair and passes afterward unchanged. Current HI/contract/context/changelog accurately separate the actual human capacity goal and finish-through-PR instruction from the agent-selected131072/400values. Existing checkpoint-only/list-only isolation and all other bounds/path/symlink/atomic protections remain. Fresh final-source verification covers post-archive provenance edits, and every old record/archive stays byte-exact. Independent review and all exact-head checks precede authorized merge. Import final accepted PR50 before final committed qualification.

## No-spec Rationale

Keep the existing proposal's131072checkpoint/400observation implementation values as agent-selected under the authorized capacity goal, satisfy its existing named-bound diagnostic requirement, and qualify current provenance text through a fresh native followup without rewriting any prior archive. No new capacity choice or other bound change.
