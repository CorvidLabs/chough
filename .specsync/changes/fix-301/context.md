---
change: fix-301
artifact: context
---

# Context

The `chough checkpoint` command's list-valued flags (`--criterion`, `--requirement`, `--case`, `--design`, `--file`, `--observation`, `--bug`, `--decision`, `--question`, `--completed`, `--blocker`) are defined without clap doc comments, so `chough checkpoint --help` prints no help text for them. `docs/cli.md` also does not document these flags, nor does it cover `change ready`, `doctor --leaks`, `migrate --json`, stage help, or the top-level `chough help` command. A user cannot tell what each flag or command does from the CLI or the reference. Separately, `chough help --help` exits 2 because there is no `help` subcommand; it should exit 0 with help text.
