---
change: new-project-setup-path-and-sdd-off-hint-document-hi-specsync-init-scaffold-and-specsync-change-adopt-for-a-new-project
artifact: testing
---

# Testing

- The new test runs in a fixture with stubbed `hi` and `specsync`. It checks an unconfigured project (old line, no hint), a configured project with no policy, `enabled: false` (hint in the JSON and text `next`, gate detail, and `status`) and an invalid policy (unchanged, no hint). It fails before the change.
- Case `TC-cli-sdd-off-names-specsync-change-adopt` (cargo-test, target `stages`) is linked to REQ-cli-031.
- The Linux reproduction is `.chough/artifacts/bug40-repro.log`. `fledge lanes run verify` covers the full suite, lint, the walkthrough-backed exercise and audit. Hosted CI is reported only as observed.
