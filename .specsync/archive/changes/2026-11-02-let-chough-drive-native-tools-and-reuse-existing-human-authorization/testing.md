---
change: let-chough-drive-native-tools-and-reuse-existing-human-authorization
artifact: testing
---

# Testing

The executable upgrade case is TC-cli-agent-native-workflow-upgrade, bound to
workflow_guidance_from_main_825a380_updates_and_preserves_edits in
 tests/workflow_authority.rs. It verifies exact independent predecessor digests,
known-outdated status, real updates to current bytes, and byte-identical preservation
of edited copies. Existing package tests enforce bundle hygiene and packaging.

Run this regression before predecessor registration and retain the failure; rerun
it after registration. No static phrase match is presented as agent compliance proof.
Run fledge fmt/lint/test and the required verify lane, plus Rust1.88, after meaningful
changes. Materialize REQ-cli-029 before linking/running its case. Incomplete tasks
must describe implementation work, not mandatory post-materialization case recording.

Review the shipped text for both branches: existing explicit applicable authority
continues without a repeated question, while absent or exceeded authority asks a
real decision. Native status and check remain required, and implementation acceptance
stays separate. No agent or bot label is used as a human approval source.

Observed platform, CI and omissions are recorded only after they run. A fresh-agent
cold-start experiment, release, deployment and merge are not run by this plan.
