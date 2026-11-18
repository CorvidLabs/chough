- `chough help --help` now prints the top-level help and exits 0, and `chough help
  <command>` prints that command's help, so the CLI reference a user reads is the one
  Chough actually lists; `chough checkpoint --help` now prints clap help text for the
  eleven list-valued flags (`--criterion`, `--requirement`, `--case`, `--design`,
  `--file`, `--observation`, `--bug`, `--decision`, `--question`, `--completed`,
  `--blocker`), and `docs/cli.md` documents the `migrate --json`, `doctor --leaks`,
  `stage help` and top-level `chough help` commands the reference missed (#301, REQ-cli-057,
  DRIVE-17).
