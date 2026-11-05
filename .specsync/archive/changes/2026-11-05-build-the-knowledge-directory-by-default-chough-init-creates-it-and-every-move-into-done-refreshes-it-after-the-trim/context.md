---
change: build-the-knowledge-directory-by-default-chough-init-creates-it-and-every-move-into-done-refreshes-it-after-the-trim
artifact: context
---

# Context

On 2026-11-04 Chough's own `chough next` said "Knowledge index: not built; run `chough
library init` to create knowledge/". The librarian is on by default (KNOW-5), but no
command builds `knowledge/` except `library init` and `library index`. The trim at done
(REQ-cli-041) regenerates entries only when it removes something, and does so before it
removes and ledgers the files. The trim ran on this repository five times that day and
left 85 ledger entries in `.chough/pruned.jsonl` pointing to
`knowledge/archives/<change>.md` files that were never committed. The one folder it did
write, on #97's branch, was deleted as a stray by the agent. Nothing is lost:
`chough library index` regenerates all 79 entries in about 2 seconds, with the trimmed
observations shown as "not-run (pruned)" (KNOW-4). But a fresh agent is told there is no
index, against KNOW-1.

The module lesson sources hold 5 lessons, all in cli and all from 2026-10-31. The
history and reporting sections say none are recorded yet. No change on 2026-11-04 ran
the closeout lesson step from the librarian reference, including the agent's own
closeouts.

Leif (0xLeif) answered "yes" on 2026-11-04 to building and committing `knowledge/`
here, making the librarian default create it at init and refresh it when the trim runs
at done, and folding that day's lessons into the cli and history lesson sources.
