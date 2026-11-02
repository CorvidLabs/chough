## ADDED

### REQUIREMENT REQ-cli-031

New-project native setup hint. When a native SpecSync configuration exists but `.specsync/sdd.json` is absent or sets `enabled` to false, `chough next` SHALL report the approved-delta line as ``not checked: native SDD is disabled; run `specsync change adopt` to gate stages``, and the `ungated:sdd-disabled` gate detail shown by `next`, `status` and stage moves SHALL name `specsync change adopt`. Without a native configuration the line SHALL remain `not checked: no native lifecycle` and the gate `ungated:not-configured`; an invalid policy SHALL keep `refused:sdd-policy-invalid` and `not checked: SDD policy invalid`. The helper SHALL NOT run `specsync change adopt` or write the policy. The walkthrough SHALL begin with a new-project setup path covering a minimal HI criteria file, `specsync init` and scaffolded module files, `specsync change adopt`, and what `ungated:sdd-disabled` means. Trace: LIFE-9, LIFE-6, LIFE-7.

Acceptance Criteria
- With SpecSync configured and `.specsync/sdd.json` absent or `enabled: false`, `chough next` (JSON and text) shows the adopt hint on the approved-delta line, and the `ungated:sdd-disabled` gate detail in `next` and `status` names `specsync change adopt`.
- Without a SpecSync configuration the line stays `not checked: no native lifecycle` with no hint, and an invalid policy still reads `not checked: SDD policy invalid` with no hint.
- The walkthrough begins with the new-project setup path.
