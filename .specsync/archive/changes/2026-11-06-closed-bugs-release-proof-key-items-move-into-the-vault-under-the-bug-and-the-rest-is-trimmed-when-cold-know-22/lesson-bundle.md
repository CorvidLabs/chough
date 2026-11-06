# Lesson bundle: closed-bugs-release-proof-key-items-move-into-the-vault-under-the-bug-and-the-rest-is-trimmed-when-cold-know-22

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Closed bugs release proof: key items move into the vault under the bug and the rest is trimmed when cold (KNOW-22)
- **Kind**: Feature
- **Specs**: cli
- **Paths**: src/evidence.rs, src/bugs.rs, src/vault.rs, src/library.rs, tests/reduce.rs, tests/evidence.rs, docs/cli.md, CHANGELOG.md
- **Acceptance**: In the librarian trim a verified, duplicate or wont-fix bug no longer protects its linked observations; its earliest linked failure with artifacts, for a verified bug its first linked pass after it, and its fixture under .chough move into knowledge/attachments/<bug folder>/ when they would otherwise be trimmed; other linked observations are trimmed and ledgered when cold; library index writes bugs/<id>.md linking what moved; open bugs and evidence prune keep every link. The new tests fail first and fledge lanes run verify passes on Linux.

## Evidence

- Verification commit: `3d2709369d066318aa8570031ad66ea0d33c0361`
- Base commit: `9991d09cb153ac409a43c5071ad5371ddd53fe49`
- Verified by: `specsync check --spec cli --spec history`

## From the change's context.md

# Context

## KNOW-22

"A verified, duplicate or won't-fix bug stops holding its observations: its key items (reproduction, failure-first log, the fixing run) move into the vault under the bug's note, and the rest is summarized and trimmed." Leif captured it as written on 2026-11-05.

## Leif's choice: "Approve as proposed"

2026-11-05, 16:34 MST, via the orc. For verified, duplicate and wont-fix bugs, the key items are:
- (a) the fixture under `.chough/`;
- (b) the earliest linked failed observation, with its artifacts;
- (c) for verified bugs, the first linked passed observation after it.

They move to `knowledge/attachments/<bug folder>/`, linked from a minimal bug note. Other linked observations are summarized and trimmed by the normal cold rule. Records keep their status and IDs, and open bugs keep everything.

## On this repository

There are 52 bugs: 45 verified and 7 open. They make 167 observation links, and closed bugs make about 150 of the librarian's protections.

## Found while building

`references()` also serves `chough evidence prune`, which deletes outright and moves nothing into the vault. The release therefore applies only inside the trim's protection rules. `evidence prune` still keeps every bug's links, so a key item is never deleted without being moved.

## Out of scope

- The full bug note (KNOW-27).
- The vault's folders and index (KNOW-28).
- Ledger rotation (KNOW-23).

## From the change's design.md

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

## From the change's testing.md

# Testing

## tests/reduce.rs

- `a_closed_bug_moves_its_key_items_into_the_vault_and_releases_the_rest`, for verified, duplicate and wont-fix:
  - f1 (the earliest failure, with its log) moves to `knowledge/attachments/BUG-0002/`;
  - p1 (the first pass after it) moves there only for the verified bug;
  - x1 is trimmed and ledgered;
  - f2 and n1 keep their KNOW-17 protections;
  - the bug record is unchanged;
  - `knowledge/bugs/BUG-0002.md` links what moved.

  It fails first: all three runs are protected by `BUG-0002 <status>`.
- `an_open_bug_keeps_every_linked_observation`: a guard; it passes before and after.
- `a_closed_bug_releases_its_loose_fixture`: now also asserts the fixture's bytes at `knowledge/attachments/BUG-0001/artifacts/bug-repro.log`. It fails first: the fixture is deleted, not moved.

## tests/evidence.rs

- The `evidence prune` test still asserts `BUG-0001 verified` as a protection.
- The shared `knowledge_fixture` uses an open bug.

## Lanes

`fledge lanes run verify` passes on Linux.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-cli-048 | tests/reduce.rs `a_closed_bug_moves_its_key_items_into_the_vault_and_releases_the_rest`, `a_closed_bug_releases_its_loose_fixture`, `an_open_bug_keeps_every_linked_observation`; tests/evidence.rs `every_protection_reason_is_named` (evidence prune keeps a verified bug's link) |

## Where these lessons go

- `specs/cli/context.md`
