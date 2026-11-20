---
change: docs-387-no-git-while-agents-write
artifact: context
---

# Context

#387 (0.6.2 candidate swarm s3, 2026-11-18): a `git merge` started while Chough
writers ran dropped writes that had already exited 0. In 12 of 20 rounds, six
parallel `checkpoint --decision` plus `git merge side` lost one to five decisions with
nothing on standard error, even when Git refused ("would be overwritten"). A plain
flock-and-rename appender loses them too, and the guard of #330 cannot see a merge
before `MERGE_HEAD` exists. This is a Git race, not Chough's lock: the writer lock
orders Chough's own writers, and Git does not take it.

So this change is guidance only: the skill and the docs say it. Doctor noting it was
raised in the issue and is left out (it is code, and a Git command that has not
started yet leaves nothing for doctor to see).

Changing the skill's checkpoint reference needs its 0.6.1 digest in
`KNOWN_PREVIOUS` (`src/skill.rs`), so a 0.6.1 install updates; that is why the cli
module and `src/skill.rs` are declared.
