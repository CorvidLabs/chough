## ADDED

### REQUIREMENT REQ-cli-018

Reference-aware retention. The helper SHALL preview by default which observation sets (a record and the artifacts only it references) are eligible for removal and which are protected and why, and SHALL protect sets referenced by the checkpoint, a bug, an active or the most recent archived native change, non-routine records, the newest runs and newest failure per case and environment, validation failures and boundary passes, and recent sets. It SHALL remove eligible sets only under a run consent or a standing retention preference set by a human, SHALL record each removal in an append-only ledger, and SHALL label referenced observations current, historical, pruned or unavailable, never treating pruned or unavailable ones as current or passed. It SHALL NOT change native archives, ignore rules or Git history. Trace: LIFE-2, LIFE-7, FLOW-7, FLOW-13.

Acceptance Criteria
- The preview changes no file and lists eligible and protected sets with reasons and reclaimed bytes.
- Apply without consent or the standing preference is refused.
- A pruned observation referenced anywhere is reported pruned, and a missing unlisted one unavailable; neither is current or passed.

### REQUIREMENT REQ-cli-019

Installable package. The crate package SHALL contain the helper source, the canonical skill bundle, Cargo files, README and LICENSE, and SHALL NOT contain Chough's own development records; initializing a new project SHALL install the helper's skills and an empty checkpoint only, and skill status SHALL recognize copies installed from earlier builds. Trace: LIFE-4.

Acceptance Criteria
- The package list contains no `.chough/`, `.specsync/`, examples, tests or host copies.
- A new project initialized from the installed binary has no bugs, evidence or artifacts.
