---
change: fix-394-a-code-fence-that-never-closes-hides-no-section
artifact: design
---

# Design

Nothing visual changes. One fence rule in `src/markdown.rs`, shared by the section
reader (`section_span`) and the bug merge driver's `parts()`: a line opens a fence as
CommonMark reads one (three or more backticks or tildes, at most three spaces of
indent), and the fence hides the lines up to its closing line; a fence with no closing
line hides nothing after it. A closed fence still hides a heading it quotes, as the
existing `## Log` test pins.

Options considered:
- A (recommended): the rule above, as REQ-records-001 already says for markers.
- B: a fence also ends at the next heading of a section Chough owns. It would also stop
  a later closing line in a person's own section from re-hiding Chough's headings, but
  it breaks quoting a Chough heading inside a closed fence, which a test pins today.
- C: fold the objective when it is a fence line, at write. It covers only the objective
  and changes the text a person gave; A covers every source. Not taken.
