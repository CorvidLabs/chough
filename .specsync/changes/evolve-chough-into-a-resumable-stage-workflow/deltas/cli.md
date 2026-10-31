## ADDED

### REQUIREMENT REQ-cli-007

Canonical checkpoint and explicit migration. The helper SHALL keep one canonical checkpoint at `.chough/session.json`, schema version 2, holding the objective, intent scope, modules, current stage with its status and side-stage callers, the native change identity, referenced criteria, requirements, cases, design sources and files, decisions, open questions, completed work, observation and bug references, blockers, awaiting-human gates, run consents and the next action. It SHALL migrate a schema 1 session only through an explicit command that first preserves the original bytes, and SHALL keep reading schema 1 sessions for context and status. Trace: FLOW-13, LIFE-3.

Acceptance Criteria
- A checkpoint saved by one process is read unchanged by `chough context` and `chough next` in another process; unknown fields and unsupported versions are rejected.
- `chough migrate` preserves the schema 1 bytes in `.chough/session.v1.json`, refuses to overwrite that backup, and leaves the original session unchanged when migration fails.
- Stage commands refuse a schema 1 session with a migration instruction, while context and status still work.
- No second writable checkpoint store is introduced and no adapter such as APS is required.

### REQUIREMENT REQ-cli-008

Stage routing bounded by native gates. The helper SHALL route the stages intent, design, plan, build, test, accept and done, the optional release stage, and the side stages bugs, validate and librarian. It SHALL record each advance, iterate, back, reset, pause, resume, skip and side-stage enter or return as an append-only transition with its reason. Back and reset SHALL list the later stages whose outputs need refresh and SHALL point to native SpecSync status for any reapproval or recovery. Entering build SHALL require the recorded native change to be past definition approval, and leaving accept SHALL require that change to be archived; unavailable, unknown or undecodable native state SHALL refuse those transitions. The helper SHALL NOT record or infer human approval or review. Trace: FLOW-2, FLOW-5, FLOW-10, LIFE-1.

Acceptance Criteria
- Every transition kind is exercised in an isolated fixture; history survives back and reset; skip without a reason is refused.
- With stubbed native status, advancing from plan to build is refused for draft and unavailable states and allowed for an approved state.
- Leaving accept is refused while the native change is verifying and allowed only once it is archived.
- Entering a side stage records its caller, and return restores that caller.

### REQUIREMENT REQ-cli-009

Validated test definitions. The helper SHALL validate test definitions in `.chough/cases/<module>.cases.toml` against profile `chough-testing/1`: unique stable case IDs, active HI criteria, native requirement IDs, track, method, environments, artifact types, explicit expected outcomes, setup and steps for manual or agent-assisted cases, and a binding for automated cases. Unknown fields and unknown profiles SHALL be rejected. Each case SHALL have a definition digest that is independent of its position in the file. Trace: FLOW-7, LIFE-2.

Acceptance Criteria
- Reordering cases leaves their IDs and digests unchanged; editing expected outcomes, steps or a binding changes the digest.
- Duplicate IDs, unknown criteria or requirements, unknown fields, missing expected outcomes and automated cases without a binding are refused with the offending location.

### REQUIREMENT REQ-cli-010

Focused case selection. The helper SHALL select active cases linked to the given affected criteria, requirements or paths, always including their functional cases, adding copy-localization and design-QA cases only when those tracks are requested as relevant, and SHALL report every omitted case with its reason together with any configured project-required lane. Trace: FLOW-6.

Acceptance Criteria
- An affected requirement selects its linked functional cases and omits unrelated cases with a reason.
- Design-QA and copy-localization cases are selected only when their track is requested.

### REQUIREMENT REQ-cli-011

Honest case execution. When explicitly invoked, the helper SHALL execute an automated case's binding directly without a shell, capture its output as an artifact, and record an observation linked to the case and its definition digest. The observation SHALL be passed only when the process exits zero and the binding's assertion holds: exactly one test ran for the `cargo-test` adapter, or every declared expected-output fragment appeared for the `command` and `fledge-task` adapters. A run in which no test matched SHALL be recorded as not-run. A repeated run SHALL record each attempt and a summary with attempt and failure counts and elapsed time, and SHALL stop before the next attempt when a stop request exists. Trace: FLOW-7, FLOW-9, LIFE-7.

Acceptance Criteria
- A deliberately failing assertion records failed even when another case passes in the same session.
- A cargo filter that matches no test records not-run, never passed.
- Changing a case definition reports its earlier observations as stale-case-definition.
- A stop request halts a repeated run before the next attempt, and resuming continues with the stored limits.

### REQUIREMENT REQ-cli-012

Local bug backlog. The helper SHALL persist bug records in `.chough/bugs/` with symptom, trigger, expected and actual behavior, criterion, requirement and case links, environment, revision, evidence references, impact, classification, scope and status. It SHALL list open bugs for a fresh session and SHALL search existing records by symptom, trigger and affected IDs before a new record is added. A bug SHALL remain unverified until a failed observation is linked, and a regression classification SHALL require a passed observation at an ancestor revision. Additional evidence SHALL be appended rather than overwritten. The helper SHALL render a report candidate and SHALL NOT publish it. Trace: FLOW-8, LIFE-5.

Acceptance Criteria
- A sample failure becomes a reproduced and classified bug; a second report of the same symptom is found by search and its evidence is appended.
- A regression classification without a passed observation at an ancestor revision is refused.
- An out-of-scope bug stays in the open backlog without changing the checkpoint scope.
- Rendering a report candidate prints Markdown and performs no network action.

### REQUIREMENT REQ-cli-013

Preferences and run consent. The helper SHALL keep standing project preferences in `.chough/preferences.json`, each with value, source and scope, separately from run consents stored in the checkpoint for the current objective. Repeated validation, monitoring, release routing, monitor initiation and the librarian SHALL default to ask, ask, on-request, capture-only and off respectively. A run consent SHALL never be written as a standing preference, and changing the objective SHALL clear run consents. Trace: FLOW-9, FLOW-11.

Acceptance Criteria
- Granting a run consent leaves `.chough/preferences.json` byte-identical; setting a standing preference requires a source.
- Repeated validation is refused without a consent or standing preference, and a new objective clears the consent.
- Failures found by repeated validation create local bug records that remain discoverable after restart; monitor initiation proposes a native change command only when configured and otherwise captures locally.

### REQUIREMENT REQ-cli-014

Optional release routing. The helper SHALL advance from accept to done and never to release, and SHALL allow entering release only with a release run consent or a standing release preference. Release observations SHALL use distinct kinds for publication, installation, deployment, live health and dry runs, and a dry run SHALL never satisfy a delivery kind. Trace: FLOW-11, LIFE-7.

Acceptance Criteria
- Advancing from accept never enters release, and entering release without authorization is refused.
- A dry-run observation is reported as a dry run, not as deployment or live health.

### REQUIREMENT REQ-cli-015

Optional librarian. When the librarian is enabled, the helper SHALL copy requested originals into dated snapshots with their source path, Git revision and SHA-256, SHALL regenerate a deterministic index of canonical sources, native change records, open bugs, observations, briefs and snapshots, and SHALL never move, rewrite or replace a canonical source. The librarian SHALL be usable as a side stage from any stage. Trace: FLOW-12.

Acceptance Criteria
- Snapshotting leaves the source byte-identical and records its digest; snapshotting an escaping or symlinked path fails.
- The index lists every open bug, including bugs whose reports were snapshotted.
- Checkpoints and resume work while the librarian is disabled.

### REQUIREMENT REQ-cli-016

Host skill installation. The helper SHALL install the complete skill bundle for each requested supported host directory, defaulting to `.agents/skills`, SHALL report each installed file as current, edited, known-outdated or missing, and SHALL update only missing files and files byte-identical to a known earlier bundle. Trace: FLOW-1, LIFE-4.

Acceptance Criteria
- Installing for the claude host creates `.claude/skills/chough-workflow/` with the complete bundle, and repeating the install preserves edits.
- An unmodified 0.1.0 bundle file is updated, while an edited file is reported and left unchanged.

### REQUIREMENT REQ-cli-017

Observation schema 2. The helper SHALL record schema 2 observations that may add case IDs with definition digests, method, track, executed argument vector, exit code, tests run, limitations and the not-run outcome, and SHALL continue to read schema 1 observations with unchanged freshness semantics. Trace: LIFE-2, LIFE-7, FLOW-7.

Acceptance Criteria
- Existing schema 1 records keep their freshness results, and schema 2 records add stale-case-definition.
- The kinds publication-observation, installation-observation, deployment-observation, live-health-observation and dry-run are accepted, and the existing kinds remain valid.

## MODIFIED

### SPEC SECTION Purpose

Provide the optional local helper for Chough's current-agent workflow. It installs
the workflow skill for supported hosts without replacing user files, keeps one
canonical checkpoint of the current stage, scope, decisions, preferences and human
gates, routes stage transitions within native lifecycle gates, assembles bounded
focused context, validates test definitions and runs explicitly invoked automated
cases, keeps a local bug backlog, supports an optional project-local librarian, and
records observations tied to repository content and environment. See
requirements.md and docs/FRD.md.

### SPEC SECTION Public API

The binary exposes CLI commands; it does not export a Rust library API. The
observable command contract is `chough --help`, docs/cli.md and README.md. The
command groups are init, context, status, next, migrate, checkpoint, stage, cases,
evidence, bug, pref, consent, validate, library and skill.

Internal modules under src/ make their entry points visible to the crate root only.
They are implementation details of the binary, not exports or library promises, and
command behavior is specified under Behavioral Examples and in docs/cli.md rather
than presented as Rust API symbols to the structural checker.

### SPEC SECTION Invariants

1. Initialization and skill installation never overwrite an existing user file;
   updates replace only files byte-identical to a known earlier bundle.
2. The helper never creates, records or authenticates human scope approval,
   implementation review, release or deployment.
3. HI owns criterion IDs; SpecSync owns its lifecycle, approvals, reviews and
   archives. The checkpoint references native records and never duplicates their state.
4. Observations bind revision, working content, environment and, for cases, the
   definition digest; current evidence is not equivalent to proven criterion achievement.
5. Managed and selected paths stay inside the project and symlinks are rejected.
   Context, records and captured output are bounded and deterministic for the same
   inputs and native tool responses.
6. Missing tools, missing sources, unsupported schemas, unknown native state and
   stale evidence remain visible; they never silently become successful states.
7. The helper executes a product check only when `cases run` or `validate` is
   explicitly invoked, runs the bound argument vector without a shell, and records
   passed only when the binding's assertion holds.
8. `.chough/session.json` is the single writable checkpoint. Transition history is
   append-only, and schema migration is explicit and preserves the original bytes.
9. The helper publishes nothing to an external service.

### SPEC SECTION Behavioral Examples

```gherkin
Given an edited session and installed skill
When init is repeated
Then existing files remain byte-for-byte unchanged

Given product-test observations on one revision/environment
When relevant working files change or a different environment is selected
Then status reports those observations stale or mismatched

Given a selected module and scoped HI family
When a new agent requests context
Then it receives instructions, intent, module companions, decisions and next action
without loading the whole repository or conversation history

Given a checkpoint at plan whose native change is still a draft
When the agent asks to advance to build
Then the transition is refused and native status is named as the authority

Given two cases where one bound assertion fails
When both cases are run
Then the failing case is recorded failed and the passing case does not hide it

Given a run consent for repeated validation
When the objective changes
Then the consent is cleared and project preferences are unchanged
```

### SPEC SECTION Error Cases

| Error | When | Behavior |
|-------|------|----------|
| Invalid path/schema | A record escapes the root or uses an unsupported version | Fail with an actionable error |
| Missing native tool/source | Context or lifecycle information is unavailable | Report unavailable/unknown without inventing state |
| Existing observation ID | An observation record already exists | Refuse replacement and retain the original |
| Invalid traceability | A criterion, requirement or case cannot be established | Refuse to record the observation |
| Native gate not satisfied | A stage transition needs a native state that is absent or unknown | Refuse the transition and name the native status command |
| Unauthorized optional track | Repeated validation or release lacks consent or a standing preference | Refuse and explain how a human can authorize it |
| Vacuous check | A bound command exits zero without its declared assertion | Record not-run or failed, never passed |
| Schema 1 session | A stage command meets a schema 1 checkpoint | Refuse and name the explicit migration command |

### SPEC SECTION Dependencies

Rust dependencies are declared in Cargo.toml, including a TOML parser for test
definitions. Git supplies revision and working content identity; HI supplies scoped
intent; SpecSync supplies native lifecycle output. Cargo, Fledge or another bound
command runs only when a case is explicitly executed. The helper invokes no model
provider and no hosted service.
