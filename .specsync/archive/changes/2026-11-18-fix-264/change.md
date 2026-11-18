---
id: fix-264
state: archived
type: bug_fix
base_commit: 9dd28de28015ce6f873e237044cc9650e74a3712
---

# Fix 264

## Intent

fix 264

## Affected Canonical Specs

- `records`

## Acceptance Criteria

- A second migration on one day names its receipt <date>-from-0-5-2.md in the preview, as JSON and as text, with the name part of the plan's digest; its apply writes that receipt, leaves the first receipt byte for byte and removes the 0.5.x record. When another plan's receipt takes the previewed name after the preview, the apply refuses with every file unchanged. An interrupted apply still completes under its own receipt name.

## No-spec Rationale

Not applicable
