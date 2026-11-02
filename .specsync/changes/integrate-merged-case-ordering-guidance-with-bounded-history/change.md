---
id: integrate-merged-case-ordering-guidance-with-bounded-history
state: implementing
type: documentation
base_commit: d80066472bc52eb0968d39e3db8b4e4a274eb19f
---

# Integrate merged case ordering guidance with bounded history

## Intent

Integrate merged case ordering guidance with bounded history

## Affected Canonical Specs

- None

## Acceptance Criteria

- Preserve already approved case-order guidance and bounded-history guidance together, keep canonical and installed copies consistent without overwriting edits, preserve all archived proof and transition occurrences with one coherent PR37 checkpoint cycle, and pass fresh native audit plus the mandatory Fledge lane, Rust1.88 suite and affected history cases after integration.

## No-spec Rationale

Reconcile already approved and archived main case-order guidance with this active bounded-history branch. No new requirement or product policy is introduced; canonical contract reconciliation stays in the primary cli change. Preserve all proof and one coherent current-cycle checkpoint.
