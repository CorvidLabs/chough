---
id: new-project-setup-path-and-sdd-off-hint-document-hi-specsync-init-scaffold-and-specsync-change-adopt-for-a-new-project
state: implementing
type: feature
base_commit: 825a3805530456005e361dfa6d96d93eee52cd09
---

# New-project setup path and SDD-off hint: document HI, specsync init/scaffold and specsync change adopt for a new project, and have chough next say to run specsync change adopt when SpecSync is configured but SDD is off (issue 40)

## Intent

New-project setup path and SDD-off hint: document HI, specsync init/scaffold and specsync change adopt for a new project, and have chough next say to run specsync change adopt when SpecSync is configured but SDD is off (issue 40)

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- When SpecSync is configured but SDD is disabled or not adopted, chough next reports the approved-delta line as `not checked: native SDD is disabled; run specsync change adopt to gate stages`, and the ungated:sdd-disabled gate detail (shown by stage advance and status) names `specsync change adopt`; not checked: no native lifecycle stays when SpecSync is not configured, and a malformed policy is unchanged
- docs/walkthrough.md starts with a New project section covering a minimal hi/<family>.md, specsync init and scaffold with files filled in, specsync change adopt and what ungated:sdd-disabled means; README Start here gains only a one-line pointer to it
- docs/walkthrough.md shows the current BUG-<slug>-<24 hex> ID form, and docs/exercises/feature-bug-loop.md states the SpecSync version only as observed
- A Linux reproduction of issue 40 and the new tests fail before the change and pass after it; fledge lanes run verify passes; hosted CI is reported only as observed

## No-spec Rationale

Not applicable
