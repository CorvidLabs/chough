# CLI requirements

These IDs describe durable behavior. Passing observations live separately in
`.chough/evidence/`, tied to repository content and environment. FRD provenance
and scope live in [docs/FRD.md](../../docs/FRD.md).

### REQ-cli-001 — Preserve user files

The helper SHALL initialize a resumable session and discoverable workflow skill
without replacing existing files. Trace: LIFE-4, LIFE-6.

Acceptance: repeated init preserves edited session and skill bytes; missing bundle
files can be added without overwriting neighbors.

### REQ-cli-002 — Focus context for resumption

The helper SHALL assemble applicable instructions, scoped HI export, selected
module contracts/companions, objective, decisions, open questions and next action.
Trace: LIFE-3, LIFE-6.

Acceptance: the same inputs yield the same ordered context; missing sources and
truncation remain explicit. Native module .3md is the lesson source where present.

### REQ-cli-003

The helper SHALL report actual native SpecSync status and guidance, with unavailable, failed, undecodable, oversized or timed-out responses explicitly unknown. Native reads SHALL retain at most the parse bound of standard output and a display bound of standard error while reading, and SHALL end by a bounded deadline. Trace: LIFE-1, LIFE-6, LIFE-7.

Acceptance Criteria
- Missing SpecSync never becomes an approved or delivered state, and an agent-written session next action never grants approval or overrides a native gate.
- A native command that never exits is reported as timed-out by its deadline, and gated moves refuse it.
- Output beyond the bounds is counted and reported, not retained.

### REQ-cli-004

Bind observations to content and environment. The helper SHALL record with each observation the Git revision, a working content identity and an explicit environment, and SHALL report freshness limitations. Freshness SHALL compare the working content identity, which covers every tracked or untracked non-ignored file except Chough's own records, build output and SpecSync lifecycle bookkeeping (the state, approval, verification, review, finalization, accepted-state and generated lesson-bundle records of a change and the lifecycle state field of its front matter); a change's deltas and other artifacts remain covered wherever the change directory is. The recorded revision SHALL be kept for display and regression ancestry and SHALL NOT by itself make an observation stale. An observation recorded without a content identity SHALL keep the revision-bound comparison. Trace: LIFE-2, LIFE-6, LIFE-7.

Acceptance Criteria
- Changing observed content, including a native change's delta or artifacts before materialization, makes prior observations stale; another environment is mismatched; no chosen environment is unchecked.
- Recording another observation, editing the session, committing only Chough records, an empty commit, SpecSync lifecycle bookkeeping writes and archiving a change do not self-invalidate the content identity.
- An observation without a content identity remains stale after any new commit.

### REQ-cli-005

Validate traceability. The helper SHALL validate observation criterion IDs, requirement IDs and contract paths against their native records before recording. A refused requirement link SHALL name the companion files searched and a likely cause with its fix: the module whose companion holds the ID when it belongs to another module, otherwise a typo or a requirement an approved native change adds that `specsync change check` has not yet materialized. The hint SHALL NOT read the checkpoint or native deltas and SHALL NOT accept an unresolved link. Trace: LIFE-1, LIFE-2, LIFE-5, FLOW-7.

Acceptance Criteria
- Unknown active HI IDs or absent native requirement IDs are refused; observation IDs cannot overwrite prior observations.
- Observations are unauthenticated claims, including those labelled human or delivery observations.
- A requirement of another module is refused with that module and the fix named; a typo and an unmaterialized delta ID are refused with the check-first hint.

### REQ-cli-006 — Bound local input

The helper SHALL bound assembled context and reject unsupported records or paths
escaping the project. Trace: LIFE-3, LIFE-4.

Acceptance: traversal, invalid module/record IDs and symlink escape attempts fail;
large context is visibly bounded. Local artifacts remain reviewable plain files.

### REQ-cli-007

Canonical checkpoint and explicit migration. The helper SHALL keep one canonical checkpoint at `.chough/session.json`, schema version 2, holding the objective, intent scope, modules, current stage with its status and side-stage callers, the native change identity, referenced criteria, requirements, cases, design sources and files, decisions, open questions, completed work, observation and bug references, blockers, awaiting-human gates, run consents and the next action. It SHALL migrate a schema 1 session only through an explicit command that first preserves the original bytes, and SHALL keep reading schema 1 sessions for context and status. Trace: FLOW-13, LIFE-3.

Acceptance Criteria
- A checkpoint saved by one process is read unchanged by `chough context` and `chough next` in another process; unknown fields and unsupported versions are rejected.
- `chough migrate` preserves the schema 1 bytes in `.chough/session.v1.json`, refuses to overwrite that backup, and leaves the original session unchanged when migration fails.
- Stage commands refuse a schema 1 session with a migration instruction, while context and status still work.
- No second writable checkpoint store is introduced and no adapter such as APS is required.

### REQ-cli-008

Stage routing bounded by native gates. The helper SHALL route the stages intent, design, plan, build, test, accept and done, the optional release stage, and the side stages bugs, validate and librarian. It SHALL record each advance, iterate, back, reset, pause, resume, skip and side-stage enter or return as an append-only transition with its reason. Back and reset SHALL list the later stages whose outputs need refresh and SHALL point to native SpecSync status for any reapproval or recovery. Every move SHALL meet the prerequisite of its destination, whichever command makes it: build SHALL require the recorded native change to be approved, implementing or verifying with a valid definition approval; test and accept SHALL require the build prerequisite or the done prerequisite; done SHALL require the change to be archived with authenticated terminal evidence. Unavailable, unknown or undecodable native state, an approval reported invalid, unauthenticated archive evidence and an invalid SDD policy SHALL refuse those moves; a valid disabled policy or no policy SHALL leave them ungated. The helper SHALL NOT record or infer human approval or review. Trace: FLOW-2, FLOW-5, FLOW-10, LIFE-1.

Acceptance Criteria
- Every transition kind is exercised in an isolated fixture; history survives back and reset; skip without a reason is refused.
- Entering build is refused for draft, unavailable and approval-invalid states and allowed for a valid approved state.
- Leaving accept, reset to done and release to done are refused unless the change is archived with authenticated terminal evidence.
- A malformed or ill-typed SDD policy refuses gated moves; a valid disabled policy leaves them ungated.
- Entering a side stage records its caller, and return restores that caller.

### REQ-cli-009

Validated test definitions. The helper SHALL validate test definitions in `.chough/cases/<module>.cases.toml` against profile `chough-testing/1`: unique stable case IDs, active HI criteria, native requirement IDs, track, method, environments, artifact types, explicit expected outcomes, setup and steps for manual or agent-assisted cases, and a binding for automated cases. A binding MAY declare `env`, a bounded table of environment variable names and values; for the `cargo-test` adapter it SHALL NOT declare a variable that chooses what cargo builds or runs, and no binding SHALL declare an expected-output fragment containing the escape character. Unknown fields and unknown profiles SHALL be rejected. Each case SHALL have a definition digest that is independent of its position in the file, and an omitted or empty `env` SHALL leave that digest unchanged. Trace: FLOW-7, LIFE-2.

Acceptance Criteria
- Reordering cases leaves their IDs and digests unchanged; editing expected outcomes, steps or a binding, including its `env`, changes the digest.
- Duplicate IDs, unknown criteria or requirements, unknown fields, missing expected outcomes and automated cases without a binding are refused with the offending location.
- An invalid `env` name, a name over 128 bytes, two names equal ignoring ASCII case, a value with NUL, more than 32 entries or a value over 4 KiB, and an expected-output fragment containing ESC are refused with the offending location.
- A `cargo-test` binding that declares PATH, CARGO, CARGO_HOME, RUSTC, RUSTDOC, RUSTC_WRAPPER, RUSTC_WORKSPACE_WRAPPER, RUSTFLAGS, CARGO_ENCODED_RUSTFLAGS, RUSTDOCFLAGS, CARGO_ENCODED_RUSTDOCFLAGS, RUSTC_BOOTSTRAP, or a name starting with RUSTUP_, CARGO_BUILD_, CARGO_TARGET_ (other than CARGO_TARGET_DIR), CARGO_HOST_, CARGO_PROFILE_ or CARGO_UNSTABLE_, in any ASCII case, is refused with the offending location.

### REQ-cli-010

Focused case selection. The helper SHALL select active cases linked to the given affected criteria, requirements or paths, always including their functional cases, adding copy-localization and design-QA cases only when those tracks are requested as relevant, and SHALL report every omitted case with its reason together with any configured project-required lane. Trace: FLOW-6.

Acceptance Criteria
- An affected requirement selects its linked functional cases and omits unrelated cases with a reason.
- Design-QA and copy-localization cases are selected only when their track is requested.

### REQ-cli-011

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

### REQ-cli-012

Local bug backlog. The helper SHALL persist bug records in `.chough/bugs/` with symptom, trigger, expected and actual behavior, criterion, requirement and case links, environment, revision, evidence references, impact, classification, scope and status. It SHALL list open bugs for a fresh session and SHALL search existing records by symptom, trigger and affected IDs before a new record is added. A bug SHALL remain unverified until a failed observation is linked, and a regression classification SHALL require a passed observation at an ancestor revision. A bug SHALL read as reproduced and later passed, and SHALL accept the verified status, only when a present linked passed observation was observed strictly after its most recent present linked failed observation. Additional evidence SHALL be appended rather than overwritten. The helper SHALL render a report candidate and SHALL NOT publish it. Trace: FLOW-8, LIFE-5.

Acceptance Criteria
- A sample failure becomes a reproduced and classified bug; a second report of the same symptom is found by search and its evidence is appended.
- A regression classification without a passed observation at an ancestor revision is refused.
- An out-of-scope bug stays in the open backlog without changing the checkpoint scope.
- Rendering a report candidate prints Markdown and performs no network action.
- A pass observed before, or in the same second as, the latest linked failure leaves the bug reproduced and the verified status is refused; a bug with no linked failure cannot be marked verified.
- `--classification blocking` is refused with a hint to use `--scope`, and `--scope product-defect` with a hint to use `--classification`.

### REQ-cli-013

Preferences and run consent. The helper SHALL keep standing project preferences in `.chough/preferences.json`, each with value, source and scope, separately from run consents stored in the checkpoint for the current objective. Repeated validation, monitoring, release routing, monitor initiation and the librarian SHALL default to ask, ask, on-request, capture-only and off respectively. A run consent SHALL never be written as a standing preference, and changing the objective SHALL clear run consents. Trace: FLOW-9, FLOW-11.

Acceptance Criteria
- Granting a run consent leaves `.chough/preferences.json` byte-identical; setting a standing preference requires a source.
- Repeated validation is refused without a consent or standing preference, and a new objective clears the consent.
- Failures found by repeated validation create local bug records that remain discoverable after restart; monitor initiation proposes a native change command only when configured and otherwise captures locally.

### REQ-cli-014

Optional release routing. The helper SHALL advance from accept to done and never to release, and SHALL allow entering release only with a release run consent or a standing release preference. Release observations SHALL use distinct kinds for publication, installation, deployment, live health and dry runs, and a dry run SHALL never satisfy a delivery kind. Trace: FLOW-11, LIFE-7.

Acceptance Criteria
- Advancing from accept never enters release, and entering release without authorization is refused.
- A dry-run observation is reported as a dry run, not as deployment or live health.

### REQ-cli-015

Optional librarian. When the librarian is enabled, the helper SHALL copy requested originals into dated snapshots with their source path, Git revision and SHA-256, SHALL regenerate a deterministic index of canonical sources, native change records, open bugs, observations, briefs and snapshots, and SHALL never move, rewrite or replace a canonical source. The librarian SHALL be usable as a side stage from any stage. Trace: FLOW-12.

Acceptance Criteria
- Snapshotting leaves the source byte-identical and records its digest; snapshotting an escaping or symlinked path fails.
- The index lists every open bug, including bugs whose reports were snapshotted.
- Checkpoints and resume work while the librarian is disabled.

### REQ-cli-016

Host skill installation. The helper SHALL install the complete skill bundle for each requested supported host directory, defaulting to `.agents/skills`, SHALL report each installed file as current, edited, known-outdated or missing, and SHALL update only missing files and files byte-identical to a known earlier bundle. Trace: FLOW-1, LIFE-4.

Acceptance Criteria
- Installing for the claude host creates `.claude/skills/chough-workflow/` with the complete bundle, and repeating the install preserves edits.
- An unmodified 0.1.0 bundle file is updated, while an edited file is reported and left unchanged.

### REQ-cli-017

Observation schema 2. The helper SHALL record schema 2 observations that may add case IDs with definition digests, method, track, executed argument vector, exit code, tests run, limitations, the not-run outcome and a working content identity, which SHALL be a SHA-256 digest when present, and SHALL continue to read schema 1 observations, and schema 2 observations without a content identity, with unchanged freshness semantics. Trace: LIFE-2, LIFE-7, FLOW-7.

Acceptance Criteria
- Existing schema 1 records keep their freshness results, and schema 2 records add stale-case-definition.
- The kinds publication-observation, installation-observation, deployment-observation, live-health-observation and dry-run are accepted, and the existing kinds remain valid.
- A schema 2 record without a content identity keeps its revision-bound freshness result, and a record with a malformed content identity is refused.

### REQ-cli-018

Reference-aware retention. The helper SHALL preview by default which observation sets (a record and the artifacts only it references) are eligible for removal and which are protected and why, and SHALL protect sets referenced by the checkpoint, a bug, an active or the most recent archived native change, non-routine records, the newest runs and newest failure per case and environment, validation failures and boundary passes, and recent sets. It SHALL remove eligible sets only under a run consent or a standing retention preference set by a human, SHALL record each removal in an append-only ledger, and SHALL label referenced observations current, historical, pruned or unavailable, never treating pruned or unavailable ones as current or passed. It SHALL NOT change native archives, ignore rules or Git history. Trace: LIFE-8, LIFE-2, LIFE-7, FLOW-7, FLOW-13.

Acceptance Criteria
- The preview changes no file and lists eligible and protected sets with reasons and reclaimed bytes.
- Apply without consent or the standing preference is refused.
- A pruned observation referenced anywhere is reported pruned, and a missing unlisted one unavailable; neither is current or passed.

### REQ-cli-019

Installable package. The crate package SHALL contain the helper source, the canonical skill bundle, Cargo files, README and LICENSE, and SHALL NOT contain Chough's own development records; initializing a new project SHALL install the helper's skills and an empty checkpoint only, and skill status SHALL recognize copies installed from earlier builds. The canonical skill bundle SHALL contain no em dash or en dash. Trace: LIFE-9, LIFE-4.

Acceptance Criteria
- The package list contains no `.chough/`, `.specsync/`, examples, tests or host copies.
- A new project initialized from the installed binary has no bugs, evidence or artifacts.
- No bundle file contains U+2013 or U+2014, and unedited copies of the earlier dashed files are reported known-outdated and updated.

### REQ-cli-020

Timing report. The helper SHALL provide a read-only timing report derived from existing records: stage segments from transitions, including iterations, back, reset, skip, pause and resume, and side-stage enter and return; case observations and repeated-run summaries; bug creation, reproduction and verification; and the native lifecycle times native SpecSync exposes. It SHALL append a timestamped transition when an awaiting-human gate opens and one when it is cleared, and SHALL report active time separately from paused time and from time waiting on a human, without recording who decided or what. Every duration SHALL be labelled elapsed wall-clock time, not effort; history summaries SHALL give ranges with their sample count, labelled estimates rather than targets; a missing, unparseable or out-of-order timestamp SHALL be reported unknown, never zero. Inputs SHALL be bounded, and the report SHALL write no record. Trace: LIFE-10, LIFE-3, LIFE-6, LIFE-7.

Acceptance Criteria
- Fixture transitions with fixed timestamps for every transition kind yield the expected segments, gate records start or end no segment, and paused and waiting-on-human time are excluded from active time.
- Opening a gate appends one gate-opened transition and clearing it one gate-closed transition; the report shows its waiting time, and a gate-opened record whose gate is no longer awaiting a human, with no gate-closed record, is reported closed at an unknown time.
- A missing timestamp or an end before its start is reported unknown, not zero, and fewer than three samples are reported too few to estimate.
- Running the report leaves every Chough and SpecSync record byte-identical, and input beyond its bound is reported partial.

### REQ-cli-021

Focused next action. `chough next` SHALL print a compact action view, separate from the full `chough status` report: the objective; the current stage with its status, iteration and side-stage callers; the recorded native change with its read state, summary state, approval and correction validity, terminal evidence validity, native next action and approved-delta result; the next valid action with its reason and, when one applies, its command; the gate for the next main stage; awaiting-human gates; blockers; open acceptance-blocking bug IDs with the count of other open bugs; and only the referenced observations that are not current, with counts for the rest. It SHALL print text by default and the same fields as JSON with `--json`, and each form SHALL be at most 16384 bytes, with every list bounded, its total stated and any truncation marked. `chough next` and `chough status` SHALL derive the next valid action from the same rules and SHALL each make at most one native status read. In stage done, when the recorded native change is archived with authenticated terminal evidence or no native change is recorded, the next valid action SHALL recommend a new cycle, a stage reset that carries a reason followed by a new native change, rather than repeat the checkpoint's next action; a stage reset SHALL replace the checkpoint's next action. Trace: LIFE-3, FLOW-13, FLOW-2, LIFE-7.

Acceptance Criteria
- With 500 referenced stale observations, 40 awaiting-human gates and 40 blockers of 4 KiB each, and a native response near the parse bound, the text and JSON forms each stay within 16384 bytes, list at most 10 entries per list and state each total.
- The view lists referenced observations that are historical, pruned or unavailable and counts referenced current and unreferenced ones; it carries no raw native output and no observation record.
- `chough next` and `chough status` report the same next valid action for the same records, and each runs `specsync change status` at most once.
- In stage done with an archived, authenticated change, the next valid action recommends `chough stage reset` with `--reason` and `specsync change new`, and the suggested reset succeeds; after a reset the checkpoint's next action is the new-cycle text, and back leaves it unchanged.

### REQ-cli-022

Approved-delta guard. Before a move to build, test or accept on a native change that is approved, implementing or verifying with a valid definition approval, the helper SHALL also confirm that every affected semantic delta matches the digest recorded by the latest native definition approval, computed as SpecSync defines it, reading the native approval ledger and the deltas read-only within a bound. A changed, missing or unapproved delta SHALL refuse the move as delta-changed, and a comparison that cannot be made SHALL refuse it as delta-unverifiable; each refusal SHALL name the next native action. The guard SHALL start no process and SHALL NOT run native verification. Trace: FLOW-5, LIFE-7, LIFE-1.

Acceptance Criteria
- Digests computed for archived changes match the digests their approvals recorded.
- An edited delta, a module added after approval and a deleted delta refuse the move with `refused:native-delta-changed`; a missing, oversized or malformed ledger and an approval without delta digests refuse it with `refused:native-delta-unverifiable`; a refused move leaves the session and transitions byte-identical.
- A change whose deltas match, including after a line-ending-only edit, passes, and a no-spec-change approval with no deltas passes.

