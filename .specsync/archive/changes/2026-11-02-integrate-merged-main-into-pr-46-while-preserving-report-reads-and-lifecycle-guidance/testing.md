---
change: integrate-merged-main-into-pr-46-while-preserving-report-reads-and-lifecycle-guidance
artifact: testing
---

# Testing

Retain all incoming package hashes and tests and all report read-budget, per-request invalidation, trace parity and typed-error regressions. Run TC-cli-report-case-read-budget, TC-cli-report-read-invalidation and TC-cli-report-trace-parity on the integrated source. Run the complete nine-step Fledge verify lane with Rust 1.95 and all tests with Rust 1.88 in a separate temporary target. Compare immutable proof files from both parents byte-for-byte; verify checkpoint additive arrays and every legacy transition occurrence survive. Native scoped verification and audit must pass and be current before root's closeout. Hosted checks are observed separately after push; an active integration change may keep Archive red until root records review and ship. Preserve actual failures rather than weakening assertions.
