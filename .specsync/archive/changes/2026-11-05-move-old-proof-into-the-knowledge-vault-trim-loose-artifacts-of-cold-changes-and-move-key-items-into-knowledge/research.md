---
change: move-old-proof-into-the-knowledge-vault-trim-loose-artifacts-of-cold-changes-and-move-key-items-into-knowledge
artifact: research
---

# Research

After #108's trim on Chough's own repository:
- `.chough/` is 26 MB: 501 evidence records and 1,111 artifact files.
- 360 artifact files (about 10 MB) are referenced by no observation record. Most sit in
  folders kept from earlier pull requests, such as `pr46-combined-qualification/`,
  `pr51-writer-atomicity/` and `history-reference-boundary-final/`. 7 are named by
  bugs, through their `fixture` field.
- Bugs count as closed when their status is `verified`, `duplicate` or `wont-fix`.
- 76 observation sets stay only because they are manual or agent-assisted
  (non-routine).
- Retention ledger readers need only an `id`, so loose-artifact entries with
  `artifact:` IDs never collide with observation IDs.
