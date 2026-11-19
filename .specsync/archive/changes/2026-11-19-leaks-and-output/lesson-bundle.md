# Lesson bundle: leaks-and-output

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Leaks and output
- **Kind**: BugFix
- **Specs**: records, cli, history
- **Paths**: src/leaks.rs, src/checkpoint.rs, src/records.rs, src/doctor.rs, src/cases.rs, src/migrate.rs, src/gates.rs, src/main.rs, src/sessions.rs, src/bugs.rs, src/evidence.rs, src/humans.rs, src/markdown.rs, tests/change.rs, tests/bugs.rs, tests/cases.rs, tests/checkpoint.rs, tests/git_merge.rs, tests/humans.rs, tests/doctor.rs, tests/migrate.rs, tests/migrate_bugs.rs, tests/leaks.rs, tests/session_pin.rs, tests/records.rs, tests/sessions.rs, tests/stages.rs, tests/gates.rs, tests/evidence.rs, docs/cli.md, changelog.d/2026-11-19-leaks-and-output.md, .chough/cases/records.leaks-and-output.cases.toml, .chough/cases/cli.leaks-and-output.cases.toml, .chough/cases/history.leaks-and-output.cases.toml
- **Acceptance**: checkpoint --resolve-question --answer is leak-checked, redacted under --redact and bounded at 4096 bytes (#364). Conflict-marker refusals and doctor findings name only the marker's seven characters (#365). An error shows a value a person gave only when it holds no leak and no control character, otherwise it names the field and what it holds (#367). The leak guard adds password JSON keys, SECRET_KEY and AWS_SECRET_ACCESS_KEY assignments, xapp- and hf_ tokens, ~user/ paths, the macOS LocalHostName and its .local name, dotted host names inside hyphenated build names, and checks session new --module; evidence add --redact reports its count (#368). The user name comes from the account, and a system root such as /dev/ is never a user-name path (#379). gate record's refusal says Error: once (#380).

## Evidence

- Verification commit: `8ab22af2450048896b18c6f5f413fc8dae407aea`
- Base commit: `bd77bbd15f0e373776192a7d058a09c70fdbc00e`
- Verified by: `specsync check --spec cli --spec history --spec records`

## From the change's context.md

# Context

The first batch of the 0.6.2 bug round, leaks and output, from the 0.6.2 candidate swarm's s6 run (leak guard red team and hostile input). The run's notes are on Leif's Mac.

- #364 (HIGH, s6-F1): `checkpoint --resolve-question … --answer` stores the answer in the session's answers map, which the save's leak guard never visits, so every leak kind is committed, `--redact` changes nothing, and there is no 4,096-byte bound.
- #365 (s6-F3): doctor's `conflict_markers` finding, and every other marker refusal, prints the whole marker line, which for a merge, rebase or cherry-pick carries a branch name or a commit subject.
- #367 (s6-F4): errors echo the offending value: the session pin (`CHOUGH_SESSION`, `--session`, `session use`), `bug update --related`/`--duplicate-of`/`--observation`, `bug add --observation` and `gate record --decided-by`, for all 25 leak kinds, and with control characters raw.
- #368 (s6-F5): leak forms still missing since the 0.6.1 swarm.
- #379 (s6-F8): the user name comes only from `USER`/`LOGNAME`, so it is unchecked when they are unset, and `USER=dev` makes every `/dev/` path a leak.
- #380 (s6-F9): `gate record`'s refusal says `Error:` twice.

#366, escaping control characters on print, is a later batch, since it touches every printer.

## From the change's design.md

# Design

- `src/leaks.rs`: new forms in the existing hand-written matchers: a quoted key before `:` in `credentials`, the `secret_key` and `secret_access_key` keys, the `xapp-` and `hf_` token prefixes, `~<user>/` paths. `Local` reads the user from the account (`getpwuid_r`), falling back to `USER`/`LOGNAME`, and on macOS adds the LocalHostName from the system preferences file. Each host name is also kept as `<name>.local`, and a dotted host matches with hyphens around it. A user name is no path leak as the first component of an absolute path that is a system root. `shown(local, field, value)` returns the value when it holds no match and no control character, and otherwise says what the field holds, for the error echo sites.
- `src/checkpoint.rs`: `--answer` goes through `ensure_text`, and the save's guard visits every answer; `modules` are checked as IDs.
- `src/records.rs`: a conflict marker is named by its seven characters, so every refusal and finding built on the scan names only that.
- `src/main.rs`, `src/sessions.rs`, `src/bugs.rs`, `src/humans.rs`: the echoing errors use `shown`.
- `src/evidence.rs`: `evidence add --redact` prints its count, as the session save does.
- `src/gates.rs`: the refused close's own `Error: ` prefix is dropped before it is quoted.

## From the change's testing.md

# Testing

Each issue gets a test that fails on main first:
- #364: `--answer` with leaking values refuses naming `answer`, `--redact` stores the redacted form, and a 4,097-byte answer is refused.
- #365: a committed conflict whose `>>>>>>>` line names a token-holding branch gives findings whose `marker` is `>>>>>>>`, and no output holds the branch name; the write refusal names only the marker.
- #367: each echo site, given a value holding a token or a control character, refuses without it and names the field and what it holds; a clean unknown value is still named.
- #368: unit tests for each new form in `src/leaks.rs`, and `session new --module` and `evidence add --redact` through the CLI.
- #379: unit tests for the account user and the system-root rule.
- #380: `gate record`'s refusal holds `Error:` once.

## Where these lessons go

- `specs/records/context.md`
- `specs/cli/cli.3md`
- `specs/history/context.md`
