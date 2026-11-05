---
change: reduce-old-observation-proof-drop-closed-cycle-checkpoint-references-at-done-trim-superseded-runs-of-cold-changes
artifact: testing
---

# Testing

tests/reduce.rs uses five archives finalized 60 to 20 days ago (two cold), a stub
`specsync`, runs of one case listed by a cold archive, and three unlisted records:
- Superseded runs are trimmed, keeping the newest run and the newest failure.
- Unlisted proof is attributed by cycle time: trimmed and ledgered with its cold owner,
  and named in its note; the hot-cycle and current records stay.
- Done drops closed-cycle checkpoint references, and the trim can then remove them.
- Native validity reads overlap (the stub counts concurrent reads).

Each fails before the change. The existing librarian, evidence and vault tests keep
passing. The trim is run and measured on this repository, and `fledge lanes run verify`
runs on Linux.
