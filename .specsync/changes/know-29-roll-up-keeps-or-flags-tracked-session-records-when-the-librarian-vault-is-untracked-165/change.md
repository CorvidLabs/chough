---
id: know-29-roll-up-keeps-or-flags-tracked-session-records-when-the-librarian-vault-is-untracked-165
state: draft
type: feature
base_commit: 4e7bd5cbba1175f17e69752f317def5a49ac001f
---

# KNOW-29 roll-up keeps or flags tracked session records when the librarian vault is untracked (#165)

## Intent

KNOW-29 roll-up keeps or flags tracked session records when the librarian vault is untracked (#165)

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- When the librarian vault directory is untracked or ignored by git, the KNOW-29 roll-up at done keeps a tracked session record in .chough/sessions/archive/ instead of moving it out of the tree, or refuses/warns naming what would leave the tree and how to recover; when the vault is tracked, the roll-up behaves as today. Existing keep-five retention and vault summary behavior are unchanged.

## No-spec Rationale

Not applicable
