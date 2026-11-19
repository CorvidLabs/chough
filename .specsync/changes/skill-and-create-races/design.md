---
change: skill-and-create-races
artifact: design
---

# Design

- `src/records.rs` (cli): `write_new_bytes` treats a create-new that finds the file already there, between its check and its open, as "it exists" (`Ok(false)`), as the earlier check does, and names the file in every other error, relative to the project.
- `src/skill.rs` (cli): `file_state` compares a CRLF copy with the bundle once each CRLF is read as LF, and a refresh of such a copy writes CRLF line breaks; `report` keeps a host whose folder cannot be inspected as `uninspected`, with its relative directory and why, when no `--host` named it, and refuses naming it when one did; a missing file another run writes first is read again, counted as added when it holds the bundle's bytes and preserved as edited otherwise, so no `ensure!` text is ever printed.
- `src/main.rs` (cli): `init` reports a file another run wrote first as preserved, as it does a person's.
- `src/doctor.rs` (cli): the `unreadable` check also reads `git ls-files --others --exclude-standard -- .chough`, with the same readers and recovery as the committed records.
