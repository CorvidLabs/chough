---
id: chough-init-sets-up-specsync-for-a-new-project-init-scaffold-check-and-adopt-drive-23-slice-6
state: draft
type: feature
base_commit: f41e7ab4c6380698783c1e98a4c6cf2fa96c1ff8
---

# Chough init sets up SpecSync for a new project: init, scaffold, check and adopt (DRIVE-23, slice 6)

## Intent

chough init sets up SpecSync for a new project: init, scaffold, check and adopt (DRIVE-23, slice 6)

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- REQ-cli-062 (new): chough init with at least one --module, in a project with neither .specsync/ nor specs/, runs specsync init, scaffold per module, check and change adopt before its own writes, reporting each under specsync_setup; otherwise it runs none and says why; a failed step or a missing specsync refuses with nothing of Chough's written, naming the steps left (or the install command) and the same chough init, which a test runs. The walkthrough's new-project section uses chough init --module. Five tests in tests/setup.rs fail first, each mutant in testing.md fails one, and fledge run linux-verify passes.

## No-spec Rationale

Not applicable
