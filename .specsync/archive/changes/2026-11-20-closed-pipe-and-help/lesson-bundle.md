# Lesson bundle: closed-pipe-and-help

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Closed pipe and help
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: src/main.rs, src/records.rs, src/change.rs, src/ready.rs, tests/cli.rs, docs/cli.md, changelog.d/2026-11-20-closed-pipe-and-help.md, .chough/cases/cli.closed-pipe-and-help.cases.toml
- **Acceptance**: Every command whose standard output or standard error is closed, as in chough next | true or chough next | head -1, stops printing and exits with the code it would have exited with, never 101 and never a panic or a Broken pipe message; a write command still saves (#383). chough --root . help --help, chough --root=. help -h and chough --redact help --help print the top-level help and exit 0, as chough help --help does, and docs/cli.md names --help and -h as the help flags and chough -V and --version as the top-level version flags (#399).

## Evidence

- Verification commit: `de0f1dc7ec89772141d8f938e73cb490dce2ffef`
- Base commit: `7bf110dcca70bdd3565007086aee40a286f40eff`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

Issues #383 (its closed-pipe half, s7-F5; the Unicode half is #410 in 0.6.3) and #399 (the unaddressed review findings on #309; Leif: fix in 0.6.2).

Measured on main 7bf110dc (debug build), as Leif asked before fixing, in a project with an open session:
- #383: with standard output closed (`| true`), `next`, `status`, `context`, `bug list --all`, `history show`, `doctor`, `session list`, `timing`, `checkpoint` and `cases list` each exit 101 with "panicked … failed printing to stdout: Broken pipe"; `--help` and `help next` (clap's own output) exit 0. `checkpoint --decision` saves its decision and then exits 101 the same way. `change ready`, the ship summary and SpecSync's pass-through write with `writeln!(…)?` and end with `Error: Broken pipe` and exit 1 instead. With `| head -1` on the real projects the output fitted the pipe and nothing failed, which is why one tester could not reproduce it.
- #399: `chough --root . help --help`, `chough --root=. help -h` and `chough --redact help --help` exit 2 (`unrecognized subcommand`); `chough help --help` exits 0. `chough checkpoint -V`, `next -V` and `help -V` exit 2, while docs/cli.md says `-V` is a help flag on every command.

## From the change's design.md

# Design

- `src/main.rs` (cli): crate-level `print!`, `println!`, `eprint!` and `eprintln!` macros, defined before the modules so every module uses them, write through `std::io::Write` and ignore an `ErrorKind::BrokenPipe` error; any other write error panics as the std macros do. A command whose reader went away therefore finishes and exits with its own code. `run`'s `help --help` check skips the leading global options (`--root <dir>`, `--root=<dir>`, `--redact`, `--session <id>`, `--session=<id>`) before looking for `help` followed by `--help` or `-h`.
- `src/records.rs` (cli): `stdout()` and `stderr()` writers that ignore a broken pipe, for the sites that use `writeln!` on a locked stream.
- `src/change.rs`, `src/ready.rs` (cli): those sites use the writers, so `change ready`, the ship summary and the pass-through end with their own code too.
- Choice (recommended, the only one built): a closed stream ends the command with the code it would have had. Not chosen: dying by SIGPIPE (exit 141, as `cat` does), which could stop a command between two writes of its own records or while it writes to a child's pipe; or always exit 0, which would hide a `doctor` finding or a refusal from a script using `pipefail`.

## From the change's testing.md

# Testing

Failing tests first, in `tests/cli.rs`, each red on main 7bf110dc:
1. With standard output closed before the command writes (a pipe whose reader has exited), `next`, `status`, `context`, `bug list --all`, `history show`, `doctor`, `session list`, `timing`, `checkpoint`, `cases list` and `change ready` each exit with the code they exit with when output goes to a file, and standard error holds no `panicked` and no `Broken pipe`.
2. `checkpoint --decision <text>` into a closed pipe saves the decision and exits 0.
3. With standard error closed too (`2>&1` into the closed pipe), a refusal still exits 1 and does not panic.
4. `chough --root . help --help`, `chough --root=. help -h`, `chough --redact help --help` and `chough --session <id> help --help` print the top-level help and exit 0.

Case file `.chough/cases/cli.closed-pipe-and-help.cases.toml`: the closed-pipe cases link REQ-cli-021, the help case REQ-cli-019.

Sweep at main 7bf110dc: no test asserts exit 101 into a pipe or `-V` on a subcommand (`tests/change.rs:1790` asserts a ship does not panic, still true); `tests/cli.rs:3628`'s `help --help` test stays true; no active case is made false; no docs test checks the help-flag sentence; no pinned wording is touched (no requirement changes).

## Where these lessons go

- `specs/cli/cli.3md`
