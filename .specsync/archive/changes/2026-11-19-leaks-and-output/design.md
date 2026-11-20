---
change: leaks-and-output
artifact: design
---

# Design

- `src/leaks.rs`: new forms in the existing hand-written matchers: a quoted key before `:` in `credentials`, the `secret_key` and `secret_access_key` keys, the `xapp-` and `hf_` token prefixes, `~<user>/` paths. `Local` reads the user from the account (`getpwuid_r`), falling back to `USER`/`LOGNAME`, and on macOS adds the LocalHostName from the system preferences file. Each host name is also kept as `<name>.local`, and a dotted host matches with hyphens around it. A user name is no path leak as the first component of an absolute path that is a system root. `shown(local, field, value)` returns the value when it holds no match and no control character, and otherwise says what the field holds, for the error echo sites.
- `src/checkpoint.rs`: `--answer` goes through `ensure_text`, and the save's guard visits every answer; `modules` are checked as IDs.
- `src/records.rs`: a conflict marker is named by its seven characters, so every refusal and finding built on the scan names only that.
- `src/main.rs`, `src/sessions.rs`, `src/bugs.rs`, `src/humans.rs`: the echoing errors use `shown`.
- `src/evidence.rs`: `evidence add --redact` prints its count, as the session save does.
- `src/gates.rs`: the refused close's own `Error: ` prefix is dropped before it is quoted.
