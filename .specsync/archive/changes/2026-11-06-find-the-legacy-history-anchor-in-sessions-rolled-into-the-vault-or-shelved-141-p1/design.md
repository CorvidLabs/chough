---
change: find-the-legacy-history-anchor-in-sessions-rolled-into-the-vault-or-shelved-141-p1
artifact: design
---

# Design

## Approach

`adopted_anchor` scans, in order:
1. `.chough/sessions/`;
2. `archive/`;
3. `shelved/`;
4. each `<library-dir>/attachments/*/sessions/`.

Each file is read within the checkpoint cap. The match rule stays exact: the recorded anchor must equal the current one. The library directory comes from the `library-dir` preference; if that can't be read, the vault scan is skipped.

## Risks

- The anchor now also depends on the knowledge directory. A follow-up could keep the anchor holder out of the roll-up, which would also need the KNOW-29 fixture to model a single anchor holder. It is offered to Leif, not done here.

## Constraints consulted

- REQ-history-002: unexpected anchor changes still refuse.
- KNOW-29.
- REQ-history-006: shelve.
