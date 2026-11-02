---
change: bound-transition-history-and-preserve-referenced-proof
artifact: requirements
---

# Requirements

Propose ADDED REQ-cli-025 to REQ-cli-027 in deltas/cli.md. They refine existing LIFE/FLOW intent. Existing REQ-cli-007, 008, 018, 020, 021, 023 and 024 keep their IDs and meaning; their readers and tests must continue to pass.

- REQ-cli-025: explicit transition-history adoption, immutable events, a cursor in the sole schema-2 checkpoint, parent-based lineage and coherent checkpoint merging.
- REQ-cli-026: bounded recent/timing readers, per-cycle storage limits, explicit retention preview/apply and honest partial history.
- REQ-cli-027: retention protection revalidation and conservative interrupted-cleanup reporting without changing historical observation bytes, hashes or existing ledger entry shapes.

The new optional checkpoint cursor is part of the atomic current-cycle group. An older helper may refuse an adopted checkpoint as unsupported; documentation must say so. Legacy schema-2 checkpoints without the field and schema-1 migration remain supported. Adoption is explicit and no live repository adoption or pruning is authorized by approving implementation scope.
