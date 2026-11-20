---
change: doctor-fix-safety
artifact: design
---

# Design

- `src/sessions.rs`: `remove_archived_folder` reads the story's receipts and keeps an evidence entry, with its log and screenshot, unless a receipt has its ID and equals the receipt the entry gives now (`evidence::receipts_of` for that entry), and keeps every entry file under `agent/inbox/`; it removes macOS metadata in the folder whether committed or not. It returns what it kept and why.
- `src/doctor.rs`: the `unfinished_archive` finding names each kept entry and the way out by hand; `timeline_rewritten` skips an entry equal to HEAD's entry redacted again; the conflict scan covers the front matter, and `unreadable` names resolving a front-matter conflict instead of shelving; `root_code` walks with `symlink_metadata`, follows no link, and stops at 4,096 entries or 32 folders deep, saying so in the advice.
- `src/main.rs`: `next`'s no-open-session view names committing the session folder, story and journal post, then `chough doctor --fix`, while an archive is unfinished.
