---
change: integrate-merged-main-into-pr-46-while-preserving-report-reads-and-lifecycle-guidance
artifact: design
---

# Design

No new architecture or persisted format is introduced. The report cache remains ephemeral for one root-bound request. Native gates remain authoritative. Main's case ordering and unlinked-failure behavior are retained. The existing schema-2 checkpoint stays the only checkpoint. Its current-cycle fields are selected as one group; collection and legacy transition integration preserves both branches' recorded additions. Native integration ownership is separate from the already archived report implementation.
