---
change: an-open-session-sets-the-stage-label-when-archived-sessions-are-also-on-the-pull-request
artifact: design
---

# Design

Nothing visual changes. There is no HTML mockup. The label is a GitHub label name taken from the session `stage:` field.

When a pull request introduces an open session, that session's stage is the label and archived stories on the same pull request are ignored. When the pull request introduces no open session, the archived stories supply the stage. One distinct main stage among the chosen sessions is applied. Several stages among them, or a stage outside the main line, clears every `chough-stage:*` label.
