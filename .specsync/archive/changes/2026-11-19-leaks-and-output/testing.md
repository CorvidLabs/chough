---
change: leaks-and-output
artifact: testing
---

# Testing

Each issue gets a test that fails on main first:
- #364: `--answer` with leaking values refuses naming `answer`, `--redact` stores the redacted form, and a 4,097-byte answer is refused.
- #365: a committed conflict whose `>>>>>>>` line names a token-holding branch gives findings whose `marker` is `>>>>>>>`, and no output holds the branch name; the write refusal names only the marker.
- #367: each echo site, given a value holding a token or a control character, refuses without it and names the field and what it holds; a clean unknown value is still named.
- #368: unit tests for each new form in `src/leaks.rs`, and `session new --module` and `evidence add --redact` through the CLI.
- #379: unit tests for the account user and the system-root rule.
- #380: `gate record`'s refusal holds `Error:` once.
