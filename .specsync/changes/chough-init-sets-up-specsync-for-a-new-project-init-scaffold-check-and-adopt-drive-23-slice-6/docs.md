---
change: chough-init-sets-up-specsync-for-a-new-project-init-scaffold-check-and-adopt-drive-23-slice-6
artifact: docs
---

# Docs

- `docs/walkthrough.md`: the "New project" section runs `chough init --objective … --module shipping`, which sets up SpecSync, instead of `specsync init`, `scaffold` and `change adopt`. Filling in the scaffolded files stays a step.
- `README.md` and `docs/cli.md`: the `init` row says it sets up SpecSync for a new project (REQ-cli-062).
- `CHANGELOG.md`: one Unreleased entry citing DRIVE-23.
