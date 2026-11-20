---
change: closed-pipe-and-help
artifact: context
---

# Context

Issues #383 (its closed-pipe half, s7-F5; the Unicode half is #410 in 0.6.3) and #399 (the unaddressed review findings on #309; Leif: fix in 0.6.2).

Measured on main 7bf110dc (debug build), as Leif asked before fixing, in a project with an open session:
- #383: with standard output closed (`| true`), `next`, `status`, `context`, `bug list --all`, `history show`, `doctor`, `session list`, `timing`, `checkpoint` and `cases list` each exit 101 with "panicked … failed printing to stdout: Broken pipe"; `--help` and `help next` (clap's own output) exit 0. `checkpoint --decision` saves its decision and then exits 101 the same way. `change ready`, the ship summary and SpecSync's pass-through write with `writeln!(…)?` and end with `Error: Broken pipe` and exit 1 instead. With `| head -1` on the real projects the output fitted the pipe and nothing failed, which is why one tester could not reproduce it.
- #399: `chough --root . help --help`, `chough --root=. help -h` and `chough --redact help --help` exit 2 (`unrecognized subcommand`); `chough help --help` exits 0. `chough checkpoint -V`, `next -V` and `help -V` exit 2, while docs/cli.md says `-V` is a help flag on every command.
