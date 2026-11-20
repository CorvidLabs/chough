---
change: closed-pipe-and-help
artifact: testing
---

# Testing

Failing tests first, in `tests/cli.rs`, each red on main 7bf110dc:
1. With standard output closed before the command writes (a pipe whose reader has exited), `next`, `status`, `context`, `bug list --all`, `history show`, `doctor`, `session list`, `timing`, `checkpoint`, `cases list` and `change ready` each exit with the code they exit with when output goes to a file, and standard error holds no `panicked` and no `Broken pipe`.
2. `checkpoint --decision <text>` into a closed pipe saves the decision and exits 0.
3. With standard error closed too (`2>&1` into the closed pipe), a refusal still exits 1 and does not panic.
4. `chough --root . help --help`, `chough --root=. help -h`, `chough --redact help --help` and `chough --session <id> help --help` print the top-level help and exit 0.

Case file `.chough/cases/cli.closed-pipe-and-help.cases.toml`: the closed-pipe cases link REQ-cli-021, the help case REQ-cli-019.

Sweep at main 7bf110dc: no test asserts exit 101 into a pipe or `-V` on a subcommand (`tests/change.rs:1790` asserts a ship does not panic, still true); `tests/cli.rs:3628`'s `help --help` test stays true; no active case is made false; no docs test checks the help-flag sentence; no pinned wording is touched (no requirement changes).
