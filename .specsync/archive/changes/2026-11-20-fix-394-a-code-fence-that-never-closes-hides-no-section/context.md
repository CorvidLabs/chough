---
change: fix-394-a-code-fence-that-never-closes-hides-no-section
artifact: context
---

# Context

#394 (62-F12, the 0.6.2 candidate swarm). A session body holding a three-backtick line
that never closes, as from `session new --objective '```'` or a pasted code block, makes
every save append new `## Decisions and why`, `## Open threads` and `## Timeline`
sections, since the section reader treats the rest of the body as code.

Measured on main 056cbb80 (t4b, 2026-11-20): `session new fence --objective '```'`,
then three `checkpoint --decision dN --question qN?`: 4 `## Decisions and why`, 4
`## Open threads`, 2 `## Timeline`; `chough next` says "Decisions: none" and "Open
questions: none". A `~~~` objective does not break it, since the reader knows only
backtick fences. A decision whose text is a fence opener does not either: it is written
as a list item.

The same toggle sits in the bug merge driver (`parts()` in `src/bugs.rs`). Measured:
two sides each adding a Log entry to a bug whose write-up holds an unclosed fence exit
1 with "both sides changed a paragraph differently; resolve this bug by hand", while the
same merge without the fence keeps both entries. That breaks REQ-cli-047.

REQ-records-001 already settles the rule for conflict markers: "a fence that never
closes hides nothing after it". REQ-records-004 says the sections are the only place
the session keeps decisions and open questions. The fix brings the two readers in line;
it adds no refusal and changes no approved sentence (one criterion added to each).
