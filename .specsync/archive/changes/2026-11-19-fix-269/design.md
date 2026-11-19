---
change: fix-269
artifact: design
---

# Design

- `Record::fill_placeholder(heading, untouched, with)` in `src/markdown.rs` makes two replacements, each only when the text is untouched:
  - the `## <heading>` section's text becomes `with` when, trimmed, it is exactly `untouched`, with the blank lines around it kept;
  - the body's first line becomes `# <with>` when it is exactly `# <untouched>`.
- `session_md::render_with`, through which every save of a session passes, calls it with the Goal, the placeholder and `one_line(objective)` whenever the objective is not the placeholder.
  - The first save after `checkpoint --objective` fills both.
  - From then on neither matches, so nothing is rewritten again, and a person's text never matches.
- The journal post takes its Goal paragraph from the record at the move into done, so it carries the filled objective.
- The placeholder is named once in `src/session_md.rs`, matching `sessions::DEFAULT_OBJECTIVE`. The tests use the literal, so a change to the default fails them.
