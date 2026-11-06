---
change: closed-bugs-release-proof-key-items-move-into-the-vault-under-the-bug-and-the-rest-is-trimmed-when-cold-know-22
artifact: design
---

# Design

## Approach

- `bugs::closed_links` returns each closed bug with its title, status, links in order and fixture.
- `evidence::closed_bug_key_items` derives the key items:
  - by observation ID, the earliest failure by time and then ID, and for verified bugs the first pass after it;
  - by file key, a fixture under `.chough/`.
- In `summary_protections`, a closed bug's link is no longer a reason. A key-item set, or a loose-group file under a closed fixture, carries `bug_files`. `candidate_moves` sends those files to the bug's attachments folder. Everything else (occupied and too-long checks, ledger, restore) works unchanged through `candidate_moves`.
- `vault::bug_notes` writes `bugs/<id>.md` for each closed bug whose folder has files. `library index` includes these notes, and its `--check` covers them.

## Risks

- **`evidence prune`:** it keeps its existing bug protection, so it never deletes a key item.
- **Tests:** the shared `knowledge_fixture` in tests/evidence.rs used a verified bug to show that a bug link protects in the summary. It now uses an open bug, which keeps that assertion true under the new rule.

## Constraints consulted

- REQ-cli-033, REQ-cli-041, REQ-cli-046.
- KNOW-13, KNOW-17, KNOW-18, KNOW-19.
- The #109 and #110 lessons: never overwrite, and keep another writer's bytes.
