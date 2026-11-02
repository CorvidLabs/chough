## MODIFIED

### REQUIREMENT REQ-cli-002

Focus context for resumption. The helper SHALL assemble applicable instructions, scoped HI export, selected module contracts and companions, objective, decisions, open questions and next action. The native module lesson source SHALL be `<module>.3md` where present and otherwise the Lessons section of the module `context.md`; the helper SHALL point to it and SHALL NOT keep a lesson store, index or ledger of its own. The shipped skill SHALL tell agents to record each lesson in that source with a certainty label (observation, hypothesis or verified), its source and its limits, and SHALL state that a project decision always outranks a lesson. The shipped skill SHALL ask an agent resuming work to state in its own message, before its first edit, the goal, each recorded decision that binds the next task with its reason, the last failure with its bug and status, the lessons that apply and the next task, and to say what the records do not tell it instead of guessing. Trace: LIFE-3, LIFE-6, LIFE-1, FLOW-12, FLOW-13.

Acceptance Criteria
- The same inputs yield the same ordered context; missing sources and truncation remain explicit.
- Native module .3md is the lesson source where present; otherwise the module context.md is used.
- The canonical skill states the three certainty labels, that each lesson names its source and limits, and that a project decision outranks a lesson; no Chough lesson store, index or ledger exists.
- The canonical skill's resume steps ask the agent to state the goal, binding decisions with reasons, the last failure with its bug status, the applicable lessons and the next task in its own message before its first edit, and to name what the records do not say.
