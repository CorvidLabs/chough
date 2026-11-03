---
change: integrate-accepted-history-and-workflow-changes-into-pr-46-report-reads
artifact: design
---

# Design

No architecture or persisted schema is added by this scope. Preserve the accepted history cursor, immutable event envelopes and fail-closed retention semantics. The report cache remains ephemeral and root-bound per request. Native tools retain lifecycle authority and typed diagnostics. The checkpoint remains the only current-state store; current-cycle fields stay atomic while additive histories are merged. Imported archived implementation ownership is retained; actual reconciliation has this active native owner.
