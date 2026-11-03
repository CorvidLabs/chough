---
change: integrate-the-approved-unlinked-failure-gate-follow-up-from-main
artifact: docs
---

# Integration and verification

Merge the explicit pinned3d00e44 commit, rather than a moving remote ref. Preserve
its source gate implementation and tests exactly unless a genuine integration
conflict requires preserving both approved meanings. Existing source ownership in
the first integration scope continues to cover common case/spec/bundle paths.

Main canonical version19 and PR44 version17 combine at version20, retaining
REQ-cli-008's approved unlinked-failure/not-run text, requirement009's ordering
clause and requirement029's actor/source contract. This updates the earlier plan's
version18 example for the previously pinned8fb0ea7 input; it does not alter any
frozen definition or archived evidence.

Retain original input refs/checkpoints/logs/protected blob identities. Root selected
the PR44 current-cycle group. Preserve all additive references and independent log
occurrences. Check protected bytes and run the imported failure/not-run gate cases,
case-ordering/authority cases, full nine-step Fledge lane and complete Rust1.95 and
Rust1.88 tests without exclusions. Record failures and limits before any rerun.
The prior timing-budget flake stays visible if it recurs.

The worker may commit, check and push normally but does not review/ship. Root
performs native closeout only after qualifying the final integration source. No
existing archive, cold-start material, observation, artifact or approval event is
edited by this scope.
