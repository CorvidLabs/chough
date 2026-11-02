---
change: new-project-setup-path-and-sdd-off-hint-document-hi-specsync-init-scaffold-and-specsync-change-adopt-for-a-new-project
artifact: requirements
---

# Requirements

ADD REQ-cli-031 (see `deltas/cli.md`):

New-project native setup hint. When a native SpecSync configuration exists but `.specsync/sdd.json` is absent or sets `enabled` to false, `chough next` SHALL report the approved-delta line as ``not checked: native SDD is disabled; run `specsync change adopt` to gate stages``, and the `ungated:sdd-disabled` gate detail shown by `next`, `status` and stage moves SHALL name `specsync change adopt`. Without a native configuration the line SHALL remain `not checked: no native lifecycle` and the gate `ungated:not-configured`; an invalid policy SHALL keep `refused:sdd-policy-invalid` and `not checked: SDD policy invalid`. The helper SHALL NOT run `specsync change adopt` or write the policy. The walkthrough SHALL begin with a new-project setup path covering a minimal HI criteria file, `specsync init` and scaffolded module files, `specsync change adopt`, and what `ungated:sdd-disabled` means. Trace: LIFE-9, LIFE-6, LIFE-7.
