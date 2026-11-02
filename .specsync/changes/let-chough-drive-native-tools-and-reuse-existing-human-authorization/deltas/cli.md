## ADDED

### REQUIREMENT REQ-cli-029

Agent-operated native workflow. Chough's current agent SHALL execute HI, SpecSync and Fledge operations internally for authorized work, keeping each tool as the owner of its records. It SHALL use an applicable explicit prior or standing human authorization for a concrete scope instead of asking the human to repeat permission merely to run or record an internal tool operation. Before recording native scope approval it SHALL identify the actual human instruction, its source, covered scope and limits in the checkpoint and native change context, and use the human actor. A general continue instruction without applicable authority, an agent label, a bot review or a passing check SHALL NOT manufacture human approval. Fresh native checks, scope integrity and separate human implementation acceptance and delivery authority SHALL remain required. The user-facing guidance SHALL describe giving Chough the goal and authorization and reviewing the resulting PR, with native commands assigned to the agent. Unedited known predecessor skill copies SHALL update safely, and edited copies SHALL remain preserved. Trace: FLOW-1, FLOW-2, FLOW-5, FLOW-13, LIFE-1, LIFE-6, LIFE-7.

Acceptance Criteria
- The main skill, lifecycle reference and README consistently assign internal native command execution to the current agent and do not require the operator to run tool-specific approval commands for already authorized work.
- Explicit applicable prior or standing human authority is recorded with source, scope and limits before a concrete native scope approval; absent or exceeded authority asks for the actual missing decision. Agent labels and bot approvals do not become human decisions.
- Native status/check, frozen scope, evidence freshness, human implementation acceptance and separate merge/release/deployment boundaries are preserved; no additional lifecycle ledger or runtime is introduced.
- Independently hashed exact predecessor fixtures exercise real install/status/update, replacing only known unchanged copies and preserving edited bytes. Package hygiene remains enforced.
- Qualification distinguishes source/package verification from a fresh-agent cold-start run, and no skipped cold-start proof is promoted to a pass.
