---
change: closed-pipe-and-help
artifact: design
---

# Design

- `src/main.rs` (cli): crate-level `print!`, `println!`, `eprint!` and `eprintln!` macros, defined before the modules so every module uses them, write through `std::io::Write` and ignore an `ErrorKind::BrokenPipe` error; any other write error panics as the std macros do. A command whose reader went away therefore finishes and exits with its own code. `run`'s `help --help` check skips the leading global options (`--root <dir>`, `--root=<dir>`, `--redact`, `--session <id>`, `--session=<id>`) before looking for `help` followed by `--help` or `-h`.
- `src/records.rs` (cli): `stdout()` and `stderr()` writers that ignore a broken pipe, for the sites that use `writeln!` on a locked stream.
- `src/change.rs`, `src/ready.rs` (cli): those sites use the writers, so `change ready`, the ship summary and the pass-through end with their own code too.
- Choice (recommended, the only one built): a closed stream ends the command with the code it would have had. Not chosen: dying by SIGPIPE (exit 141, as `cat` does), which could stop a command between two writes of its own records or while it writes to a child's pipe; or always exit 0, which would hide a `doctor` finding or a refusal from a script using `pipefail`.
