---
change: new-project-setup-path-and-sdd-off-hint-document-hi-specsync-init-scaffold-and-specsync-change-adopt-for-a-new-project
artifact: design
---

# Design

- `src/stage.rs` `delta_summary`: split the `NotConfigured | Disabled` arm. `NotConfigured` keeps "not checked: no native lifecycle", and `Disabled` gets the adopt hint.
- `src/stage.rs` Disabled gate detail: append "Run `specsync change adopt` to gate stages." The label stays `ungated:sdd-disabled` and the move stays allowed, so this changes reporting only.
- Docs: add a "New project" section at the top of `docs/walkthrough.md`, replace the legacy bug IDs with the `BUG-<slug>-<24 hex>` form, add a version note in `docs/exercises/feature-bug-loop.md` stating only what was observed, add a `docs/cli.md` note, add one README pointer line under Start here, and add a CHANGELOG entry.
