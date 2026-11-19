---
change: leaks-and-output
artifact: context
---

# Context

The first batch of the 0.6.2 bug round, leaks and output, from the 0.6.2 candidate swarm's s6 run (leak guard red team and hostile input). The run's notes are on Leif's Mac.

- #364 (HIGH, s6-F1): `checkpoint --resolve-question … --answer` stores the answer in the session's answers map, which the save's leak guard never visits, so every leak kind is committed, `--redact` changes nothing, and there is no 4,096-byte bound.
- #365 (s6-F3): doctor's `conflict_markers` finding, and every other marker refusal, prints the whole marker line, which for a merge, rebase or cherry-pick carries a branch name or a commit subject.
- #367 (s6-F4): errors echo the offending value: the session pin (`CHOUGH_SESSION`, `--session`, `session use`), `bug update --related`/`--duplicate-of`/`--observation`, `bug add --observation` and `gate record --decided-by`, for all 25 leak kinds, and with control characters raw.
- #368 (s6-F5): leak forms still missing since the 0.6.1 swarm.
- #379 (s6-F8): the user name comes only from `USER`/`LOGNAME`, so it is unchecked when they are unset, and `USER=dev` makes every `/dev/` path a leak.
- #380 (s6-F9): `gate record`'s refusal says `Error:` twice.

#366, escaping control characters on print, is a later batch, since it touches every printer.
