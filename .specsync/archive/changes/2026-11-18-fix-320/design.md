---
change: fix-320
artifact: design
---

# Design

- `checkpoint::check_id` takes the session's ID. Its refusal adds the recovery: remove the value from the command, or, when the session already holds it, `chough session shelve <id>` and continue in `chough session new <key>`.
- `doctor`: `id_lines(text)` returns the front matter lines whose key gives an ID: `id`, `aliases`, `native_change`, `criteria`, `requirements`, `cases`, `observations`, `bugs`, `evidence`, `related`, `duplicate_of`, `case`, a flow list on the key's own line, or a block list under it.
  - A `leak` finding on such a line names the recovery as its fix and is blocked.
  - The `--fix --redact` fix redacts the body as a whole and each other front matter line, leaves every ID line byte for byte, and refuses (so the finding is `skipped`) when its own line is an ID line.
