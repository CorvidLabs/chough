---
change: control-and-echo
artifact: testing
---

# Testing

Failing tests first, each red on the base first:
- `tests/output_safety.rs` (new): (1) the write sweep, ESC, BEL, DEL, U+009B, U+202E and U+2066 in each checked field, refused as `refused:control-character` with every file byte-identical, `--redact` too, and a line feed and a tab accepted where a field takes one, CRLF stored as LF; (2) the print sweep, a fixture holding each control character by hand in every record kind, then every command in `chough --help`'s tree, walked from the help output so a new command joins by itself, in text and JSON with the fixture's IDs, with no raw control character and JSON that parses; (3) the echo sweep, every value slot of every command given a token and an escape sequence, with neither in any output (#367).
- `tests/leaks.rs`: `file:///opt/<user>/x` refused, `https://…/<user>/…` written; `USER`, `LOGNAME` and `USERNAME` set to three names.
- `tests/checkpoint.rs`: each `--pr` form accepted or refused as REQ-cli-007 lists; `--pr ""` removes `pr`; YAML reserved scalars read back as text.
- `tests/next.rs`: the `PR:` line and the JSON field; a `--no-native-change` reason with a line feed shown as `\n`.
- `tests/journal.rs`: a stored `pr` that is neither form is left out of the post's links; a post and a session from an earlier release holding each control character give a feed that parses as XML, an index and a story with none raw.
- `tests/history.rs`: `correct` with a token or an escape sequence names what it holds.
- `tests/timing.rs`: a reason holding ESC and U+202E printed escaped, text and JSON.

One case file per module: `.chough/cases/{records,cli,history,reporting}.control-and-echo.cases.toml`, each case linking its own module's requirement.
