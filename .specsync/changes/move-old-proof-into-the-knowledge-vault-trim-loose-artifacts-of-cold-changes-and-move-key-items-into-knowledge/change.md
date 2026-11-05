---
id: move-old-proof-into-the-knowledge-vault-trim-loose-artifacts-of-cold-changes-and-move-key-items-into-knowledge
state: approved
type: feature
base_commit: 784b1406d9e5b4b3a53176c3772aaeb003483412
---

# Move old proof into the knowledge vault: trim loose artifacts of cold changes and move key items into knowledge attachments (KNOW-18, KNOW-19)

## Intent

Move old proof into the knowledge vault: trim loose artifacts of cold changes and move key items into knowledge attachments (KNOW-18, KNOW-19)

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- Loose artifact entries no record references are trimmed and ledgered as loose artifacts once the change that listed or made them is cold, and listed in its knowledge note; entries of current work, named by the checkpoint or an open bug, or listed by a hot archive stay; a closed bug releases its fixture. Key items of a cold change (manual or agent-assisted observations whose only protection is being non-routine, failed-run logs, screenshots and failure-first logs) move to knowledge/attachments/<archive>/, mapped in the ledger and linked from the note. New tests fail first, the trim on this repository is measured, and fledge lanes run verify passes on Linux.

## No-spec Rationale

Not applicable
