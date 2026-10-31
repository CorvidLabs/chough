---
change: parse-native-status-beyond-the-display-bound-and-apply-checkpoint-clears-first
artifact: context
---

# Context

Two Chough defects surfaced while closing the archived change
`evolve-chough-into-a-resumable-stage-workflow` on PR #2. Both are reproduced with
failed observations and recorded in the local backlog.

- BUG-0004 (`bug-0004-repro-1`): once archived, `specsync change status <id>
  --json` returned 100,133 bytes (262 acceptance entries). `records::native` keeps
  only 16,384 bytes of stdout for display, and the gate parsed that truncated text,
  so `chough stage advance` refused to leave accept as `native-unknown` although
  `summary.state` was `archived`. `chough status` showed the same response as
  `invalid-output`. The failure was safe (a refusal, not a pass). Unit tests stubbed
  small status JSON and the exercise stops at accept, so nothing reached a real
  archive before this.
- BUG-0005 (`bug-0005-repro-1`): `chough checkpoint --clear-awaiting-human
  --awaiting-human X` returned an empty list, because clears run after additions.
  The same ordering applies to `--clear-blockers`.

The human asked for this fix after accepting PR #2. PR #2 is not merged yet, so
this branch starts at its archive tip `1a108d9` and its pull request is stacked on
`feat/resumable-stage-workflow`.

## Interview note

`public_contract` is answered `no`: the canonical contract and requirements do not
change, and SpecSync refuses `no_spec_change` with a public-contract change. The
observable CLI behavior does change, from a defect back to what REQ-cli-007 and
REQ-cli-008 already specify.
