---
change: a-pull-request-that-already-has-its-stage-label-keeps-that-label
artifact: design
---

# Design

Nothing visual changes. There is no HTML mockup.

When the pull request already has `chough-stage:<stage>` for the one main stage, that label stays. The script does not remove it and does not add it. Any other `chough-stage:*` label is still removed. When there is no stage to apply, every `chough-stage:*` label is still removed.

The same edit is copied into both script files so they stay byte-identical. The workflow is unchanged.
