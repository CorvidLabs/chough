---
change: fix-301
artifact: requirements
---

# Requirements

No canonical spec or requirement changes; this change is `--no-spec-change`. The user-facing requirements are:
- all eleven list-valued checkpoint flags carry clap help text and are documented in `docs/cli.md`;
- `docs/cli.md` documents `change ready`, `doctor --leaks`, `migrate --json`, stage help and the top-level `chough help` command;
- `chough help --help` exits 0 with help text, covered by a test.
