---
change: keep-hidden-state-panels-hidden-in-the-prototype-template-whatever-display-rules-a-project-adds-58
artifact: context
---

# Context

The design stage starts from `skills/chough-workflow/templates/prototype/`. Its
`index.html` switches between the ready, loading, empty and error states by setting
`panel.hidden` on each `section[data-state-panel]` (the loading, empty and error
sections start `hidden`). Its `styles.css` has no `[hidden]` rule, so hiding relies on
the browser's own `display: none` for `[hidden]`. Any project rule that sets `display`
on those sections, for example `.surface section { display: grid }` or a token or base
stylesheet, wins over it and every state renders at once. Leif hit this in his Raven
session (2026-11-03, finding 6), filed as #58 and assigned by orc in the 2026-11-06
burn-down plan. FLOW-4 covers it: Leif approves the rendered prototype, so the
template's states must render one at a time.

`styles.css` has not changed since #2, so releases 0.1.0 to 0.4.0 and main f41e7ab4
all ship the same bytes (ce8f6374...). Changing it means that digest joins
`KNOWN_PREVIOUS` in `src/skill.rs` (REQ-cli-019), or every existing install would
read `edited` and never update.

Constraints: failing test first; the `KNOWN_PREVIOUS` row is appended at the end; no em
or en dash in the bundle; the canonical and `.agents` copies stay identical; no helper
behavior or requirement text changes.
