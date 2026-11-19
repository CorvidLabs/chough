---
change: pin-the-edited-workflow-fast-path-in-the-skill-release-corpus
artifact: context
---

# Context

Editing the workflow fast path in `skills/chough-workflow/references/stages.md` made `tests/skill_releases.rs` fail in CI. The current bundle pin still named the previous bytes of that file. Product on Ubuntu, MSRV, and the Fledge verify lane all failed on `613f7bdb` with `current bundle changed: references/stages.md`.

The historical release blobs stay as they are. Only the current pin and the manifest digest constant move to the edited file.
