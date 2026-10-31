## MODIFIED

### REQUIREMENT REQ-cli-003

The helper SHALL report actual native SpecSync status and guidance, with unavailable, failed, undecodable, oversized or timed-out responses explicitly unknown. Native reads SHALL retain at most the parse bound of standard output and a display bound of standard error while reading, and SHALL end by a bounded deadline. Trace: LIFE-1, LIFE-6, LIFE-7.

Acceptance Criteria
- Missing SpecSync never becomes an approved or delivered state, and an agent-written session next action never grants approval or overrides a native gate.
- A native command that never exits is reported as timed-out by its deadline, and gated moves refuse it.
- Output beyond the bounds is counted and reported, not retained.

### REQUIREMENT REQ-cli-011

Honest case execution. When explicitly invoked, the helper SHALL execute an automated case's binding directly without a shell, capture its output as an artifact, and record an observation linked to the case and its definition digest. Execution SHALL end by the binding's deadline: on the deadline, or when output stays open after the command exits, the helper SHALL stop the command and its descendants and SHALL NOT wait on output it cannot complete. Captured output SHALL be bounded per stream while reading. The observation SHALL be passed only when the process exits zero, its capture is complete and the binding's assertion holds: exactly one test ran for the `cargo-test` adapter, or every declared expected-output fragment appeared for the `command` and `fledge-task` adapters. A run in which no test matched SHALL be recorded as not-run. A repeated run SHALL record each attempt and a summary with attempt and failure counts and elapsed time, and SHALL start no further attempt once a stop request or a paused checkpoint exists, including after waiting its interval. Trace: FLOW-7, FLOW-9, LIFE-7.

Acceptance Criteria
- A deliberately failing assertion records failed even when another case passes in the same session.
- A cargo filter that matches no test records not-run, never passed.
- Changing a case definition reports its earlier observations as stale-case-definition.
- A case whose descendant holds its output returns by its deadline, and its descendants are stopped.
- A stop request made during the interval halts a repeated run before the next attempt, and resuming continues with the stored limits.
