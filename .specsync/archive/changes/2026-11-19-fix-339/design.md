---
change: fix-339
artifact: design
---

# Design

- `records::markers_in(text, markdown)`, behind `conflict_markers` (records) and `conflict_markers_in_toml` (case definitions):
  - Skips the front matter, and in Markdown, every line inside a fenced code block: a backtick or tilde fence of three or more, indented at most three spaces, closed by the same mark at least as long.
  - Counts a `=======` as a marker only inside a conflict. A lone one outside a conflict and a fence is returned separately, by `lone_separators`.
- `cases::load` checks a definition file for markers before parsing it and refuses, naming the file, the line and the marker.
- `doctor::conflict_marker_findings` also reads committed `.chough/cases/*.cases.toml`, and reports each lone separator as an advisory `possible_conflict_separator` finding. `Finding::advisory` adds `"advisory": true` to its JSON and keeps it out of the exit status.
- `migrate`'s `planned_conflicts` and `conflict_refusal` find each planned record that would hold a marker, and its 0.5.x source through the plan's `becomes`.
  - The preview sets `apply_refused` to `refused:conflict-markers`, naming the source, when no open change refuses first.
  - `--apply` gives the same refusal before writing anything.
