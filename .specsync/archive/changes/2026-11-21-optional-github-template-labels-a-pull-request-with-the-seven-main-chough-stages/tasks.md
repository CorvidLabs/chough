---
change: optional-github-template-labels-a-pull-request-with-the-seven-main-chough-stages
artifact: tasks
---

# Tasks

Write REQ-cli-070 in `deltas/cli.md`.

Add `integrations/github/README.md`, `chough-labels.yml` and `chough-label-prs.sh`, and copy the workflow and the script into `.github/workflows/` and `tools/`.

Say in `integrations/README.md` and `docs/integrations.md` that copying the template is what turns the labels on.

Add `tests/github_labels.rs`, `.chough/cases/cli.optional-github-stage-labels.cases.toml` and `changelog.d/2026-11-21-optional-github-stage-labels.md`.

Scope approval uses the user's 2026-11-21 instruction. Implementation review, push and hosted CI stay separate.
