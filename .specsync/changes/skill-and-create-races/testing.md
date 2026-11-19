---
change: skill-and-create-races
artifact: testing
---

# Testing

Failing tests first, each red on main first:
- `tests/cli.rs`: a CRLF copy of each bundle file reads as its LF state and is refreshed with CRLF; with `.codex/skills` a symlink to `../.agents/skills`, plain status and update exit 0, report codex `uninspected` relative and update `.agents`, while `--host codex` refuses; a missing file in a read-only folder is refused naming it; no output holds an absolute path.
- `tests/parallel.rs` (new): eight `skill update` runs at once over missing files and six `init --objective` runs at once in a fresh repository, repeated, with every run exiting as REQ-cli-001 and REQ-cli-016 say and no `Condition failed` or `File exists` in any output.
- `tests/doctor.rs`: an uncommitted session with an unknown stage, an uncommitted 0-byte evidence entry and an uncommitted cut-off bug each give `unreadable` with the file and reason; a file under `.chough/local/` gives none.

One case file per module: `.chough/cases/cli.skill-and-create-races.cases.toml` (REQ-cli-016, REQ-cli-001) and `.chough/cases/history.skill-and-create-races.cases.toml` (REQ-history-016).
