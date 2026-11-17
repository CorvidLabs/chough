---
change: fix-301
artifact: context
---

# Context

The `chough checkpoint` command's list-valued flags (`--criterion`, `--requirement`, `--case`, `--design`, `--file`, `--observation`, `--bug`) are defined without clap doc comments, so `chough checkpoint --help` prints no help text for them. `docs/cli.md` also does not document these flags. A user cannot tell what each flag does or what values it takes from the CLI or the reference.
