## MODIFIED

### REQUIREMENT REQ-cli-008

Stage routing bounded by native gates. The helper SHALL route the stages intent, design, plan, build, test, accept and done, the optional release stage, and the side stages bugs, validate and librarian. It SHALL record each advance, iterate, back, reset, pause, resume, skip and side-stage enter or return as an append-only transition with its reason. Back and reset SHALL list the later stages whose outputs need refresh and SHALL point to native SpecSync status for any reapproval or recovery. Every move SHALL meet the prerequisite of its destination, whichever command makes it: build SHALL require the recorded native change to be approved, implementing or verifying with a valid definition approval; test and accept SHALL require the build prerequisite or the done prerequisite; done SHALL require the change to be archived with authenticated terminal evidence. Unavailable, unknown or undecodable native state, an approval reported invalid, unauthenticated archive evidence and an invalid SDD policy SHALL refuse those moves; a valid disabled policy or no policy SHALL leave them ungated. The helper SHALL NOT record or infer human approval or review. Trace: FLOW-2, FLOW-5, FLOW-10, LIFE-1.

Acceptance Criteria
- Every transition kind is exercised in an isolated fixture; history survives back and reset; skip without a reason is refused.
- Entering build is refused for draft, unavailable and approval-invalid states and allowed for a valid approved state.
- Leaving accept, reset to done and release to done are refused unless the change is archived with authenticated terminal evidence.
- A malformed or ill-typed SDD policy refuses gated moves; a valid disabled policy leaves them ungated.
- Entering a side stage records its caller, and return restores that caller.
