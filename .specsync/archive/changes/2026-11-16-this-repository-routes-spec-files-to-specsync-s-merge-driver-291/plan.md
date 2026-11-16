---
change: this-repository-routes-spec-files-to-specsync-s-merge-driver-291
artifact: plan
---

# Plan

## Approach

Route this repository's module contracts through SpecSync's ancestor-aware merge driver. `.gitattributes` sends `specs/**/*.spec.md` and `specs/**/requirements.md` to `merge=specsync`. `AGENTS.md` and `docs/cli.md` name the one-time `specsync merge --install`. A package test checks both lines. The user-facing note is a `changelog.d/` fragment, because #290 assembles those at release. No requirement text changes.

## Out of scope

Teaching `chough init` or `chough doctor` to install the SpecSync driver. That needs its own wording. Making GitHub's merge button run the driver. It does not run custom drivers. Changing any requirement.

## Steps

1. Declare the two attribute lines, the setup note, the package test and the case.
2. Retarget the change base to current main, so readiness sees this change's files and not every file main added since the old base.
3. Replace the Unreleased bullets a union merge put back into `CHANGELOG.md` with one fragment.
4. Record the scope decision, the package-test evidence, the implementation review and the ship on the existing pull request.

## Risks

GitHub still reports a conflict when only the driver can merge two spec rows, because the hosted merge does not run it. Local merges do. Readiness diffs from `base_commit`, so an old base lists files this change never touched. A union merge of `CHANGELOG.md` can copy already released notes back into Unreleased.

## Constraints consulted

DRIVE-12, so parallel work does not hand-resolve files the tools write. REQ-cli-047, this repository's merge configuration. SpecSync 6.3.0 `specsync merge --install`. #290, one fragment per change. The lesson that GitHub runs no custom merge driver.

Relative size: small, one module, configuration and one test. Recorded history is not used as a deadline.
