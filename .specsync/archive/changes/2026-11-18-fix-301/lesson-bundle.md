# Lesson bundle: fix-301

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: CLI reference and help text cover every command and flag (#301)
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: docs/cli.md, src/checkpoint.rs, src/main.rs, tests/cli.rs, changelog.d/
- **Acceptance**: docs/cli.md documents the checkpoint command's eleven list-valued flags (--criterion, --requirement, --case, --design, --file, --observation, --bug, --decision, --question, --completed, --blocker), adds change ready to the change row and to the SpecSync change steps section, adds doctor --leaks to the doctor row, adds migrate --json to the migrate row, and adds stage help and the top-level chough help command; chough checkpoint --help prints help text for each of the eleven list-valued flags; chough help --help exits 0 with help text, covered by a test

## Evidence

- Verification commit: `d057d40e3f4fecb1af8e52dcb12dfb7c7e43c01a`
- Base commit: `0fd9fd229744da039e9d43a5ce1df6dfe5ddc8d7`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

The `chough checkpoint` command's list-valued flags (`--criterion`, `--requirement`, `--case`, `--design`, `--file`, `--observation`, `--bug`, `--decision`, `--question`, `--completed`, `--blocker`) are defined without clap doc comments, so `chough checkpoint --help` prints no help text for them. `docs/cli.md` also does not document these flags, nor does it cover `change ready`, `doctor --leaks`, `migrate --json`, stage help, or the top-level `chough help` command. A user cannot tell what each flag or command does from the CLI or the reference. Separately, `chough help --help` exits 2 because there is no `help` subcommand; it should exit 0 with help text.

## From the change's testing.md

# Testing

No product test beyond the help-command test. Verify `chough checkpoint --help` prints help text for each of the eleven list-valued flags, that `docs/cli.md` documents them, and that `chough help --help` exits 0 with help text (covered by a test).

## Where these lessons go

- `specs/cli/cli.3md`
