---
change: control-and-echo
artifact: design
---

# Design

- `src/leaks.rs` (records): the guard every written value passes refuses a control character as `refused:control-character`, naming the field and the code point, before the leak match and under `--redact` too, in a value the stored record does not already hold, so a value written by hand or by an earlier release never blocks a save and is escaped on print; it stores a CRLF as LF; the one-line checks of `--title` and `--pr` stay where they are. A `printable(text, one_line)` helper escapes control characters as `\u` and four hex digits, and a line feed as `\n` on one line. `in_absolute_path` reads a `file:` URL's path. The `USERNAME` variable is already read; the sentence now says so.
- `src/markdown.rs` (records): the scalar writer double-quotes `null`, `~`, `true` and `false` in any letter case.
- `src/journal.rs` (records): the post links the PR only when it is a number or an `http(s)` URL.
- `src/identity.rs` (records): the explicit evidence ID error uses `shown`.
- `src/records.rs` (cli): `print_json` also writes DEL, C1 controls and the bidi characters as `\u` escapes, which serde_json leaves raw, so every JSON view prints none raw and parses back the same.
- `src/main.rs`, `src/context.rs`, `src/checkpoint.rs`, `src/bugs.rs` (cli), `src/sessions.rs`, `src/history.rs` (history), `src/timing.rs` (reporting): each text view that prints stored text, `next`, `status`, `context`, the checkpoint view, `session list`, `inbox list`, `history show`, `bug list`, `bug show` and `timing`, goes through `printable`; `next` gains a `PR:` line, and `status`, `context` and the checkpoint view a `pr` field.
- `src/checkpoint.rs`: `--pr` accepts digits with no sign, `#` or leading zero, or an `http://` or `https://` URL with a host and no whitespace; `--pr ""` removes the key.
- `src/prefs.rs`, `src/skill.rs`, `src/stage.rs`, `src/validate.rs`, `src/library.rs`, `src/cases.rs` (cli), `src/history.rs`: the remaining echo sites, and the `--environment` and `cases select` output, use `shown`.
