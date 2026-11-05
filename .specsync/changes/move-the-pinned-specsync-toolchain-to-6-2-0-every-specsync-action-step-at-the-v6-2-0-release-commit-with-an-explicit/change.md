---
id: move-the-pinned-specsync-toolchain-to-6-2-0-every-specsync-action-step-at-the-v6-2-0-release-commit-with-an-explicit
state: implementing
type: operations
base_commit: ded0e9ef4ec27f634d5175c4601f65b7e5f5b694
---

# Move the pinned SpecSync toolchain to 6.2.0: every SpecSync Action step at the v6.2.0 release commit with an explicit version 6.2.0, and every pin in README, AGENTS, CI scripts and docs kept in agreement

## Intent

Move the pinned SpecSync toolchain to 6.2.0: every SpecSync Action step at the v6.2.0 release commit with an explicit version 6.2.0, and every pin in README, AGENTS, CI scripts and docs kept in agreement

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- Every SpecSync Action step in .github/workflows/ci.yml (the Archive gate and the Fledge / HI / SpecSync and SpecSync Archive and CI gate fixtures jobs) uses CorvidLabs/spec-sync@40dcf33a6b0f0b55dcb7f17e8e98be9d55fa2c56 # v6.2.0 with version '6.2.0' passed explicitly; README.md (install command), AGENTS.md, .github/scripts/check-specsync-archive.py, .github/scripts/test-specsync-archive.py, docs/ci.md, docs/integrations.md and .specsync/config.toml name SpecSync 6.2.0 and docs/DECISIONS.md records the move; no other current 6.1.x pin remains while recorded exercises keep the versions they observed; tests/package.rs tool_and_rust_version_pins_agree passes on 6.2.0; fledge lanes run verify and the CI gate fixtures pass locally with SpecSync 6.2.0; and the hosted PR checks pass, with SpecSync step durations compared against the latest main run

## No-spec Rationale

Toolchain pin move only: REQ-cli-019 already requires the README, agent instructions and CI to name agreeing versions and names no version number; Chough behavior and requirements are unchanged.
