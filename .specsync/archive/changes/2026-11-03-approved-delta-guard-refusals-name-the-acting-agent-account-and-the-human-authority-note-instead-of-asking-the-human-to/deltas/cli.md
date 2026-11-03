## MODIFIED

### REQUIREMENT REQ-cli-022

Approved-delta guard. Before a move to build, test or accept on a native change that is approved, implementing or verifying with a valid definition approval, the helper SHALL also confirm that every affected semantic delta matches the digest recorded by the latest native definition approval, computed as SpecSync defines it, reading the native approval ledger and the deltas read-only within a bound. A changed, missing or unapproved delta SHALL refuse the move as delta-changed, and a comparison that cannot be made SHALL refuse it as delta-unverifiable; each refusal SHALL name the next native action, and a remedy that names a native approval SHALL name the acting agent account as the actor with the human authority source, covered scope and limits in the note, never the human's name or a request that the human run it. The guard SHALL start no process and SHALL NOT run native verification. Trace: FLOW-5, LIFE-7, LIFE-1.

Acceptance Criteria
- Digests computed for archived changes match the digests their approvals recorded.
- An edited delta, a module added after approval and a deleted delta refuse the move with `refused:native-delta-changed`; a missing, oversized or malformed ledger and an approval without delta digests refuse it with `refused:native-delta-unverifiable`; a refused move leaves the session and transitions byte-identical.
- A change whose deltas match, including after a line-ending-only edit, passes, and a no-spec-change approval with no deltas passes.
- A refusal whose remedy is a new definition approval names `specsync change approve <id> --actor "<agent-account>" --note "Human authority: <source>; covered scope: <scope>; limits: <limits>"` and does not ask the human to approve or name them as the actor.
