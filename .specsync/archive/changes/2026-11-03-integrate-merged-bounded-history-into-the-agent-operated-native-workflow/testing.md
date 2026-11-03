---
change: integrate-merged-bounded-history-into-the-agent-operated-native-workflow
artifact: testing
---

# Testing

Run the existing package checkpoint safe-update regression and both workflow_authority regressions, plus all affected history/retention cases and the source ordering case. Keep case execution outside tasks.md checkboxes. Complete implementation tasks and run the scoped check before validating or recording cases. Required gates are all9 Fledge verify steps and every Rust test on1.95 and1.88, with actual counts measured and no exclusions. Verify149 case IDs, all protected parent blob identities, exact requirements and all45 predecessor registrations. Retain new raw logs and source identities; never modify prior proof.

Limits: local macOS execution is distinct from hosted CI; source/package checks are not fresh-agent behavior, Kimi is not run, existing timing flake remains open unless reproduced and genuinely fixed, and no live project adoption or cleanup occurs. Root alone records native review/ship/done.
