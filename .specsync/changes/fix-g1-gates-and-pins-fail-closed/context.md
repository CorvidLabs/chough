---
change: fix-g1-gates-and-pins-fail-closed
artifact: context
---

# Context

Five reports from the 0.6.2 candidate swarm and the reviews of merged PRs, all in the
code that decides which session a command acts on and whether a gate holds
(#401, #400, #369, #393, #372), with one item added by the lead from the leaks sweep
(#367): arguments of `chough change` reach SpecSync's error text unchecked.

Each was reproduced on main (9d260310) with the built binary before this was written:

- #401: a session file edited to hold `no_native_change: " "` reads as "none needed"
  with an empty reason, so plan to build moves ungated. The blank and both-fields
  checks exist only in `chough checkpoint --no-native-change`.
- #393: with the pointer on beta, `chough inbox list --session alpha` lists beta.
  `InboxCommand::List` calls `active(root)`; `Fold` resolves the pinned path.
- #372: `CHOUGH_SESSION=beta chough session use alpha` prints `active: alpha` and
  says nothing about the variable; `chough session shelve alpha-work` says no such
  folder though `session use alpha-work` resolves it.
- #369: `change ship` runs its gate close and the move into done as child `chough`
  commands with no `--session`, so a `--session` pin is lost for those two steps.
- #400: `refuse_corrupt_history` stops after 1,000 commits and then passes.

Already true and kept: `session new` and `session use` ignore the pin (REQ-cli-037);
`session list` reports a session that cannot be read with its reason and
`chough session shelve <id>`. `chough doctor` does not name a session it cannot read;
that is doctor's code (the fixer's batch) and is not changed here.

The REQ-cli-057 part needs `leaks::unshowable` from the leaks PR (#407) and is built
after that merges.
