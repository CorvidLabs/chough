---
change: fix-349
artifact: requirements
---

# Requirements

REQ-history-006 is modified, in `deltas/history.md`: the What was verified line gains
`; limits: ` and the receipt's limitations, when it has any, before `^<evidence id>`.
Nothing else in it changes.

REQ-cli-017 is modified, in `deltas/cli.md` (part 3 of #349, Leif's bound): a person
gives an entry at most 8 limitations of at most 512 bytes each; `evidence add` and
`cases record` refuse more as `refused:limitations` with nothing written.
