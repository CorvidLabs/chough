---
id: closed-bugs-release-proof-key-items-move-into-the-vault-under-the-bug-and-the-rest-is-trimmed-when-cold-know-22
state: draft
type: feature
base_commit: 9991d09cb153ac409a43c5071ad5371ddd53fe49
---

# Closed bugs release proof: key items move into the vault under the bug and the rest is trimmed when cold (KNOW-22)

## Intent

Closed bugs release proof: key items move into the vault under the bug and the rest is trimmed when cold (KNOW-22)

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- In the librarian trim a verified, duplicate or wont-fix bug no longer protects its linked observations; its earliest linked failure with artifacts, for a verified bug its first linked pass after it, and its fixture under .chough move into knowledge/attachments/<bug folder>/ when they would otherwise be trimmed; other linked observations are trimmed and ledgered when cold; library index writes bugs/<id>.md linking what moved; open bugs and evidence prune keep every link. The new tests fail first and fledge lanes run verify passes on Linux.

## No-spec Rationale

KNOW-22 (Leif chose Approve as proposed, 2026-11-05 16:34 MST): a new requirement for how the librarian trim treats observations and fixtures of verified, duplicate and wont-fix bugs.
