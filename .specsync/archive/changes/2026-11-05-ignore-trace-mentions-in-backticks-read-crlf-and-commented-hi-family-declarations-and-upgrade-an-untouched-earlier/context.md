---
change: ignore-trace-mentions-in-backticks-read-crlf-and-commented-hi-family-declarations-and-upgrade-an-untouched-earlier
artifact: context
---

# Context

corvid-agent's review of #107 at 5326ec4 found three more gaps:
1. Reading the last `Trace:` on a line loses a list that a backticked mention follows
   (`Trace: UX-1, SEND-2. Readers grep the \`Trace:\` lines.`).
2. HI files saved with CRLF lost their family, because the front matter parser expected
   `\n` (Windows is a supported CI target).
3. A `knowledge/README.md` that an earlier Chough generated (#106, sha256 5365c2f9...)
   never gained the new front matter, because the README is create-only. This was
   raised in review 5779587370 by Leif's review agent, which asked to migrate only the
   exact old template and keep edited READMEs.

A YAML comment after the families list (`families: [SEND] # send flows`), noted as
non-blocking, is handled too.
