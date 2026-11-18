---
change: fix-266
artifact: context
---

# Context

GitHub #266, from the 0.6.0 swarm report (#273, s4). During a `git merge` that conflicts on `.chough/` records, `chough next` and `chough status` exit 0 with normal guidance and no warning, and `chough doctor` reports nothing about the merge or the conflict markers. With a session-archive-versus-session-edit conflict, `next` even fails with a misleading "no open session". Nothing tells the user their records are mid-conflict, and Chough writes into the conflicted tree.

Leif's decision (2026-11-18, planning interview, recorded on #266 and relayed by hc t4c as the scope approval): warn, and refuse `.chough` writes. `next`, `status` and `doctor` report an in-progress git merge or rebase (`MERGE_HEAD`, `REBASE_HEAD`, unmerged paths) with "finish or abort it first". Write commands refuse only while paths under `.chough/` are unmerged.

The rule is a new requirement, REQ-cli-069. The `doctor` finding is defined there, beside the findings of REQ-history-012, rather than by editing that requirement, which #250 and #320 also change.

Sent back once, 2026-11-18, on Leif's word "Let the fixer send it back once" (relayed by hc t4c): the #330 review found that a finished rebase can leave `REBASE_HEAD` behind, so the code reads a rebase only from its `rebase-merge` or `rebase-apply` folder, and REQ-cli-069 and the acceptance criteria drop `REBASE_HEAD` to match. Nothing else in the scope changed.
