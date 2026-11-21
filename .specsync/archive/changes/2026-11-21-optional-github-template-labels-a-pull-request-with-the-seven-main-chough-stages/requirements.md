---
change: optional-github-template-labels-a-pull-request-with-the-seven-main-chough-stages
artifact: requirements
---

# Requirements

REQ-cli-070 (added): an optional `integrations/github/` template labels a pull request with the seven main stages, in order, with the rainbow colors. Chough's own workflow and script are byte-identical copies. `chough init` does not install the template, and `src/` does not call GitHub. A pull request is labeled only when the sessions it introduces share one main stage. The full text is in `deltas/cli.md`.
