---
change: show-open-questions-in-next-and-status-report-an-archived-session-as-archived-and-warn-when-an-inbox-observation-has-no
artifact: design
---

# Design

- src/main.rs: the next view adds `open_questions` as `{"total", "items"}` through the
  existing `listed` helper (at most 10 entries, each cut to 512 bytes), it joins the
  lists `fit` may shorten, and the text form prints `Open questions: none` or
  `Open questions: N open` with one `  - <question>` line each, like blockers.
  `status` adds `open_questions` (every one) to its checkpoint section.
- src/sessions.rs: `session list` reports `status: "archived"` for an archived
  session. `inbox add` warns on stderr, naming the ID, when an observation entry's ID has no
  `.chough/evidence/<id>.json` record (`evidence::is_present`), and still writes the
  entry; `inbox list` adds `recorded: true|false` to each observation entry. Fold is
  unchanged.
