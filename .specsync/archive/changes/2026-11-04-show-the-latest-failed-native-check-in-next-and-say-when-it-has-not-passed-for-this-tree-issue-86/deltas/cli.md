## ADDED

### REQUIREMENT REQ-cli-039

Native check result in next. When the recorded native change has a workspace under `.specsync/changes/<id>/`, `chough next` (text and JSON) SHALL report the native check: the latest attempt SpecSync recorded in that workspace's `verification-attempts.json` by timestamp, with its time and commit, and when it failed, the failing commands with their exit codes, at most 3 with their total. When the native state `next` read is `approved` or `implementing`, `next` SHALL also say that the native check has not passed for the current tree, that SpecSync keeps no reason when it refuses before verifying, and name `specsync change check <id>`. When that state is `verifying`, it SHALL use the same read's `handoff.readiness`: readiness `safe` means the last check matches the tree; any other readiness SHALL be reported as not passed for this tree with SpecSync's reason; and no readiness SHALL be reported as unknown, so a bare `verifying` state never clears the statement. A failed attempt that names no failing command SHALL say so. It SHALL read only that local file and the native state it already read, with no additional native read, and SHALL report an unreadable, oversized or malformed attempts record as unknown, never as no failure. Trace: LIFE-7, LIFE-3.

Acceptance Criteria
- After a failed attempt, `next` text and JSON show its time, commit, failing commands and exit codes; a later passing attempt replaces it.
- With the change approved or implementing, `next` says the native check has not passed for this tree, that SpecSync keeps no reason for refusals, and names `specsync change check <id>`; when verifying, it does not.
- A malformed attempts file is reported as unknown, and `next` still makes one native status read.
- A verifying change whose handoff readiness is not `safe` (the tree moved after the last check) is reported as not passed for this tree with SpecSync's reason; with readiness `safe` there is no note; with no readiness, freshness is reported as unknown.
