# Lesson bundle: fix-268

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Fix 268
- **Kind**: BugFix
- **Specs**: records, cli
- **Paths**: src/checkpoint.rs, tests/leaks.rs, .chough/cases/records.checkpoint-file-leak-guard.cases.toml, changelog.d/2026-11-18-fix-268.md
- **Acceptance**: A token form in a chough checkpoint --file or --design value, such as src/ghp_<token>.txt, refuses the write as refused:leak naming the field (file or design) and the kind token, never echoing the value, with every file unchanged; with --redact the stored path holds <redacted:token> and the command says it redacted. A file or design path that holds no leak is saved as before.

## Evidence

- Verification commit: `4375553d7bbfec467bbd122cecf72f9cc258e1c5`
- Base commit: `be6c9f126e448bd33945a52261d55db2a9677d3d`
- Verified by: `specsync check --spec cli --spec records`

## From the change's context.md

# Context

GitHub #268, from the 0.6.0 swarm report (#273): `chough checkpoint --file "src/ghp_<token>.txt"` exits 0 and commits the token-bearing path to `session.md`'s `files:` list. `chough doctor --leaks` finds it only after the commit. `chough evidence add --artifact` already refuses the same value at the write.

REQ-records-006 requires Chough to check every value it writes, front matter included, before writing a committed record. On main at be6c9f1, `guard_leaks` in `src/checkpoint.rs` checks the objective, intent scope, next action, the story lists, consents, aliases, `pr`, title, summary and pending Timeline values, but not the `files` and `design` references, which hold free-form paths. The criterion, requirement, case, observation and bug references are IDs that `validate_v2` already limits to an ID form, so they cannot hold a path, an email or a token.

This change adds the `files` and `design` references to the guard. It changes no requirement text, command or flag. Leif approved the scope on 2026-11-18, as relayed by his planning session (hc t4c).

## From the change's testing.md

# Testing

`cargo test --locked --test leaks a_token_in_a_file_or_design_path_is_refused_at_the_write` checks that `chough checkpoint --file` and `--design` given a path that holds a GitHub token refuse as `refused:leak: file holds token` and `refused:leak: design holds token`, without echoing the token and with every file unchanged, that `--redact` stores `<redacted:token>` in the path and says it redacted, and that a clean path is still saved. It fails on main. Case `TC-records-a-file-or-design-path-passes-the-leak-guard` binds it. The verify lane's `leaks` step scans this repository's own records.

## Where these lessons go

- `specs/records/context.md`
- `specs/cli/cli.3md`
