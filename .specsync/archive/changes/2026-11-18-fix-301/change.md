---
id: fix-301
state: archived
type: bug_fix
base_commit: 0fd9fd229744da039e9d43a5ce1df6dfe5ddc8d7
---

# CLI reference and help text cover every command and flag (#301)

## Intent

Add clap help text to all eleven list-valued checkpoint flags (--criterion, --requirement, --case, --design, --file, --observation, --bug, --decision, --question, --completed, --blocker), document the full CLI reference in docs/cli.md (checkpoint flags, change ready, doctor --leaks, migrate --json, stage help, top-level chough help), and make chough help --help exit 0 with help text, covered by a test.

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- docs/cli.md documents the checkpoint command's eleven list-valued flags (--criterion, --requirement, --case, --design, --file, --observation, --bug, --decision, --question, --completed, --blocker), adds change ready to the change row and to the SpecSync change steps section, adds doctor --leaks to the doctor row, adds migrate --json to the migrate row, and adds stage help and the top-level chough help command; chough checkpoint --help prints help text for each of the eleven list-valued flags; chough help --help exits 0 with help text, covered by a test

## No-spec Rationale

No canonical spec or requirement wording changes. This is a CLI bug fix: chough help --help currently exits 2 (no help subcommand exists) and should exit 0 with help text, so it changes CLI behavior (exit code and help output) only. A changelog fragment at changelog.d/<YYYY-MM-DD>-fix-301.md accompanies this change, per repo convention.
