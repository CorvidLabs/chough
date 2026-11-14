---
change: fix-245-chough-library-index-links-no-source-git-ignores-so-a-fresh-clone-s-index-check-passes
artifact: context
---

# Context

corvid-agent, adopting Chough 0.6.0 in CorvidLabs/peck, found that `chough init --host
grok` writes the skill to `.grok/skills/chough-workflow/`, which that project's
`.gitignore` ignores. `knowledge/index.md` then links that file, which is never
committed, so in a fresh clone or in CI `chough library index --check` fails. That
breaks REQ-cli-044's rule that links resolve. Filed as GitHub #245. t37f reproduced it
(red at aa652cac): `src/vault.rs`'s `rule_files()` links every host's
`<dir>/chough-workflow/SKILL.md` that exists on disk, without asking Git.

On 2026-11-13T14:12Z Leif approved the scope ("Approve (Recommended)", relayed by t370):
the index stops linking skill copies Git ignores. The issue's second ask, a warning
from `chough init` when the host's skill directory is ignored, will come later as its
own change.
