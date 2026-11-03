---
change: read-report-case-definitions-once-per-request-and-expose-local-phase-costs
artifact: tasks
---

# Tasks

- [x] Retain baseline provenance and implement the root-bound request-local case index.
- [x] Thread the index through report observation and reference freshness paths.
- [x] Add bounded opt-in local phase/counter tracing without changing stdout or exit results.
- [x] Add and pass focused read-budget, failure and next-request invalidation regressions.
- [x] Compare old/new binaries on frozen inputs and update docs and executable case definitions.

After implementation tasks are complete, native check materializes REQ-cli-028.
Then execute all linked cases, the required Fledge lane and the Rust 1.88 suite.
Those post-materialization executions remain mandatory and are recorded as actual
observations, not task checkboxes that prevent materialization. Human implementation
acceptance, native review/ship and merge are separate gates.
