---
change: optional-github-template-labels-a-pull-request-with-the-seven-main-chough-stages
artifact: docs
---

# Docs

`integrations/github/README.md` tells a project how to copy the two files and which line to change when the default branch is not `main`.

`integrations/README.md` gains a short section pointing at that template, and keeps the sentences REQ-cli-065 requires.

`docs/integrations.md` says the template is optional and that Chough does not call GitHub. The existing "GitHub plugin" row stays as it is. That row is the Fledge plugin, not this template.

`changelog.d/2026-11-21-optional-github-stage-labels.md` records the change for the next release. `hi/life.md` gains LIFE-31.
