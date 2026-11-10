---
change: know-29-roll-up-keeps-or-flags-tracked-session-records-when-the-librarian-vault-is-untracked-165
artifact: context
---

# Context

Issue #165 (Codex's P2 on CorvidLabs/spec-sync#941, deferred here by Leif): with the librarian on, the roll-up at done moves archived session records into the vault at `knowledge/attachments/`. Where `knowledge/` is untracked (as in spec-sync), git shows the moved records as deletions and their only copies sit untracked in one worktree; no Chough command brings a rolled session back.

Open questions for the human: keep tracked records in place when the vault is untracked, or refuse/warn and name the recovery; whether a new `session unroll`/restore command is wanted either way; and how this interacts with REQ-history-011's keep-five retention once records stay.
