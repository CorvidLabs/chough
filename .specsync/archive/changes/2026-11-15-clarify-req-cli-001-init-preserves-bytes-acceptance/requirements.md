---
change: clarify-req-cli-001-init-preserves-bytes-acceptance
artifact: requirements
---

# Requirements

The change modifies REQ-cli-001 in `specs/cli/requirements.md` to clarify its
acceptance line. The requirement itself is unchanged; only its acceptance
criteria are restated to name both preservation behaviors explicitly:

- repeated init preserves edited session and skill bytes
- missing bundle files can be added without overwriting neighbors
- a session whose files were edited since init stays byte-identical on later init

No new requirements are added and none are removed.
