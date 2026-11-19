---
change: fix-g1-gates-and-pins-fail-closed
artifact: requirements
---

# Requirements

Five requirements gain sentences and criteria; none loses any (the delta holds the
text):

- REQ-cli-007: a blank, placeholder or doubled `no_native_change` is refused on read
  and on save, with the edit named (#401).
- REQ-cli-037: `session use` and `session new` say when a pin still names another
  session; `session shelve` resolves its key (#372).
- REQ-history-009: `inbox list` under a pin lists the named session (#393).
- REQ-cli-057: `chough change` arguments are checked before SpecSync runs (#367).
- REQ-cli-059: the history check covers the whole range and fails closed; the gate
  close and the move into done act on the resolved session (#400, #369).
