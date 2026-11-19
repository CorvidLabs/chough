---
change: fix-301
artifact: tasks
---

# Tasks

- [x] Add clap help text to all eleven list-valued checkpoint flags in `src/checkpoint.rs`.
- [x] Document the checkpoint flags and the full CLI reference in `docs/cli.md`.
- [x] Add a `chough help` subcommand and a test so `chough help --help` exits 0 with help text.

Out of scope by Leif's decision (chat, 2026-11-17, "drop change ready from #301"): `change ready` in `docs/cli.md`, since main has no such command.

Human gates (not checkboxes): scope approval, implementation review, merge.
