## MODIFIED

### REQUIREMENT REQ-cli-003

The helper SHALL report actual native SpecSync status and guidance, with unavailable, failed, undecodable, oversized or timed-out responses explicitly unknown. A failed native command SHALL be shown with its result in plain words, `exit <code>` or `killed by signal <number>`, never a program's internal debug form. Native reads SHALL retain at most the parse bound of standard output and a display bound of standard error while reading, and SHALL end by a bounded deadline. Trace: LIFE-1, LIFE-6, LIFE-7.

Acceptance Criteria
- Missing SpecSync never becomes an approved or delivered state, and an agent-written session next action never grants approval or overrides a native gate.
- A native command that never exits is reported as timed-out by its deadline, and gated moves refuse it.
- Output beyond the bounds is counted and reported, not retained.
- A native command that exits 2 is shown as `exit 2`, and one ended by SIGTERM as `killed by signal 15`.
