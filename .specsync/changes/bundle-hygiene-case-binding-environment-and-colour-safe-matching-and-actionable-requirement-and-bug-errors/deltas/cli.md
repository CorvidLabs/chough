## MODIFIED

### REQUIREMENT REQ-cli-005

Validate traceability. The helper SHALL validate observation criterion IDs, requirement IDs and contract paths against their native records before recording. A refused requirement link SHALL name the companion files searched and a likely cause with its fix: the module whose companion holds the ID when it belongs to another module, otherwise a typo or a requirement an approved native change adds that `specsync change check` has not yet materialized. The hint SHALL NOT read the checkpoint or native deltas and SHALL NOT accept an unresolved link. Trace: LIFE-1, LIFE-2, LIFE-5, FLOW-7.

Acceptance Criteria
- Unknown active HI IDs or absent native requirement IDs are refused; observation IDs cannot overwrite prior observations.
- Observations are unauthenticated claims, including those labelled human or delivery observations.
- A requirement of another module is refused with that module and the fix named; a typo and an unmaterialized delta ID are refused with the check-first hint.

### REQUIREMENT REQ-cli-009

Validated test definitions. The helper SHALL validate test definitions in `.chough/cases/<module>.cases.toml` against profile `chough-testing/1`: unique stable case IDs, active HI criteria, native requirement IDs, track, method, environments, artifact types, explicit expected outcomes, setup and steps for manual or agent-assisted cases, and a binding for automated cases. A binding MAY declare `env`, a bounded table of environment variable names and values; for the `cargo-test` adapter it SHALL NOT declare a variable that chooses what cargo builds or runs, and no binding SHALL declare an expected-output fragment containing the escape character. Unknown fields and unknown profiles SHALL be rejected. Each case SHALL have a definition digest that is independent of its position in the file, and an omitted or empty `env` SHALL leave that digest unchanged. Trace: FLOW-7, LIFE-2.

Acceptance Criteria
- Reordering cases leaves their IDs and digests unchanged; editing expected outcomes, steps or a binding, including its `env`, changes the digest.
- Duplicate IDs, unknown criteria or requirements, unknown fields, missing expected outcomes and automated cases without a binding are refused with the offending location.
- An invalid `env` name, a name over 128 bytes, two names equal ignoring ASCII case, a value with NUL, more than 32 entries or a value over 4 KiB, and an expected-output fragment containing ESC are refused with the offending location.
- A `cargo-test` binding that declares PATH, CARGO, CARGO_HOME, RUSTC, RUSTDOC, RUSTC_WRAPPER, RUSTC_WORKSPACE_WRAPPER, RUSTFLAGS, CARGO_ENCODED_RUSTFLAGS, a CARGO_BUILD_ name or a CARGO_TARGET_<triple>_RUNNER name, in any ASCII case, is refused with the offending location.

### REQUIREMENT REQ-cli-011

Honest case execution. When explicitly invoked, the helper SHALL execute an automated case's binding directly without a shell, with the binding's declared environment set on top of the inherited one, capture its output as an artifact, and record an observation linked to the case and its definition digest. Execution SHALL end by the binding's deadline: on the deadline, or when output stays open after the command exits, the helper SHALL stop the command and its descendants and SHALL NOT wait on output it cannot complete. Captured output SHALL be bounded per stream while reading. The artifact SHALL keep the captured bytes, while assertions SHALL be evaluated on that text with complete terminal escape sequences removed. Removal SHALL never remove a line break, and a sequence that does not complete before a line break or the end of the output SHALL be left unchanged. The observation SHALL be passed only when the process exits zero, its capture is complete and the binding's assertion holds: exactly one test ran for the `cargo-test` adapter, or every declared expected-output fragment appeared for the `command` and `fledge-task` adapters. Every observation of an executed case SHALL state as a limitation that inherited environment variables were not recorded; when the binding declares variables it SHALL also name them without their values, and when escape sequences were removed it SHALL say so. A run in which no test matched SHALL be recorded as not-run. A repeated run SHALL record each attempt and a summary with attempt and failure counts and elapsed time, and SHALL start no further attempt once a stop request or a paused checkpoint exists, including after waiting its interval. Trace: FLOW-7, FLOW-9, LIFE-7.

Acceptance Criteria
- A deliberately failing assertion records failed even when another case passes in the same session.
- A cargo filter that matches no test records not-run, never passed.
- Changing a case definition reports its earlier observations as stale-case-definition.
- A case whose descendant holds its output returns by its deadline, and its descendants are stopped.
- A stop request made during the interval halts a repeated run before the next attempt, and resuming continues with the stored limits.
- A declared variable reaches the command, and the observation names it without its value.
- An observation of a case that declares no variables still states that inherited environment variables were not recorded.
- An escape sequence interrupted by a line break or the end of the output is left unchanged, and removal never lowers the number of lines that start with `test result:`.
- Coloured output whose visible text holds a fragment passes with the escape limitation, the artifact stays byte-exact, and a fragment absent from the visible text still fails.

### REQUIREMENT REQ-cli-012

Local bug backlog. The helper SHALL persist bug records in `.chough/bugs/` with symptom, trigger, expected and actual behavior, criterion, requirement and case links, environment, revision, evidence references, impact, classification, scope and status. It SHALL list open bugs for a fresh session and SHALL search existing records by symptom, trigger and affected IDs before a new record is added. A bug SHALL remain unverified until a failed observation is linked, and a regression classification SHALL require a passed observation at an ancestor revision. Additional evidence SHALL be appended rather than overwritten. A classification or scope value given to the other field SHALL be refused with the correct flag named, and help SHALL list both value sets. The helper SHALL render a report candidate and SHALL NOT publish it. Trace: FLOW-8, LIFE-5.

Acceptance Criteria
- A sample failure becomes a reproduced and classified bug; a second report of the same symptom is found by search and its evidence is appended.
- A regression classification without a passed observation at an ancestor revision is refused.
- An out-of-scope bug stays in the open backlog without changing the checkpoint scope.
- Rendering a report candidate prints Markdown and performs no network action.
- `--classification blocking` is refused with a hint to use `--scope`, and `--scope product-defect` with a hint to use `--classification`.

### REQUIREMENT REQ-cli-019

Installable package. The crate package SHALL contain the helper source, the canonical skill bundle, Cargo files, README and LICENSE, and SHALL NOT contain Chough's own development records; initializing a new project SHALL install the helper's skills and an empty checkpoint only, and skill status SHALL recognize copies installed from earlier builds. The canonical skill bundle SHALL contain no em dash or en dash. Trace: LIFE-9, LIFE-4.

Acceptance Criteria
- The package list contains no `.chough/`, `.specsync/`, examples, tests or host copies.
- A new project initialized from the installed binary has no bugs, evidence or artifacts.
- No bundle file contains U+2013 or U+2014, and unedited copies of the earlier dashed files are reported known-outdated and updated.
