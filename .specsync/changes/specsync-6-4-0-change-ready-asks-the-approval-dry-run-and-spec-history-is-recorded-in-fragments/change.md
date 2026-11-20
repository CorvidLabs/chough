---
id: specsync-6-4-0-change-ready-asks-the-approval-dry-run-and-spec-history-is-recorded-in-fragments
state: implementing
type: migration
base_commit: de49485fc5fb93f05bb15db7cd471023beebd8a8
---

# SpecSync 6.4.0: change ready asks the approval dry run, and spec history is recorded in fragments

## Intent

SpecSync 6.4.0: change ready asks the approval dry run, and spec history is recorded in fragments

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- The criteria in deltas/cli.md (REQ-cli-067): ready names whatever SpecSync's approval dry run refuses, drops its delta-set rule, leaves SpecSync's own untracked files and another active change's paths out, and touches no .specsync/hashes.json; the pins name 6.4.0 and the spec merge line is gone.

## No-spec Rationale

Not applicable
