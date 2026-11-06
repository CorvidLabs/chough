# CLI requirements

These IDs describe durable behavior. Passing observations live separately in
`.chough/evidence/`, tied to repository content and environment. FRD provenance
and scope live in [docs/FRD.md](../../docs/FRD.md).

### REQ-cli-001 — Preserve user files

The helper SHALL initialize a resumable session and discoverable workflow skill
without replacing existing files. Trace: LIFE-4, LIFE-6.

Acceptance: repeated init preserves edited session and skill bytes; missing bundle
files can be added without overwriting neighbors.

### REQ-cli-002

Focus context for resumption. The helper SHALL assemble applicable instructions, scoped HI export, selected module contracts and companions, objective, decisions, open questions and next action. The native module lesson source SHALL be `<module>.3md` where present and otherwise the Lessons section of the module `context.md`; the helper SHALL point to it and SHALL NOT keep a lesson store, index or ledger of its own. The shipped skill SHALL tell agents to record each lesson in that source with a certainty label (observation, hypothesis or verified), its source and its limits, and SHALL state that a project decision always outranks a lesson. The shipped skill SHALL ask an agent resuming work to state in its own message, before its first edit, the goal, each recorded decision that binds the next task with its reason, the last failure with its bug and status, the lessons that apply and the next task, and to say what the records do not tell it instead of guessing. Trace: LIFE-3, LIFE-6, LIFE-1, FLOW-12, FLOW-13.

Acceptance Criteria
- The same inputs yield the same ordered context; missing sources and truncation remain explicit.
- Native module .3md is the lesson source where present; otherwise the module context.md is used.
- The canonical skill states the three certainty labels, that each lesson names its source and limits, and that a project decision outranks a lesson; no Chough lesson store, index or ledger exists.
- The canonical skill's resume steps ask the agent to state the goal, binding decisions with reasons, the last failure with its bug status, the applicable lessons and the next task in its own message before its first edit, and to name what the records do not say.

### REQ-cli-003

The helper SHALL report actual native SpecSync status and guidance, with unavailable, failed, undecodable, oversized or timed-out responses explicitly unknown. A failed native command SHALL be shown with its result in plain words, `exit <code>` or `killed by signal <number>`, never a program's internal debug form. Native reads SHALL retain at most the parse bound of standard output and a display bound of standard error while reading, and SHALL end by a bounded deadline. Trace: LIFE-1, LIFE-6, LIFE-7.

Acceptance Criteria
- Missing SpecSync never becomes an approved or delivered state, and an agent-written session next action never grants approval or overrides a native gate.
- A native command that never exits is reported as timed-out by its deadline, and gated moves refuse it.
- Output beyond the bounds is counted and reported, not retained.
- A native command that exits 2 is shown as `exit 2`, and one ended by SIGTERM as `killed by signal 15`.

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

Canonical checkpoint and explicit migration. The helper SHALL keep one canonical checkpoint at `.chough/session.json` or, in sessions mode, the active session file (REQ-cli-037), schema version 2, holding the objective, intent scope, modules, current stage with its status and side-stage callers, the native change identity, referenced criteria, requirements, cases, design sources and files, decisions, open questions, completed work, observation and bug references, blockers, awaiting-human gates, run consents and the next action. It SHALL migrate a schema 1 session only through an explicit command that first preserves the original bytes, and SHALL keep reading schema 1 sessions for context and status. Trace: FLOW-13, LIFE-3, DRIVE-3.

The helper SHALL provide bounded `chough checkpoint --merge <base> <ours> <theirs>` for three-way reconciliation of schema 2 checkpoints. For `decisions`, `open_questions`, `completed`, `observations`, `bugs` and every collection under `references`, entries SHALL be compared as exact strings. Base entries not removed on either side SHALL retain base order, entries added by ours SHALL follow in ours order with an identical addition on both sides included once, and remaining entries added by theirs SHALL follow in theirs order. An entry removed on only one side SHALL remain removed.

The complete values of `objective`, `intent_scope`, `modules`, `stage`, `native_change`, `next_action`, `run_consents`, `blockers` and `awaiting_human` SHALL form one current-cycle group. If neither side changes that group from base, the merge SHALL select base; if exactly one side changes it, the merge SHALL select that side's complete group; if both sides make the identical change, the merge SHALL select that group. If both sides change it differently, the merge SHALL refuse with a current-cycle conflict reason rather than combine fields or select a side.

Every input SHALL be read within the existing checkpoint bounds and validated without unknown fields. All three inputs SHALL use schema version 2, and the merged result SHALL satisfy every existing checkpoint bound before it is written. A successful merge SHALL write the result only to `<ours>`, leaving `<base>` and `<theirs>` byte-identical. Every refusal SHALL leave all three inputs byte-identical and SHALL create no checkpoint, transition, gate, consent, approval, review or other record. This reconciliation path SHALL NOT change one-owner live writes, native SpecSync authority or any non-checkpoint ID scheme, and SHALL NOT introduce a second writable checkpoint store.

The repository SHALL declare `.chough/session.json merge=chough-session` in `.gitattributes`, and the same driver for `.chough/sessions/*.json` and `.chough/sessions/archive/*.json`. Documentation SHALL give the per-clone command `git config merge.chough-session.driver 'chough checkpoint --merge %O %A %B'`; initialization SHALL NOT change Git configuration. The explicit merge command SHALL remain the fallback when no driver is configured.

When SpecSync is configured, `chough checkpoint --native-change <id>` SHALL read `specsync change status <id>` before it applies any update, and SHALL refuse as `refused:native-change-unknown`, naming the ID and that status command, when SpecSync does not report the change; a failed or timed-out read SHALL also refuse, and a reply whose `summary.id` or `change.id` names a different change SHALL refuse as `refused:native-change-mismatch`, naming both IDs. A JSON reply that names neither `summary.id` nor `change.id` SHALL refuse as `refused:native-change-unknown`, since it does not identify the requested change. Every such refusal SHALL leave `.chough/session.json` byte-identical, including the other updates given in the same call. An ID SpecSync reports, archived or open, SHALL be saved as before. Without a SpecSync configuration only the existing ID format check SHALL apply, with no native read.

Acceptance Criteria
- A checkpoint saved by one process is read unchanged by `chough context` and `chough next` in another process; unknown fields and unsupported versions are rejected.
- `chough migrate` preserves the schema 1 bytes in `.chough/session.v1.json`, refuses to overwrite that backup, and leaves the original session unchanged when migration fails.
- Stage commands refuse a schema 1 session with a migration instruction, while context and status still work.
- No second writable checkpoint store is introduced and no adapter such as APS is required.
- For a collection with base `[a, b, c]`, ours `[a, b, d, e]` and theirs `[b, c, d, f]`, the merged collection is `[b, d, e, f]`: the one-side removals of `a` and `c` survive, the shared addition `d` appears once, and base, ours and theirs ordering is preserved. The same rule passes for every history and reference collection named above.
- Selecting base, one changed side or identical current-cycle groups produces that complete group. Differently changed groups refuse even when their edited fields do not overlap, and the diagnostic names the current-cycle conflict.
- A successful explicit merge changes only ours, while base and theirs remain byte-identical. Every conflict or validation refusal leaves base, ours and theirs byte-identical and creates no checkpoint, transition, gate, consent, approval, review or other record.
- Schema 1 in any position, mixed schema versions, malformed JSON or record content, unknown fields, unsupported versions and a result exceeding an existing item, caller or 1048576-byte checkpoint bound fail closed.
- A real Git fixture using the `.gitattributes` declaration and documented per-clone driver automatically merges compatible checkpoint edits and leaves a conflicting current-cycle merge unresolved with a nonzero driver result. Init leaves Git configuration byte-identical, and the explicit command performs the same compatible merge without driver configuration.
- Real Git evidence first records the pre-implementation conflicting checkpoint merge failure and links it to a local bug, then records a strictly later pass after implementation. The evidence uses explicit readable IDs without allocating a new case number, and all non-checkpoint ID schemes remain unchanged.
- With SpecSync configured, `checkpoint --native-change` for an ID SpecSync does not report, or after a failed or timed-out status read, exits non-zero with `refused:native-change-unknown` or the read failure and leaves the checkpoint byte-identical even with other updates in the same call; a reply naming a different change refuses as `refused:native-change-mismatch` with the checkpoint byte-identical; a JSON reply naming no change ID, such as `[]` or `{}`, refuses as `refused:native-change-unknown` with the checkpoint byte-identical; a reported ID saves; without SpecSync any well-formed ID saves and a malformed one is refused.
- The repository's `.gitattributes` declares the `chough-session` driver for `.chough/session.json`, `.chough/sessions/*.json` and `.chough/sessions/archive/*.json`.

### REQ-cli-008

Stage routing bounded by native gates. The helper SHALL route the stages intent, design, plan, build, test, accept and done, the optional release stage, and the side stages bugs, validate and librarian. It SHALL record each advance, iterate, back, reset, pause, resume, skip and side-stage enter or return as an append-only transition with its reason. Back and reset SHALL list the later stages whose outputs need refresh and SHALL point to native SpecSync status for any reapproval or recovery. Every move SHALL meet the prerequisite of its destination, whichever command makes it: build SHALL require the recorded native change to be approved, implementing or verifying with a valid definition approval; test and accept SHALL require the build prerequisite or the done prerequisite; done SHALL require the change to be archived with authenticated terminal evidence. Unavailable, unknown or undecodable native state, an approval reported invalid, unauthenticated archive evidence and an invalid SDD policy SHALL refuse those moves; a valid disabled policy or no policy SHALL leave them ungated. The helper SHALL NOT record or infer human approval or review. Leaving test for accept, by advance or skip, SHALL also be refused with `refused:unlinked-failure` while the latest executed (passed or failed) observation, by recorded time and then ID, of a case referenced in the checkpoint failed and no local bug links that observation; a later `not-run` observation executed nothing and SHALL NOT replace it; the refusal SHALL name the case and the observation and SHALL say to rerun the case or link the observation to a bug. A later passed observation of that case, or any local bug linking the failed observation, SHALL lift it; stale passed observations, cases with only `not-run` observations and cases not referenced in the checkpoint SHALL NOT trigger it. At test, `chough next` and `chough status` SHALL name the same refusal from the same rule. Trace: FLOW-2, FLOW-5, FLOW-10, LIFE-1, FLOW-7, LIFE-5.

Acceptance Criteria
- Every transition kind is exercised in an isolated fixture; history survives back and reset; skip without a reason is refused.
- Entering build is refused for draft, unavailable and approval-invalid states and allowed for a valid approved state.
- Reaching test or accept is refused for draft, unavailable and approval-invalid states and allowed for a valid approved state or an archive with authenticated terminal evidence.
- Leaving accept, reset to done and release to done are refused unless the change is archived with authenticated terminal evidence.
- A malformed or ill-typed SDD policy refuses gated moves; a valid disabled policy leaves them ungated.
- Entering a side stage records its caller, and return restores that caller.
- Leaving test with a referenced case whose latest executed (passed or failed) observation failed and that no bug links is refused with `refused:unlinked-failure` naming the case, the observation and both remedies, for advance and skip, even when a later observation is only `not-run`; a later pass or a linking bug of any status allows it; a stale passed observation, a not-run-only case or an unreferenced failed case does not refuse; at test, next and status name the same refusal.

### REQ-cli-009

Validated test definitions. The helper SHALL validate test definitions in `.chough/cases/<module>.cases.toml` against profile `chough-testing/1`: unique stable case IDs, active HI criteria, native requirement IDs, track, method, environments, artifact types, explicit expected outcomes, setup and steps for manual or agent-assisted cases, and a binding for automated cases. A binding MAY declare `env`, a bounded table of environment variable names and values; for the `cargo-test` adapter it SHALL NOT declare a variable that chooses what cargo builds or runs, and no binding SHALL declare an expected-output fragment containing the escape character. Unknown fields and unknown profiles SHALL be rejected. Each case SHALL have a definition digest that is independent of its position in the file, and an omitted or empty `env` SHALL leave that digest unchanged. New case definitions SHALL use curated IDs in the form `TC-<module>-<title-slug>`, where the title slug consists of nonempty lowercase ASCII alphanumeric runs separated by single hyphens. The complete ID SHALL remain within the existing 100-byte record ID bound using a module-aware slug limit. Case IDs SHALL be globally unique across all modules, including legacy and new forms. Exact legacy `TC-<module>-NNNN` definitions SHALL remain valid with unchanged definition digests and without renumbering or migration. New regression case definitions introduced for this change SHALL use slug IDs. Documentation SHALL state that older Chough versions require an upgrade to use slug case IDs. A requirement ID the companion lacks SHALL be accepted only when the checkpoint's native change, on one native read per command, passes the build gate checks of REQ-cli-022 and its delta for the case module adds or modifies that ID; otherwise it SHALL be refused, naming why a consulted delta did not resolve it. The shipped skill SHALL say so, SHALL show the test stage linking failing-first evidence, and SHALL tell agents not to list running or recording cases as a `tasks.md` checkbox. Trace: FLOW-7, LIFE-2, LIFE-6, DRIVE-6.

Acceptance Criteria
- Reordering cases leaves their IDs and digests unchanged; editing expected outcomes, steps or a binding, including its `env`, changes the digest.
- Duplicate IDs, unknown criteria or requirements, unknown fields, missing expected outcomes and automated cases without a binding are refused with the offending location.
- An invalid `env` name, a name over 128 bytes, two names equal ignoring ASCII case, a value with NUL, more than 32 entries or a value over 4 KiB, and an expected-output fragment containing ESC are refused with the offending location.
- A `cargo-test` binding that declares PATH, CARGO, CARGO_HOME, RUSTC, RUSTDOC, RUSTC_WRAPPER, RUSTC_WORKSPACE_WRAPPER, RUSTFLAGS, CARGO_ENCODED_RUSTFLAGS, RUSTDOCFLAGS, CARGO_ENCODED_RUSTDOCFLAGS, RUSTC_BOOTSTRAP, or a name starting with RUSTUP_, CARGO_BUILD_, CARGO_TARGET_ (other than CARGO_TARGET_DIR), CARGO_HOST_, CARGO_PROFILE_ or CARGO_UNSTABLE_, in any ASCII case, is refused with the offending location.
- Curated slug IDs accept lowercase ASCII normalization and enforce the module-aware 100-byte bound; empty, uppercase, repeated-separator and leading or trailing separator forms are refused. A duplicate ID in another module is refused globally.
- Exact legacy numeric cases remain readable and executable with unchanged definition digests and no renumbering or migration. The new regression case IDs for this change use slugs rather than allocating legacy numbers.
- A case linking a requirement only the approved, unchanged delta adds passes `cases check`, `run` and `record` on one native read and is listed under `approved_delta_requirements`; an edited delta, invalid approval, wrong state or module, no change or a failed read refuses with the reason.
- The plan, testing and accept references describe this and failing-first evidence in the test stage.

### REQ-cli-010

Focused case selection. The helper SHALL select active cases linked to the given affected criteria, requirements or paths, always including their functional cases, adding copy-localization and design-QA cases only when those tracks are requested as relevant, and SHALL report every omitted case with its reason together with any configured project-required lane. Trace: FLOW-6.

Acceptance Criteria
- An affected requirement selects its linked functional cases and omits unrelated cases with a reason.
- Design-QA and copy-localization cases are selected only when their track is requested.

### REQ-cli-011

Honest case execution. When explicitly invoked, the helper SHALL execute an automated case's binding directly without a shell, with the binding's declared environment set on top of the inherited one, capture its output as an artifact, and record an observation linked to the case and its definition digest. Execution SHALL end by the binding's deadline: on the deadline, or when output stays open after the command exits, the helper SHALL stop the command and its descendants and SHALL NOT wait on output it cannot complete. On Unix, when the helper is stopped by SIGINT, SIGTERM or SIGHUP while it runs a binding, a native read or one of its own Git commands, or while it saves what a binding reported, it SHALL stop every command it started together with that command's process group and, on Linux, every descendant that left that group (for example with `setsid`), including one orphaned by its parent, record nothing partial and end by that signal: a stop before an automated case's save begins SHALL record no observation, artifact or checkpoint change, and a stop that arrives once the save has begun SHALL let the artifact and the observation both be written before the helper ends by that signal. A signal inherited as ignored, or as blocked in the signal mask, SHALL keep its inherited disposition. When a descendant of one of the helper's own Git commands keeps that command's output open past the grace period and is stopped, the helper SHALL say so on stderr; when its `git rev-parse HEAD` fails for a reason other than an absent repository or a HEAD with no commit, it SHALL say so on stderr and record no revision. Captured output SHALL be bounded per stream while reading. The artifact SHALL keep the captured bytes, while assertions SHALL be evaluated on that text with complete terminal escape sequences removed. Removal SHALL never remove a line break, and a sequence that does not complete before a line break or the end of the output SHALL be left unchanged. The observation SHALL be passed only when the process exits zero, its capture is complete and the binding's assertion holds: exactly one test ran for the `cargo-test` adapter, or every declared expected-output fragment appeared for the `command` and `fledge-task` adapters. Every observation of an executed case SHALL state as a limitation that inherited environment variables were not recorded; when the binding declares variables it SHALL also name them without their values, and when escape sequences were removed it SHALL say so. A run in which no test matched SHALL be recorded as not-run. A repeated run SHALL record each attempt and a summary with attempt and failure counts and elapsed time, and SHALL start no further attempt once a stop request or a paused checkpoint exists, including after waiting its interval. Automatic observation and repeated validation run allocation SHALL follow REQ-cli-023. An explicit observation ID SHALL remain optional for automated and manual case recording; when omitted, the helper SHALL allocate the new OBS form, and when supplied for a new write it SHALL use that form rather than an old-style ID. An old-style supplied ID SHALL be refused with a remedy to omit the ID. The automated artifact name SHALL remain exactly `.chough/artifacts/<observation-id>.log`, inheriting the allocated or explicit observation ID. Legacy observation and validation run IDs SHALL remain readable and resumable. Trace: FLOW-7, FLOW-9, LIFE-7, FLOW-14.

Acceptance Criteria
- A deliberately failing assertion records failed even when another case passes in the same session.
- A cargo filter that matches no test records not-run, never passed.
- Changing a case definition reports its earlier observations as stale-case-definition.
- A case whose descendant holds its output returns by its deadline, and its descendants are stopped.
- SIGINT, SIGTERM or SIGHUP during `cases run` before its save begins, during a native read in `next`, or during the Git that `stage backfill` or `library snapshot` runs before writing leaves no process Chough started, Git included, and no new or changed `.chough` or library file; a stop once a case run's save has begun leaves both its artifact and its observation, and no process Chough started.
- A stop request made during the interval halts a repeated run before the next attempt, and resuming continues with the stored limits.
- A declared variable reaches the command, and the observation names it without its value.
- An observation of a case that declares no variables still states that inherited environment variables were not recorded.
- An escape sequence interrupted by a line break or the end of the output is left unchanged, and removal never lowers the number of lines that start with `test result:`.
- Coloured output whose visible text holds a fragment passes with the escape limitation, the artifact stays byte-exact, and a fragment absent from the visible text still fails.
- Omitting the observation ID allocates `OBS-<slug>-<24-lowercase-hex>`, identical repeated runs receive distinct random IDs, and each automated artifact is exactly `.chough/artifacts/<observation-id>.log` with the same observation ID.
- Repeated validation allocates `VAL-<slug>-<24-lowercase-hex>` with a random 96-bit suffix. Legacy observation and validation IDs remain readable and resumable, while an old-style ID supplied for a new case observation fails with an omit-ID remedy.
- On Linux, SIGTERM during `cases run` also stops a descendant that called `setsid`, and one whose parent exited after starting it with `setsid`.
- A Git descendant that holds output past the grace period is stopped with a stderr notice, and a failed `git rev-parse HEAD` is reported on stderr while the transition is still recorded.

### REQ-cli-012

Local bug backlog. The helper SHALL persist bug records in `.chough/bugs/` with symptom, trigger, expected and actual behavior, criterion, requirement and case links, environment, revision, evidence references, impact, classification, scope and status. It SHALL list open bugs for a fresh session and SHALL search existing records by symptom, trigger and affected IDs before a new record is added. A bug SHALL remain unverified until a failed observation is linked, and a regression classification SHALL require a passed observation at an ancestor revision. A bug SHALL read as reproduced and later passed, and SHALL accept the verified status, only when a present linked passed observation was observed strictly after its most recent present linked failed observation. Additional evidence SHALL be appended rather than overwritten. The helper SHALL render a report candidate and SHALL NOT publish it. Every new bug SHALL use an ID in the form `BUG-<lowercase-title-slug>-<24-lowercase-hex>`, generated once by the same generator for manual `bug add` and automatic failure capture. The title slug SHALL be derived by lowercasing ASCII letters, replacing each maximal run of characters outside ASCII `a` to `z` and `0` to `9` with one hyphen, and trimming leading and trailing hyphens; if the result is empty it SHALL be `bug`. The slug SHALL be truncated to at most 71 bytes and any trailing hyphen left by truncation SHALL be removed, so the complete ID is at most 100 bytes and the final hyphen and all 24 digest digits are preserved. The digest suffix SHALL be the first 24 lowercase hexadecimal digits of SHA-256 over a canonical creation payload containing a format version, a bug-identity domain separator and all initial bug fields other than the ID and update timestamp. An update SHALL NOT recompute the ID or rename the record, even when stored creation content has changed. The bug record schema SHALL remain 1. Existing `BUG-NNNN` records SHALL remain readable and usable without migration or renumbering, and duplicate and related links SHALL resolve in either direction across legacy and new forms. New records SHALL be written by atomic create-new with no overwrite; a same-ID different-content result SHALL be refused, SHALL leave the existing record byte-identical and SHALL NOT fall back to numeric allocation. Documentation SHALL state that older Chough versions need an upgrade to use new-form IDs. Trace: FLOW-8, LIFE-5.

Acceptance Criteria
- A sample failure becomes a reproduced and classified bug; a second report of the same symptom is found by search and its evidence is appended.
- A regression classification without a passed observation at an ancestor revision is refused.
- An out-of-scope bug stays in the open backlog without changing the checkpoint scope.
- Rendering a report candidate prints Markdown and performs no network action.
- A pass observed before, or in the same second as, the latest linked failure leaves the bug reproduced and the verified status is refused; a bug with no linked failure cannot be marked verified.
- `--classification blocking` is refused with a hint to use `--scope`, and `--scope product-defect` with a hint to use `--classification`.
- Normalizing `  Fix   LOGIN... now!  ` produces the slug `fix-login-now`, a title with no ASCII alphanumeric character produces `bug`, and an overlong title produces an ID of at most 100 bytes with its complete final hyphen and 24 lowercase hexadecimal digest digits.
- Both manual `bug add` and automatic failure capture produce the new ID form with the normalized title slug and the first 24 lowercase hexadecimal SHA-256 digits for their canonical creation payloads; a known-answer vector fixes the payload version and domain separator.
- Changing a stored title or other content and then updating status or evidence leaves the bug ID and file name unchanged.
- TC-cli-0125 first records and links the pre-fix failure in which independent roots at the same legacy numeric high-water mark allocate the same ID, then records a later pass in which distinct manual and automatic reports in those roots receive different new-form IDs.
- A legacy `BUG-NNNN` record remains schema 1 and can be listed, searched, shown, updated and reported without renumbering or migration; duplicate and related links succeed from legacy to new and new to legacy.
- When different initial content would use an existing ID, atomic create-new refuses it, leaves the existing file byte-identical and creates no numeric fallback or replacement record.
- The CLI documentation tells users of older Chough versions to upgrade for new-form bug IDs.

### REQ-cli-013

Preferences and run consent. The helper SHALL keep standing project preferences in `.chough/preferences.json`, each with value, source and scope, separately from run consents stored in the checkpoint for the current objective. Repeated validation, monitoring, release routing and monitor initiation SHALL default to ask, ask, on-request and capture-only respectively; the librarian SHALL be on by default, as REQ-cli-015 states, and off only by a recorded preference. A run consent SHALL never be written as a standing preference, and changing the objective SHALL clear run consents. Trace: FLOW-9, FLOW-11, FLOW-12.

Acceptance Criteria
- Granting a run consent leaves `.chough/preferences.json` byte-identical; setting a standing preference requires a source.
- Repeated validation is refused without a consent or standing preference, and a new objective clears the consent.
- Failures found by repeated validation create local bug records that remain discoverable after restart; monitor initiation proposes a native change command only when configured and otherwise captures locally.
- With no preference recorded, `chough pref show` reports the librarian `on` with origin `default`, and no requirement gives it another default.

### REQ-cli-014

Optional release routing. The helper SHALL advance from accept to done and never to release, and SHALL allow entering release only with a release run consent or a standing release preference. Release observations SHALL use distinct kinds for publication, installation, deployment, live health and dry runs, and a dry run SHALL never satisfy a delivery kind. Trace: FLOW-11, LIFE-7.

Acceptance Criteria
- Advancing from accept never enters release, and entering release without authorization is refused.
- A dry-run observation is reported as a dry run, not as deployment or live health.
- Publication, installation, deployment and live-health observations are each accepted and reported with their own kind, and each is key-item proof: retention protects it with its kind as the reason and moves it into the vault when its archive turns cold. A dry run's kind is never a protection, and a superseded dry run is trimmed with no vault copy.

### REQ-cli-015

Project librarian. The librarian SHALL be on by default in `knowledge/` (or `library-dir`) and off only by a recorded preference. It SHALL copy requested originals into snapshots identified by REQ-cli-023 with source path, Git revision and SHA-256, SHALL never move, rewrite or replace a canonical source, and SHALL regenerate a deterministic index of canonical sources, native changes, open bugs, observations, briefs, snapshots, an earlier `library/` folder as is, a lessons index of pointers, and one entry per archived change with its criteria, commits, listed files with digests and locations, outcomes, bugs and lesson bundle. No gate SHALL read it. Snapshot IDs SHALL be `SNAP-<slug>-<24-lowercase-hex>` with a random 96-bit suffix; legacy dated IDs SHALL stay readable and listed without migration. It SHALL be usable as a side stage from any stage. Trace: FLOW-12, KNOW-1, KNOW-2, KNOW-3, KNOW-4, KNOW-5, KNOW-6, KNOW-12.

Acceptance Criteria
- Snapshotting leaves the source byte-identical and records its digest; an escaping or symlinked path fails.
- The index lists every open bug, including snapshotted ones.
- Checkpoints and resume work with the librarian off; with no preference it is on in `knowledge/`.
- Repeated identical snapshots get distinct SNAP IDs, keeping digests and timestamps in the manifest.
- Legacy dated snapshots stay readable and indexed beside new ones, and same-ID different-content creation refuses without overwrite, fallback or retry.
- Regenerating gives identical bytes; archive entries list records and artifacts with digests and locations; the lessons index has no lesson text.

### REQ-cli-016

Host skill installation. The helper SHALL install the complete skill bundle once for each distinct supported host directory requested, defaulting to `.agents/skills`, with the claude host sharing `.agents/skills`; SHALL report each installed file as current, edited, known-outdated or missing; SHALL update only missing files and files byte-identical to a known earlier bundle; SHALL report an installed legacy `.claude/skills/chough-workflow/` copy as legacy without writing or deleting it during init, status or update; and SHALL remove legacy files only through an explicit remove-legacy command that requires the shared copy, removes only files byte-identical to the current or a known earlier bundle and keeps every other file. Trace: FLOW-1, LIFE-4, LIFE-12, LIFE-13, LIFE-14.

Acceptance Criteria
- Installing for the claude host, alone or together with openai, creates only `.agents/skills/chough-workflow/` with the complete bundle and no `.claude/` directory, and repeating the install preserves edits.
- A bundle file byte-identical to one from a known earlier build is updated, while an edited file is reported and left unchanged.
- An installed `.claude/skills/chough-workflow/` copy is reported as legacy by status and update and stays byte-identical; remove-legacy refuses with nothing removed while the shared copy is missing, and otherwise removes current and known-outdated legacy files and keeps edited or unknown ones.

### REQ-cli-017

Observation schema 2. The helper SHALL record schema 2 observations that may add case IDs with definition digests, method, track, executed argument vector, exit code, tests run, limitations, the not-run outcome and a working content identity, which SHALL be a SHA-256 digest when present, and SHALL continue to read schema 1 observations, and schema 2 observations without a content identity, with unchanged freshness semantics. Trace: LIFE-2, LIFE-7, FLOW-7.

Acceptance Criteria
- Existing schema 1 records keep their freshness results, and schema 2 records add stale-case-definition.
- The kinds publication-observation, installation-observation, deployment-observation, live-health-observation and dry-run are accepted, and the existing kinds remain valid.
- A schema 2 record without a content identity keeps its revision-bound freshness result, and a record with a malformed content identity is refused.

### REQ-cli-019

Installable package. The crate package SHALL contain the helper source, the canonical skill bundle, Cargo files, README and LICENSE, and SHALL NOT contain Chough's own development records; initializing a new project SHALL install the helper's skills and an empty checkpoint only, and skill status SHALL recognize copies installed from earlier builds. The canonical skill bundle SHALL contain no em dash or en dash. The package SHALL declare its minimum supported Rust version, and that version SHALL be built and tested in CI. The README SHALL give one command that installs the HI, SpecSync and Fledge versions CI uses, and the versions named in the README, the agent instructions and CI SHALL agree. Trace: LIFE-9, LIFE-4, LIFE-6.

Acceptance Criteria
- The package list contains no `.chough/`, `.specsync/`, examples, tests or host copies.
- A new project initialized from the installed binary has no bugs, evidence or artifacts.
- No bundle file contains U+2013 or U+2014, and unedited copies of the earlier dashed files are reported known-outdated and updated.
- Cargo.toml declares a rust-version, and a CI job builds and tests with that Rust version.
- The README, AGENTS.md and the CI workflow name the same HI, SpecSync and Fledge versions; Cargo.toml, the README and the CI job for the minimum version name the same Rust version; a test fails naming the file when one is missing or disagrees.

### REQ-cli-021

Focused next action. `chough next` SHALL print a compact action view, separate from the full `chough status` report: the objective; the current stage with its status, iteration and side-stage callers; the recorded native change with its read state, summary state, approval and correction validity, terminal evidence validity, native next action and approved-delta result; the next valid action with its reason and, when one applies, its command; the gate for the next main stage; awaiting-human gates; blockers; the checkpoint's open questions; open acceptance-blocking bug IDs with the count of other open bugs; only the referenced observations that are not current, with counts for the rest; the most recent checkpoint decisions, at most 5 with their total, showing the kind and reason of entries that follow the `decided:`, `rejected:` or `failed:` prefix convention and other entries whole; the latest failed observation by its recorded observation time, with the status of each bug that links it; and a pointer to each module's native lesson source with the count and bounded paths of archived SpecSync lesson bundles, without reading lesson text or tracking which bundles were folded; the bundles named SHALL be the newest 3 by recorded finalization timestamp, then directory, as the librarian orders its hot archives, with an unreadable timestamp sorting oldest. `chough status` SHALL report the same open questions, all of them, in its checkpoint section. It SHALL print text by default and the same fields as JSON with `--json`, and each form SHALL be at most 16384 bytes, with every list bounded, its total stated and any truncation marked. The same records SHALL give byte-identical output; ordering SHALL use recorded fields and names, never file modification times. `chough next` and `chough status` SHALL derive the next valid action from the same rules and SHALL each make at most one native status read. In stage done, when the recorded native change is archived with authenticated terminal evidence or no native change is recorded, the next valid action SHALL recommend a new cycle, a stage reset that carries a reason followed by a new native change, rather than repeat the checkpoint's next action; a stage reset SHALL replace the checkpoint's next action. In an earlier main stage, when the recorded native change is archived with authenticated terminal evidence and no pause, human gate or blocking bug applies, the next valid action SHALL NOT continue the stage; it SHALL say the change is archived and recommend the new cycle, after recording done first at accept. An archived change finalized no later than the latest reset, or reached directly by one, is the previous cycle's: before done the action SHALL keep the reset's guidance and, when the next gate is refused, name `specsync change new`, never the archived refusal or another reset. With no native change recorded, a refused `refused:no-native-change` gate SHALL name `specsync change new`, not a bare status read. Trace: LIFE-3, FLOW-13, FLOW-2, LIFE-7, LIFE-1, FLOW-12, DRIVE-3.

Acceptance Criteria
- With 500 referenced stale observations, 40 awaiting-human gates and 40 blockers of 4 KiB each, 50 decisions of 4 KiB each (a checkpoint within the 1,048,576-byte cap of REQ-cli-032), 100 failed observations, 40 bugs linking the latest failure, 40 archived lesson bundles, 16 modules in the checkpoint and a native response near the parse bound, the text and JSON forms each stay within 16384 bytes, list at most 10 entries per list and state each total.
- The view lists referenced observations that are historical, pruned or unavailable and counts referenced current and unreferenced ones; it carries no raw native output, no observation record and no lesson text.
- The view shows the last 5 decisions newest first with the kind and reason of prefixed entries and unprefixed entries whole; the failed observation with the greatest recorded time, ties broken by the greater observation ID, with each linking bug's status; and each module's lesson source path, at most 10 with their total, with the archived bundle count and up to 3 paths, newest finalized first, matching the hot archives of `knowledge/INDEX.md`.
- Two runs over the same records, and a run after file modification times change, give byte-identical text and JSON.
- `chough next` and `chough status` report the same next valid action for the same records, and each runs `specsync change status` at most once.
- In stage done with an archived, authenticated change, the next valid action recommends `chough stage reset` with `--reason` and `specsync change new`, and the suggested reset succeeds; after a reset the checkpoint's next action is the new-cycle text, and back leaves it unchanged.
- At intent, design, plan, build or test with an archived, authenticated change and no pause, human gate or (at test) blocking bug, the next valid action recommends the same new cycle and not `Continue <stage>`; at accept it names `chough stage advance` to done first.
- After a reset from done to plan, or to intent and then advance to plan, with the old change still recorded, the command is `specsync change new`, not the archived refusal or a reset; a change finalized after the reset keeps the archived guidance. With no change recorded, the plan gate's command is `specsync change new`.
- An open question recorded with `checkpoint --question`, or folded from a session inbox, appears in the text and JSON of `next`, at most 10 with its total like blockers, and in `status`; resolving it removes it from both.

### REQ-cli-022

Approved-delta guard. Before a move to build, test or accept on a native change that is approved, implementing or verifying with a valid definition approval, the helper SHALL also confirm that every affected semantic delta matches the digest recorded by the latest native definition approval, computed as SpecSync defines it, reading the native approval ledger and the deltas read-only within a bound. A changed, missing or unapproved delta SHALL refuse the move as delta-changed, and a comparison that cannot be made SHALL refuse it as delta-unverifiable; each refusal SHALL name the next native action, and a remedy that names a native approval SHALL name the acting agent account as the actor with the human authority source, covered scope and limits in the note, never the human's name or a request that the human run it. The guard SHALL start no process and SHALL NOT run native verification. Trace: FLOW-5, LIFE-7, LIFE-1.

Acceptance Criteria
- Digests computed for archived changes match the digests their approvals recorded.
- An edited delta, a module added after approval and a deleted delta refuse the move with `refused:native-delta-changed`; a missing, oversized or malformed ledger and an approval without delta digests refuse it with `refused:native-delta-unverifiable`; a refused move leaves the session and transitions byte-identical.
- A change whose deltas match, including after a line-ending-only edit, passes, and a no-spec-change approval with no deltas passes.
- A refusal whose remedy is a new definition approval names `specsync change approve <id> --actor "<agent-account>" --note "Human authority: <source>; covered scope: <scope>; limits: <limits>"` and does not ask the human to approve or name them as the actor.

### REQ-cli-023

Slug and random record identities. Automatically allocated case observation, repeated validation run and librarian snapshot IDs SHALL use `OBS-<slug>-<24-lowercase-hex>`, `VAL-<slug>-<24-lowercase-hex>` and `SNAP-<slug>-<24-lowercase-hex>` respectively. The slug SHALL come from the case ID for observations and validation runs and from the snapshot reason for snapshots. It SHALL be normalized to nonempty lowercase ASCII alphanumeric runs separated by single hyphens, with a nonempty fallback when the source has no ASCII alphanumeric character, and bounded so the complete ID remains within the existing 100-byte record ID bound without truncating the final hyphen or random suffix. Each suffix SHALL encode 96 freshly generated random bits as exactly 24 lowercase hexadecimal digits, obtained through direct use of the existing getrandom 0.4.3 package. No runtime or service SHALL be added. Timestamps SHALL remain in their existing fields and determine chronology rather than identity.

Random allocation SHALL let identical repeated runs remain distinct history without timestamp, numeric or content-digest allocation. Observation, validation run, snapshot and automated artifact creation SHALL use atomic create-new. A same-ID different-content creation SHALL be refused without overwriting the existing bytes and without numeric, timestamp or digest fallback, another random draw or any other retry. Random-source and write failures SHALL preserve existing records.

Every legacy observation, validation run and snapshot ID SHALL remain readable, linkable and, where supported, resumable without migration or schema rewrite. An old-style ID supplied for a new automatically allocated write SHALL be refused with a remedy to omit the ID. Documentation SHALL state that older Chough versions require an upgrade to use the new-form IDs. Bug identity generation and its known-answer vectors SHALL remain unchanged. Transition, preference, consent, stop-marker and pruning-ledger entry shapes SHALL NOT gain IDs. Trace: FLOW-7, FLOW-9, FLOW-12, FLOW-13, LIFE-2, LIFE-3, LIFE-7.

Acceptance Criteria
- Identical case runs, repeated validation runs and snapshots in independent roots receive distinct IDs of their required forms, each with a different 96-bit random suffix represented by 24 lowercase hexadecimal digits; timestamps remain in their existing fields.
- A forced same-ID different-content creation refuses atomically, leaves every pre-existing record byte-identical and creates no overwrite, numeric, timestamp or digest fallback, regenerated suffix or replacement record. Random-source and write failures preserve existing records.
- Mixed legacy and new IDs work through case and bug links, checkpoint references, status and next, timing, retention and pruning, native manifests, validation resume and librarian indexes without migration or schema rewrite. Old-style IDs supplied for new automatic writes fail with an omit-ID remedy, and documentation gives the old-version upgrade boundary.
- Bug known-answer vectors and generated IDs are unchanged, and transition, preference, consent, stop-marker and pruning-ledger entry shapes gain no IDs.
- A test with a test-only pinned clock runs the same bound case in two independent Git roots on separate branches at the same second; after the branches merge, the observation IDs and artifact paths are distinct, both records keep the pinned time, and both artifacts keep their recorded bytes and fingerprints, so both observations read as current.
- In a real Git fixture, two roots on separate branches independently add case-run observations with artifacts, `evidence add` observations and bugs; the merge has no conflicts, every record from both sides is byte-identical to its branch version under a distinct ID, and `cases check` and `status` validate the merged tree.

### REQ-cli-029

Agent-operated native workflow. Chough's current agent SHALL execute HI, SpecSync and Fledge operations internally for authorized work, keeping each tool as the owner of its records. It SHALL use an applicable explicit prior or standing human authorization for a concrete scope instead of asking the human to repeat permission merely to run or record an internal tool operation. Before recording native scope approval it SHALL identify the actual human instruction, its source, covered scope and limits in the checkpoint and native change context, and record the acting agent account as the native actor while citing the human instruction as the authority source. A general continue instruction without applicable authority, an agent label, a bot review or a passing check SHALL NOT manufacture human approval. Fresh native checks, scope integrity and separate human implementation acceptance and delivery authority SHALL remain required. The user-facing guidance SHALL describe giving Chough the goal and authorization and reviewing the resulting PR, with native commands assigned to the agent. Unedited known predecessor skill copies SHALL update safely, and edited copies SHALL remain preserved. Trace: FLOW-1, FLOW-2, FLOW-5, FLOW-13, LIFE-1, LIFE-6, LIFE-7.

Acceptance Criteria
- The main skill, lifecycle reference and README consistently assign internal native command execution to the current agent and do not require the operator to run tool-specific approval commands for already authorized work.
- Explicit applicable prior or standing human authority is recorded with source, scope and limits before a concrete native scope approval; absent or exceeded authority asks for the actual missing decision. Agent labels and bot approvals do not become human decisions.
- Native status/check, frozen scope, evidence freshness, human implementation acceptance and separate merge/release/deployment boundaries are preserved; no additional lifecycle ledger or runtime is introduced.
- Independently hashed exact predecessor fixtures exercise real install/status/update, replacing only known unchanged copies and preserving edited bytes. Package hygiene remains enforced.
- Qualification distinguishes source/package verification from a fresh-agent cold-start run, and no skipped cold-start proof is promoted to a pass.

### REQ-cli-030

Open human gates hold forward moves. While the checkpoint's `awaiting_human` list is not empty, `chough stage advance` and `chough stage skip` SHALL be refused with `refused:awaiting-human` from every main stage and from release, before any native read; the refusal SHALL name at most 3 open gates, in checkpoint order, with the count of the rest, and SHALL say to record the human's decision on a gate with `chough checkpoint --close-gate "<gate>" --decided-by "<who>" --words "<their words>" --where "<where>" --recorded-by "<agent>"` (REQ-cli-050), and to withdraw gates no human decision closes with `chough checkpoint --clear-awaiting-human`, re-passing gates still waiting with `--awaiting-human`. No flag or reason SHALL bypass it. Iterate, back, reset, pause, resume and side-stage enter and return SHALL NOT be refused by it. Once the list is empty the same move SHALL be applied and gated as before. While a gate is open, the waiting next action in `chough next` and `chough status` SHALL say not to decide it for the human, SHALL name `chough checkpoint --close-gate` for recording their decision, and SHALL say that advance and skip are refused until it is cleared. Trace: FLOW-3, FLOW-5, FLOW-13, FLOW-2, FLOW-16.

Acceptance Criteria
- With one open gate, advance and skip are refused with `refused:awaiting-human` at intent, design, plan with SDD disabled, and release; the stage and the transitions log are unchanged and the message names the gate, the `--close-gate` command and the clearing command.
- With 5 open gates the message names the first 3 and says 2 more.
- After `chough checkpoint --clear-awaiting-human` the same advance is applied with its usual gate label; back, iterate, pause and resume, and side-stage enter and return, still work with a gate open.
- After `chough checkpoint --close-gate` closes the only open gate with a relay, the same advance is applied with its usual gate label.
- `chough next` and `chough status` give the same waiting action, which names `--close-gate` and says advance and skip are refused until the gate is cleared.

### REQ-cli-031

New-project native setup hint. When a native SpecSync configuration exists but `.specsync/sdd.json` is absent or sets `enabled` to false, `chough next` SHALL report the approved-delta line as ``not checked: native SDD is disabled; run `specsync change adopt` to gate stages``, and the `ungated:sdd-disabled` gate detail shown by `next`, `status` and stage moves SHALL name `specsync change adopt`. Without a native configuration the line SHALL remain `not checked: no native lifecycle` and the gate `ungated:not-configured`; an invalid policy SHALL keep `refused:sdd-policy-invalid` and `not checked: SDD policy invalid`. The helper SHALL NOT run `specsync change adopt` or write the policy. The walkthrough SHALL begin with a new-project setup path covering a minimal HI criteria file, `specsync init` and scaffolded module files, `specsync change adopt`, and what `ungated:sdd-disabled` means. Trace: LIFE-9, LIFE-6, LIFE-7.

Acceptance Criteria
- With SpecSync configured and `.specsync/sdd.json` absent or `enabled: false`, `chough next` (JSON and text) shows the adopt hint on the approved-delta line, and the `ungated:sdd-disabled` gate detail in `next` and `status` names `specsync change adopt`.
- Without a SpecSync configuration the line stays `not checked: no native lifecycle` with no hint, and an invalid policy still reads `not checked: SDD policy invalid` with no hint.
- The walkthrough begins with the new-project setup path.

### REQ-cli-032

Checkpoint capacity. The helper SHALL bound every checkpoint (the legacy `.chough/session.json` and each session under `.chough/sessions/`) at 1,048,576 bytes when it reads, validates, initializes, writes or merges it, including every `checkpoint --merge` input and the merged result, and SHALL bound its `observations` list at 400 entries. Every other checkpoint list SHALL keep its 200-item bound, and every other structured record, managed file, history event, case file, captured output and context section SHALL keep its existing bound. A checkpoint or observations list over its bound SHALL be refused with a message naming the bound and the recovery `chough session shelve <key>` (`chough session shelve --legacy` for `.chough/session.json`), leaving every record byte-identical. At the cap, `chough next` and `chough status` SHALL answer within 250 ms in a release build on macOS and Linux, and the focused view SHALL stay within 16,384 bytes. Human decision: Leif (0xLeif), 2026-11-05 8:08 PM MST via the orc, chose 1 MiB from the sizing study (DRIVE-14): at least 10 times the largest per-change session observed (73,231 bytes) and 4 times the largest single checkpoint (93,748 bytes), with next and status measured at 69 ms (macOS) and 19 ms (Linux) at 1 MiB in release. Human decision: Leif (0xLeif), 2026-11-02 9:40 PM MT, approved raising the limits: "Do your agent's cleanup and raise the progress file and test-result limits." The values 131,072 bytes and 400 observation pointers are agent-selected implementation choices under that capacity goal, standing through-PR authority and the later direct instruction to finish these PRs and merge. They double the original bounded capacities and are not numbers specified by the human. "Test-result limits" means the `observations` pointer bound; the 1 MiB captured-output bound is unchanged. Trace: DRIVE-14, LIFE-11, LIFE-3, FLOW-13.

Acceptance Criteria
- A checkpoint of exactly 1,048,576 bytes is read and validated, and `next`, `next --json`, `status --json` and `context` answer within 2 seconds in the debug test build with the focused view within 16,384 bytes; one of 1,048,577 bytes is refused by `context`, `next`, by `checkpoint --merge` in each input position and as a merged result, with a message naming 1048576 and `chough session shelve`, and every input stays byte-identical.
- A checkpoint with 400 observation pointers validates and one with 401 is refused; a list other than `observations` with 201 entries is still refused.
- A structured record other than the checkpoint over 65,536 bytes is still refused.

### REQ-cli-033

Post-archive summary. `chough library archive` SHALL preview, writing nothing, the observation sets of archived changes outside the newest 3 that are committed identically at HEAD. These are sets listed only by such archives, and sets that no archive lists, attributed by cycle time to the first archived change finalized at or after the observation when that change is outside the newest 3 (KNOW-20). It SHALL keep every retention protection but age, with reasons, except that among the routine runs of one case and environment only the newest run and the newest failure stay protected as newest (KNOW-17). `--apply` SHALL need retention authority, SHALL refuse while a native change is open, SHALL ledger each removal first with the archive that lists or owns the set and its knowledge entry, and SHALL restore every file and refuse if the validity of an archive that lists a removed file changes. It SHALL read the native validity of those archives in parallel, with at most 8 at once. Each archive's knowledge entry SHALL summarize the sets attributed to it. Removed observations SHALL read as pruned. Trace: KNOW-7, KNOW-8, KNOW-9, KNOW-10, KNOW-11, KNOW-17, KNOW-20, LIFE-8.

Acceptance Criteria
- The preview writes nothing and keeps hot-archive, referenced, last-failure and non-routine sets protected with reasons, and the newest run of each case and environment protected as newest.
- Apply refuses without authority or with an open change; otherwise it removes only HEAD-identical files, ledger first, and they read as pruned.
- A validity change restores every removed file byte-identically and refuses.
- Of four routine runs of one case and environment listed only by a cold archive, the newest run and the newest failure stay and the other two are trimmed.
- A record no archive lists, observed in a cold archive's cycle, is trimmed and ledgered with that archive and its knowledge entry, and that entry names it; a record in a hot archive's cycle, or observed after the newest archive, stays.
- The native validity reads of the affected archives overlap.
- With twelve affected archives, no more than 8 validity reads run at once, and the reader count is at most 8 whatever parallelism the host reports.

### REQ-cli-034

Spec files have no size bound. The helper SHALL read a native SpecSync spec file under `specs/`, such as a module's `requirements.md` companion, at any size and SHALL NOT refuse it or skip it for size: `chough evidence add` reads `requirements.md` companions whole, and `chough next` searches a module's `context.md` for its `## Lessons` heading at any size. One reader is a known exception until a later native change lifts it: `chough library snapshot` refuses any file over 4 MiB, spec files included. Native change workspaces under `.specsync/changes/` keep their 4 MiB read bound. Views that display spec text, such as `chough context`, keep their display budgets (16 KiB per section, 128 KiB in total) and mark what they cut. Every Chough record, including the checkpoint, managed JSON, history events, case files, captured output and context sections, SHALL keep its existing bound under REQ-cli-032. The workflow skill SHALL give advisory composability guidance: split independent behavior into its own spec and source module, keep coupled behavior together, and treat a source file nearing about 1,000 lines as a split candidate along a real seam; size SHALL be a hint and never a gate. Human decision: Leif (0xLeif), 2026-11-03 10:05 AM MT: "Spec files can grow to whatever number they want. ... we shouldn't really care about file size for specs." and "we need to have some, you know, good sense of composability but not be crazy about it." Trace: LIFE-11.

Acceptance Criteria
- `chough evidence add` validates a requirement ID against a requirements companion larger than 4 MiB, and still refuses an ID owned by another module with the same message, never a size refusal.
- `chough next` finds the Lessons heading of a module `context.md` larger than 4 MiB, and reports `no Lessons section` for one without it, never a size note.
- A structured record other than the checkpoint over 65,536 bytes and a checkpoint over 1,048,576 bytes are still refused.
- The bundled and installed workflow skill build references hold the same composability guidance, and main's previous build reference reads as known-outdated so `chough skill update` refreshes it.

### REQ-cli-036

Backfilled stage moves. `chough stage backfill --at <time> --commit <rev> --quote <text>` SHALL record a move to the next main stage that already happened. It SHALL apply every refusal `stage advance` applies, reading the destination's native prerequisite now, and SHALL record an `advance` transition at the given time with a backfill (REQ-history-005). It SHALL refuse with nothing recorded a time after now or before the latest transition, an unreadable latest transition, a commit that is not an ancestor of HEAD, a blank quote and release to done. It SHALL NOT record a human approval, review, consent or gate; the quote is evidence only. `timing` SHALL list backfilled moves separately, mark the segments they bound, report backfilled time per stage and keep those stages out of estimates. Trace: DRIVE-5, LIFE-7.

Acceptance Criteria
- A backfilled plan to build move records the given time, full commit SHA and quote, and `timing --json` lists it under `backfilled` and marks its segments.
- A draft native change, an open awaiting-human gate, a future or out-of-order time, an unknown or unreachable commit, a blank quote and a side stage refuse with the checkpoint and log unchanged.
- No approval, review, consent or gate record is written.

### REQ-cli-037

Active session checkpoint. When `.chough/sessions/` exists, the canonical checkpoint SHALL be the active session file `.chough/sessions/<key>.json`, named by the untracked pointer `.chough/local/active-session` or, with no pointer, the only open session or else the only open session that is not finished (REQ-history-006), and every command SHALL read and write that file instead of `.chough/session.json`. A finished session SHALL NOT count toward several open sessions, and telling which sessions are finished SHALL need no native read (DRIVE-16). Chough SHALL refuse with nothing written when it cannot tell which session is active, and `init` SHALL refuse in sessions mode. Only the active session is written, so this is not a second writable store (REQ-cli-007). Each command SHALL resolve the active session once and save only to that session, so a switch by another process partway through a command does not redirect its save; if that session is archived or removed meanwhile, the save SHALL refuse with nothing written. Without `.chough/sessions/` behaviour and bytes SHALL be unchanged. Trace: DRIVE-3, DRIVE-16.

Acceptance Criteria
- A stage move in sessions mode changes only the active session file and its history cycle.
- No session, several open sessions without a pointer, or a pointer to a missing or archived session refuses with nothing written.
- A `session use` by another process partway through a command leaves the newly active session's file unchanged, and the command's save lands in the session it resolved.
- With no pointer, an open session whose native change is archived on HEAD does not count toward several open sessions: the only other open session is active, with no native read.

### REQ-cli-038

Native verification that runs no project command. When SDD is enabled and the native status read that `chough next` or `chough status` makes reports the change's effective `verification_commands`, and every one of them is a SpecSync command (`specsync` followed by its arguments), `chough next` (text and JSON) and `chough status` SHALL say that native verification runs none of the project's commands, naming at most 3 of those commands, each cut to the next view's string bound, with a count for the rest, so a passing `specsync change check` does not show that the project's tests ran; when `.specsync/sdd.json` lists verification commands the effective list does not contain, they SHALL also say how many configured commands are not run. They SHALL decide this from that status read and that file, with no additional native read, and SHALL say nothing when the effective list is absent or includes a project command. At the test stage, when no pause, human decision, blocking bug, unlinked failure or gate stops it, `chough next` SHALL point to recording test evidence before accept, naming `chough cases run`. Neither SHALL allow or refuse any stage move that is not otherwise allowed or refused. Trace: FLOW-15, FLOW-6, LIFE-7.

Acceptance Criteria
- With an effective list of only `specsync check --spec cli`, `next`, `next --json` and `status` carry the warning naming that command.
- With `cargo test --locked` configured in `.specsync/sdd.json` but absent from the effective list, the warning still shows and says 1 configured command is not run.
- With a project command in the effective list, with no effective list in the status reply, with SDD disabled, or with no native configuration, there is no warning.
- At the test stage with nothing else stopping it, `next` names `chough cases run` and says recorded test evidence is needed before accept.
- `stage advance` from test is allowed with and without the warning, and the warning adds no native read.
- A 20,000-character effective command keeps `next` and `next --json` within 16384 bytes with the warning still shown, and 10 commands name 3 with "7 more".

### REQ-cli-039

Native check result in next. When the recorded native change has a workspace under `.specsync/changes/<id>/`, `chough next` (text and JSON) SHALL report the native check: the latest attempt SpecSync recorded in that workspace's `verification-attempts.json` by timestamp, with its time and commit, and when it failed, the failing commands with their exit codes, at most 3 with their total. When the native state `next` read is `approved` or `implementing`, `next` SHALL also say that the native check has not passed for the current tree, that SpecSync keeps no reason when it refuses before verifying, and name `specsync change check <id>`. When that state is `verifying`, it SHALL use the same read's `handoff.readiness`: readiness `safe` means the last check matches the tree; any other readiness SHALL be reported as not passed for this tree with SpecSync's reason; and no readiness SHALL be reported as unknown, so a bare `verifying` state never clears the statement. A failed attempt that names no failing command SHALL say so. It SHALL read only that local file and the native state it already read, with no additional native read, and SHALL report an unreadable, oversized or malformed attempts record as unknown, never as no failure. Trace: LIFE-7, LIFE-3.

Acceptance Criteria
- After a failed attempt, `next` text and JSON show its time, commit, failing commands and exit codes; a later passing attempt replaces it.
- With the change approved or implementing, `next` says the native check has not passed for this tree, that SpecSync keeps no reason for refusals, and names `specsync change check <id>`; when verifying, it shows no note only with handoff readiness `safe`, and any other or missing readiness keeps a note, as the last criterion states.
- A malformed attempts file is reported as unknown, and `next` still makes one native status read.
- A verifying change whose handoff readiness is not `safe` (the tree moved after the last check) is reported as not passed for this tree with SpecSync's reason; with readiness `safe` there is no note; with no readiness, freshness is reported as unknown.

### REQ-cli-040

Serialized checkpoint writers. Every command that reads the checkpoint to change it, in either mode (`.chough/session.json`, or the active session under `.chough/sessions/`), and `session new`, `session adopt`, `session use` and `inbox fold` SHALL hold one writer lock for the project from before that read until the command exits, so a concurrent writer applies its change to the latest checkpoint and no update is lost. The lock SHALL be the file `.chough/local/checkpoint.lock` inside the project, in a directory Git ignores, so every process working on the project uses the same lock whatever its temporary directory. A writer that cannot take the lock within 30 seconds SHALL refuse as `refused:checkpoint-busy`, naming the checkpoint file it would write, with nothing saved. A writer that read `.chough/session.json` and finds the project adopted into sessions mode before it saves SHALL refuse, naming the adoption, with nothing saved. Read-only commands SHALL take no lock. Trace: DRIVE-11, DRIVE-3.

Acceptance Criteria
- A writer started with a different `TMPDIR` waits while another process holds the project's lock, then saves its update.
- With the lock held, a writer in sessions mode refuses as `refused:checkpoint-busy` naming `.chough/sessions/<key>.json`, with nothing saved.
- Legacy writers racing `session adopt` each either save their decision in the one remaining checkpoint or refuse naming the adoption with nothing saved.
- `.chough/local/checkpoint.lock` is ignored by Git.

### REQ-cli-041

Trimming at done. When `chough stage advance` moves into done and its save succeeds, and the librarian is on (the default) and the `retention` preference is neither `manual` nor a recorded `off` (the unset default `off` does not stop it), the helper SHALL apply the post-archive summary of REQ-cli-033 with the recorded retention authority, or else with the librarian default as the standing preference (KNOW-13), keeping its rules: the ledger is written first, a removal that changes an affected archive's terminal evidence validity is restored and refused (KNOW-10), and nothing is removed while any native change is open. It SHALL report the result as `applied.librarian_trim` with status `trimmed`, `skipped` (librarian off, `retention manual` or a recorded `retention off`) or `refused` with its reason, and a refusal or failure SHALL NOT undo or refuse the move into done. `retention` SHALL accept `manual` beside `off` and `auto`. Trace: KNOW-13, KNOW-11, KNOW-7, KNOW-5.

Acceptance Criteria
- With defaults, `stage advance` into done reports `trimmed` and prunes the cold archived proof that `library archive` would, with a ledger entry whose authority names KNOW-13.
- With `retention` set to `manual`, a recorded `retention off`, or the librarian off, it reports `skipped` and removes nothing.
- With a native change open, it reports `refused` naming it, removes nothing, and done is still recorded.

### REQ-cli-042

Native sources are read at full size. The helper's size bounds SHALL apply only to its own records under `.chough/` and to what it displays. Every gate, validation and decision SHALL read native sources at full size: `chough evidence add`, `chough cases check` and `chough cases run` SHALL validate criteria against the complete `hi export -- <FAMILY>` output, parsed within the 4 MiB native parse bound and never from the 16 KiB display section; an archived `finalization.json` read by `chough evidence prune` and history retention, an archived `change.md` read by `chough library index`, and `.specsync/sdd.json` read by the stage gates SHALL be read within SpecSync's 4 MiB change-artifact bound and never Chough's 65,536-byte record bound. Display sections MAY still cut at their budgets (16 KiB per section, 128 KiB in total for `chough context`); each cut SHALL say that it is for display only and send the reader to the complete source, and SHALL NOT suggest that the source be shortened. The workflow skill SHALL state that Chough's size bounds never justify editing HI intent or criteria, specs or SpecSync files, and that a native source over a Chough bound is a Chough bug to report. Every Chough record, including the checkpoint, managed JSON, history events, case files and captured output, SHALL keep its existing bound under REQ-cli-032. Human decision: Leif (0xLeif), issue #100 and his instruction of 2026-11-04: "Chough should only reduce and do all that for its own files in .chough and not the specs, not the human intent." Trace: LIFE-15, LIFE-6, LIFE-11.

Acceptance Criteria
- `chough evidence add` accepts a criterion found only past the first 16 KiB of a HI family export larger than 64 KiB, still refuses a retired criterion as unknown or retired, and never names a 16384-byte bound.
- `chough evidence prune` reads native state from an archived `finalization.json` larger than 65,536 bytes; `chough library index` takes an archive's title and criteria from an archived `change.md` larger than 65,536 bytes; a disabled `.specsync/sdd.json` larger than 65,536 bytes gives `ungated:sdd-disabled`, not `refused:sdd-policy-invalid`.
- `chough context` still cuts a section at 16 KiB and its aggregate at 128 KiB, and each cut or omission marker says it is for display only, sends the reader to the complete source and says never to shorten it.
- The bundled and installed workflow skill state that Chough's size bounds cover only its `.chough/` records and displays, that HI intent or criteria, specs and SpecSync files are never edited to fit one, and that an oversized native source is a Chough bug to report; the bundle's previous SKILL.md and build reference read as known-outdated.
- A structured record other than the checkpoint over 65,536 bytes and a checkpoint over 1,048,576 bytes are still refused.

### REQ-cli-043

Knowledge directory built and kept current. With the librarian on, which is the default, `chough init` SHALL create the knowledge directory (`knowledge/`, or `library-dir`) with its `README.md`, `briefs/`, `INDEX.md` and one entry per archived native change, built and bounded in memory first and written create-only, so an existing directory is preserved byte-identical and reported as `preserved`. When the index cannot be built or bounded, `chough init` SHALL still succeed, write no knowledge file and report the library as `refused` with the reason. When `chough stage advance` moves into done and its save succeeds, after the trim of REQ-cli-041, the helper SHALL regenerate the index and every archived-change entry, creating `README.md` and `briefs/` if missing, whether or not anything was trimmed, so the change just archived and any proof just trimmed are summarized. It SHALL report the result as `applied.librarian_index` with status `refreshed` and the index path, `skipped` with the librarian off, or `refused` with its reason. A refusal SHALL NOT undo or refuse the move into done, nothing SHALL be built with the librarian off, and no gate SHALL read the directory. Trace: KNOW-1, KNOW-4, KNOW-5, KNOW-13.

Acceptance Criteria
- With defaults, `chough init` reports the library `created`, writes `knowledge/README.md`, `knowledge/INDEX.md`, `knowledge/briefs/` and one entry per archived change, and `chough next --json` names `knowledge/INDEX.md`.
- With an existing `knowledge/README.md` and `knowledge/INDEX.md`, `chough init` reports `preserved` and leaves both byte-identical.
- With an index that cannot be bounded, `chough init` succeeds, reports `refused` naming the 65536-byte bound, and leaves no knowledge directory.
- With defaults, `stage advance` into done reports `librarian_index` `refreshed` after `librarian_trim` `trimmed`, and the trimmed proof's archive entry lists it as pruned, in a project that had no knowledge directory.
- With `retention` set to `manual`, `stage advance` into done reports the trim `skipped` and still reports the index `refreshed`.
- With the librarian off, `stage advance` into done reports the index `skipped` and creates no knowledge directory, and done is still recorded.

### REQ-cli-044

Knowledge vault. Every note the helper generates in the knowledge directory SHALL start with YAML front matter giving at least `id`, `title`, `type` and `tags`, with titles and commit IDs as double-quoted strings and dates as ISO 8601 UTC. It SHALL link with standard relative Markdown links that resolve inside the repository, so the directory reads as an Obsidian vault and on GitHub alike. An archived-change note SHALL also give `status` (hot or cold), `finalized`, `modules` (the archive's affected specs), `requirements`, `criteria` (the HI criterion IDs that those requirements' `Trace:` lines name in the living requirements) and its base, implementation and verification commits, and SHALL link each module note and each criterion's intent note. `modules/<module>.md` SHALL link the module's spec, requirements, context and lesson source with the count of recorded lessons, and every archived change that touched the module, newest first. `intent/<file>.md`, for each HI file, SHALL link that file and list each criterion ID with the archived changes whose requirements trace it, without copying criterion text. `rules.md` SHALL link each project rule, skill, decision and policy file that exists: `AGENTS.md`, `CLAUDE.md`, `hi/AGENTS.md`, the installed and bundled workflow skill, `docs/DECISIONS.md`, each module's context and `.specsync/sdd.json`. The index SHALL link the module, intent and rules notes. Regeneration SHALL stay byte-identical for the same sources, every note SHALL stay within the 65,536-byte bound, no gate SHALL read the vault, and it SHALL point to sources instead of restating them. Trace: KNOW-14, KNOW-15, KNOW-3, KNOW-4, KNOW-12.

Acceptance Criteria
- Every Markdown note under `knowledge/` starts with front matter that gives `id`, `title`, `type` and `tags`.
- An archived-change note gives `type: archived-change`, its `status`, `finalized` in ISO 8601, a quoted title, its `modules`, `requirements` and traced `criteria` as lists, and its quoted commits, and links its module and intent notes.
- `modules/cli.md` links the cli spec, requirements and context, counts its lessons, and lists only the archived changes that touched cli; no lesson text is copied.
- `intent/knowledge.md` gives `type: intent`, links `hi/knowledge.md`, and on each criterion ID's line links the archived changes that trace it; no criterion text is copied.
- `rules.md` links the rule, skill, decision and policy files that exist and none that are missing, and the index links the rules, module and intent notes.
- Every relative Markdown link in the vault resolves to an existing file, and regenerating twice gives identical bytes.

### REQ-cli-045

Closed-cycle references at done. When `chough stage advance` moves into done and its save succeeds, with the librarian on, the helper SHALL, before the trim of REQ-cli-041, drop from the checkpoint's `observations` every ID whose record was observed before the current cycle began (the newest recorded reset) or that has no record, keeping every observation reference from the current cycle. Done closes the cycle, so the same step SHALL drop every entry of the checkpoint's file and design references, whether or not a reset time can be read (KNOW-21). It SHALL report the result as `applied.checkpoint_references` with status `dropped` and the dropped observation IDs, file references and design references as `dropped`, `dropped_files` and `dropped_design`, `unchanged`, `skipped` (librarian off, or nothing to drop and no reset time can be read), or `refused` with its reason. When no reset time can be read, no observation reference SHALL be dropped. A referenced observation record that exists but cannot be read SHALL keep every reference and refuse the step. A refusal SHALL NOT undo or refuse the move into done. Retention SHALL ignore the file and design references of an archived session, which is done, and SHALL NOT rewrite it. The archives, the retention ledger and the knowledge directory keep the dropped IDs, and the native archive keeps the paths the change touched. Trace: KNOW-16, KNOW-21, KNOW-13.

Acceptance Criteria
- With the current cycle begun 15 days ago, `stage advance` into done drops a reference to a record observed before it, keeps one observed after it, reports the dropped ID, and the trim may then remove the dropped record.
- With no reset recorded, `stage advance` into done drops every file and design reference, reports them as `dropped_files` and `dropped_design`, keeps the observation references, and a cold loose folder a dropped file reference held is then trimmed.
- With the librarian off, the references are unchanged and reported `skipped`.
- A cold loose folder named only by an archived session's file and design references is trimmed, and the archived session file is unchanged.

### REQ-cli-046

Loose artifacts and vault attachments. The post-archive summary of REQ-cli-033 SHALL also cover loose artifacts: each top-level entry under `.chough/artifacts/` whose files no observation record references. Each of its files SHALL belong to the archives whose manifests list it, or else to the archive whose cycle that file's first Git commit falls in, by cycle time (KNOW-20), and the entry belongs to all of them. It SHALL stay while any of its files is named, itself or through a folder that contains it, by the checkpoint's references or by the fixture of a bug that is not closed (verified, duplicate or wont-fix), belongs to a hot archive, belongs to no archive because it was first committed after the newest archive or has no commit, or is not committed identically at HEAD. A folder named by the checkpoint or by a bug that is not closed SHALL protect the observation sets whose files it contains the same way. An eligible entry SHALL be ledgered first as `kind: loose-artifact` with ID `artifact:<entry>`, with its files, and SHALL then be removed (KNOW-18). Key items SHALL move into the vault instead of being removed:
- the record and artifacts of a non-routine observation (manual, agent-assisted or a human observation) whose only remaining protection is that it is non-routine;
- the artifacts of a failed run;
- screenshots (png, jpg, jpeg, gif, webp, svg) and failure-first logs among artifacts.

Each key item SHALL move to `knowledge/attachments/<folder>/<path under .chough/>` before its original is removed. The folder SHALL be the archive ID when the ID has at most 24 characters; otherwise it SHALL be the ID's first 16 characters, cut back to before the last hyphen among them when there is one, then `-` and the first 8 hexadecimal digits of the SHA-256 of the ID, so attachment paths stay short enough to check out on Windows. A key item whose attachment path would be longer than 220 characters SHALL stay in `.chough/`, and its set SHALL be reported protected with that reason. A moved file and a file restored after a refused apply SHALL keep their permissions. A move SHALL never overwrite: an attachment that already holds the same bytes is kept and never deleted by a restore; when it holds different bytes, the entry or set stays, reported protected with that reason, and one that appears during the apply refuses it with both files kept. The ledger entry SHALL map each moved file to its attachment, a restore SHALL delete the copies it made, and the archive's knowledge note SHALL list its loose artifacts and link its attachments (KNOW-19). Trace: KNOW-18, KNOW-19, KNOW-20, KNOW-9, KNOW-10.

Acceptance Criteria
- A loose artifact entry listed only by a cold archive, and one first committed in a cold archive's cycle, are trimmed and ledgered as loose artifacts with that archive, and the archive's note lists their files. One first committed after the newest archive, and one named by an open bug's fixture, stay.
- The loose fixture of a verified bug is trimmed.
- A manual observation with a screenshot, and the log of an older failed run, owned by a cold archive, move to the archive's attachments folder. They are gone from `.chough/`, mapped in the ledger and linked from the archive's note. A routine record is removed, not moved.
- An archive ID longer than 24 characters gets an attachments folder of at most 25 characters, the same on every run, and no attachment path is longer than 220 characters: a key item that would exceed it stays in `.chough/`, and its set is reported protected with that reason.
- An executable key item keeps its executable bit in the vault, and an executable file that an apply removed and then restored after a refusal comes back executable.
- An open bug whose fixture is a folder, and a checkpoint reference to a folder, keep the loose files and the observation sets inside it.
- A folder with an old file a cold archive lists and a file first committed after the newest archive stays whole.
- An attachment that already holds other bytes keeps both files and its set reported protected; one written during the apply refuses it and both files are kept; an identical one already in place is kept after a refused apply.

### REQ-cli-047

No conflicting files. When two branches each record Chough work, merging one into the other SHALL need no hand-resolved conflict in a file Chough writes (DRIVE-12). Case definitions MAY be split per change: `.chough/cases/<module>.<part>.cases.toml` SHALL hold cases of `<module>`, which its `module` field SHALL name, `<part>` SHALL use letters, digits, `-` and `_`, and case IDs SHALL stay unique across all files. `chough library index --check` SHALL write nothing and SHALL fail, naming each generated knowledge file that is missing or differs from a rebuild, with line endings ignored; the verify and CI lanes SHALL run it, so generated knowledge can merge with Git's union driver and be rebuilt. `chough bug merge <base> <ours> <theirs>`, a Git merge driver for `.chough/bugs/*.json`, SHALL merge two edits of one bug: a field only one side changed SHALL take that change, list fields SHALL keep base's entries and then each side's additions without duplicates, the update time SHALL be the later one, the result SHALL be a valid bug record written as bug commands write it, and a field both sides changed differently SHALL be left for a human with `ours` unchanged and a nonzero exit. Trace: DRIVE-12, DRIVE-3.

Acceptance Criteria
- Two branches that each add a case in their own `cli.<part>.cases.toml` merge without conflict, and both cases load; a case ID defined in two files is refused.
- `library index --check` passes on a rebuilt vault, fails naming a generated file that was edited, passes again after `library index`, and passes on a CRLF copy of a current file.
- Two edits of one bug, one adding a note and verifying it and the other adding a note and an observation, merge with both notes, the observation, the verified status and the later update time; two different status changes exit nonzero and leave ours unchanged.

### REQ-cli-048

Closed bugs release proof. In the post-archive summary and trim of REQ-cli-033, REQ-cli-041 and REQ-cli-046, a bug that is closed (verified, duplicate or wont-fix) SHALL NOT protect the observations it links: its observations, note observations and regression baseline. Its key items SHALL move into `knowledge/attachments/<folder>/`, where the folder is formed from the bug ID as REQ-cli-046 forms it from an archive ID, instead of being removed (KNOW-22). The key items are:
- the earliest linked failed observation, by observed time and then ID, with its artifacts;
- for a verified bug, the first linked passed observation observed after that failure, with its artifacts;
- the bug's fixture when it is a file or folder under `.chough/`.

A key item SHALL move only when it would otherwise be trimmed: its set or loose group is owned by a cold archive and has no other protection. Every other observation the bug links SHALL be summarized, ledgered and trimmed by the same cold rule. Moves SHALL follow REQ-cli-046: they never overwrite, keep permissions, are ledgered with their attachment paths, and are undone by a restore. A linked record that is missing or unreadable is not a key item, and an item two closed bugs name SHALL go with the bug whose ID sorts first. The bug record SHALL keep its status and IDs. An open bug SHALL keep protecting everything it links. `chough evidence prune`, which moves nothing into the vault, SHALL still treat every bug's links as protection.

`chough library index` SHALL write `bugs/<id>.md` for each closed bug whose attachments folder holds files. The note SHALL have front matter giving `id`, `title`, `type: bug`, `status` and `tags`, and SHALL link the bug record and each of its attachments. Trace: KNOW-22, KNOW-18, KNOW-19, KNOW-13.

Acceptance Criteria
- For a verified, a duplicate and a wont-fix bug linking a cold failed run with a log, a later passed run and another passed run, the trim at done:
  - moves the failed run and its log into `knowledge/attachments/<bug>/`;
  - moves the later passed run there only for the verified bug;
  - trims and ledgers the other run;
  - writes `knowledge/bugs/<bug>.md` linking what moved;
  - leaves the bug record unchanged.
- An open bug keeps every observation it links.
- A closed bug's fixture under `.chough/` moves into the vault under the bug.
- `evidence prune` still names a verified bug's link as a protection.

### REQ-cli-049

Retention ledger rotation. When `chough stage advance` moves into done, with the librarian on, after the trim of REQ-cli-041 and the knowledge refresh of REQ-cli-043, the helper SHALL rotate `.chough/pruned.jsonl`. It SHALL keep the newest 100 entries there, newest by `pruned_at_unix_seconds` and then by later line, and SHALL keep every entry that has no archive or names an archive without a knowledge note under `<library-dir>/archives/`. Each other entry SHALL move into `<library-dir>/attachments/<folder of its first archive>/pruned.jsonl`, the folder formed as in REQ-cli-046, and a `restored` event SHALL move with the entry it follows for the same ID (KNOW-23). A rotation SHALL append the moved lines to their shards and only then rewrite `.chough/pruned.jsonl` without them, so an interruption leaves at most lines held in both a shard and the ledger. It SHALL refuse, moving nothing, when a shard would exceed 16 MiB. It SHALL report `applied.ledger_rotation` with status `rotated` and the counts moved and kept, `unchanged`, `skipped` (librarian off), or `refused` with its reason, and SHALL NOT undo or refuse the move into done. In sessions mode the rotation SHALL run before the active session archives itself, which is the last step of the move into done (REQ-history-006, #155). Every reader of the retention ledger SHALL read each shard in path order and then `.chough/pruned.jsonl`, each file within 16 MiB. Each shard copy of a line SHALL cancel one identical copy in `.chough/pruned.jsonl`, the leftover of an interrupted rotation, which the next rotation drops; identical lines within one file SHALL all be kept, since a prune, a restore and another prune in one second write two identical entries. These readers are observation labels, bug views, `evidence prune`, the post-archive summary and the knowledge notes. Shard files SHALL NOT be listed as attachments. The repository SHALL declare `knowledge/attachments/*/pruned.jsonl merge=union`. Trace: KNOW-23, KNOW-13, DRIVE-12.

Acceptance Criteria
- With 105 old entries of cold `a1`, a restore of the oldest, and an old entry of an archive without a note, the move into done:
  - keeps 100 entries and the entry without a note in `.chough/pruned.jsonl`;
  - moves the rest, and the restore, into `knowledge/attachments/a1/pruned.jsonl`;
  - loses and duplicates no line;
  - reports `rotated` with the moved count.
- After rotation:
  - a moved ID linked by an open bug is still labeled pruned;
  - its archive note still lists it;
  - `library index --check` passes.
- A leftover copy in `.chough/pruned.jsonl` of a shard line, after its restore, does not make the restored ID pruned again.
- `git check-attr` reports `union` for an attachments folder's `pruned.jsonl`.
- In sessions mode, the move into done reports `applied.ledger_rotation` and then `applied.session_archive`, and the session is archived only after the rotation ran.

### REQ-cli-050

Relayed decision at a human gate. `chough checkpoint --close-gate <gate>` SHALL close the one open awaiting-human gate whose text is exactly `<gate>`, and SHALL require `--decided-by`, `--words`, `--where` and `--recorded-by`: who is said to have decided, their words verbatim, where and when they said it including any relay, and the agent recording it. It SHALL keep every other open gate open, append one `gate-closed` transition for that gate carrying `relayed_by` with those four values (REQ-history-010), and add one checkpoint decision `<decided_by> decided "<gate>": "<words>" (<where>); recorded by <recorded_by>; a relayed claim, not authenticated`. Its output SHALL carry the gate, the four values and the sentence `a relayed claim, not authenticated: it records who is said to have decided, not proof that they did`. Each refusal SHALL save nothing and write no transition: `refused:relay-incomplete` when any of the four is missing or blank, naming the missing flags and the complete command; `refused:relay-without-gate` when any of the four is given without `--close-gate`; `refused:gate-not-open` when no open gate has that exact text, naming `chough next`, which lists the open gates; and `refused:relay-too-long` when the decision would exceed 4096 bytes, saying to shorten `--words` or `--where`. `--close-gate` SHALL NOT be combined with `--clear-awaiting-human`, `--awaiting-human` or `--merge`. A relayed decision SHALL NOT write or change any SpecSync record and SHALL NOT satisfy or bypass any stage gate: reaching build, test, accept and done keeps reading native state. `--clear-awaiting-human` SHALL keep withdrawing every open gate with no relay and no decision. Chough SHALL NOT authenticate any of the four values. Trace: FLOW-16, FLOW-10, FLOW-13, FLOW-3.

Acceptance Criteria
- With gates A and B open, `--close-gate A` with the four values closes only A:
  - B stays open, and advance is still refused with `refused:awaiting-human` naming B;
  - one `gate-closed` event for A carries `relayed_by` with the four values byte for byte;
  - the decision appears in `chough next` from a fresh process, and the output carries the boundary sentence.
- A missing or blank value, a value without `--close-gate`, a gate that is not open and an over-long decision each refuse with the checkpoint and history unchanged, and the command each refusal names then succeeds.
- With a native change in draft, closing its scope gate with a relay leaves every SpecSync file byte-identical, and advance to build is still refused by the native gate.
- `--clear-awaiting-human` still closes every open gate and writes no `relayed_by` and no decision.

### REQ-cli-051

Session notes. `chough library index` SHALL write `<library-dir>/sessions/<key>.md` for each session rolled into the vault by REQ-history-011, found at `<library-dir>/attachments/<folder>/sessions/<key>.json`. Each note SHALL have front matter giving `id` (`session/<key>`), `title` (the session's objective), `type: session`, `native_change`, `stage` and `tags`. It SHALL link the session's attachment and, when it exists, the native change's archive note. `library index --check` SHALL cover these notes, and the repository SHALL declare `knowledge/sessions/*.md merge=union`. Trace: KNOW-29, KNOW-14, DRIVE-12.

Acceptance Criteria
- After a roll-up, `knowledge/sessions/<key>.md` names `session/<key>` and the session's objective and links `../attachments/<key>/sessions/<key>.json`, and `library index --check` passes.
- `git check-attr` reports `union` for a session note.

### REQ-cli-052

Several forward moves on one native read, and one read into done. `chough stage advance --to <stage>` SHALL move forward through each main stage, from the current one up to `<stage>`, as consecutive advances. `<stage>` SHALL be a main stage after the current one, and any other value SHALL be refused. Every refusal of `stage advance` SHALL apply, including a stage that is not active, a side stage and an open human gate (REQ-cli-030). Before recording anything, the helper SHALL check every step as `stage advance` alone would check it: its destination's native prerequisite (REQ-cli-008) and, on leaving test, the blocking-bug and unlinked-failure refusals. All steps SHALL share one `specsync change status` read, made only when a step has a native prerequisite. A refused step SHALL refuse the whole command, naming that step with its label and detail. Then no transition SHALL be recorded and the checkpoint SHALL stay byte-identical. Otherwise the helper SHALL record one `advance` transition per step, in order, each with its own gate label and the given note. It SHALL report the steps in order in `applied.steps`, with `applied.from` as the starting stage, `applied.to` as the target and `applied.gate` as the last step's label. `stage advance` without `--to` SHALL be unchanged.

On a move into done by `stage advance`, with or without `--to`, the trim of REQ-cli-041 SHALL take the change's acceptance manifest from the gate's `specsync change status` response for that change instead of reading it again. So the move into done SHALL make one native status read when no other native change is open or archived later and the trim removes nothing. Every other read SHALL stay live and keep its refusals: another change's manifest, and every validity probe around a removal. The reused response SHALL serve only that command and SHALL NOT persist. Trace: DRIVE-9, DRIVE-2, FLOW-2, LIFE-3.

Acceptance Criteria
- Under a per-change session (`chough session new`), with a counting SpecSync stub reporting an approved change at plan, `stage advance --to accept` exits 0 and runs `specsync change status` once. It records three advances (plan to build, build to test, test to accept) as event files in that session's cycle, reports them with their gate labels in `applied.steps`, and leaves the stage at accept. In a single-checkpoint project the same command appends the three to `.chough/transitions.jsonl`.
- From intent, `stage advance --to plan` runs no native read and records two advances.
- Each of these refusals makes at most one native read, records no transition, and leaves the session file and the history directory byte-identical:
  - at build with an open acceptance-blocking bug, `--to accept` is refused naming `refused:blocking-bugs` and the test to accept step;
  - at plan with the stub reporting draft, `--to test` is refused naming `refused:native-draft`;
  - a target at or before the current stage, a side, optional or unknown stage name, and an open human gate are refused.
- With the stub reporting an archived change with authenticated terminal evidence and an acceptance manifest, and the librarian on by default:
  - `stage advance --to done` from test runs `specsync change status` exactly once and reports `librarian_trim`;
  - `stage advance` from accept to done also runs it exactly once, where before this change it ran twice.
- `stage advance` without `--to` records one transition, as before.
- Every test of new behavior above fails against main before the implementation, and passes after it. The unchanged plain advance passes both times. `fledge lanes run verify` passes.

### REQ-cli-054

Bug value, deduplication and report mappings. `chough bug add` and `chough bug update` SHALL accept exactly the six classifications (unclassified, product-defect, regression, environment, requirement-gap, evidence-gap), the three scopes (blocking, in-scope, out-of-scope) and the seven statuses (open, investigating, fix-pending-verification, verified, deferred, duplicate, wont-fix), and SHALL refuse any other value naming the allowed ones, with nothing written. A bug SHALL block leaving test only while its scope is blocking and its status is not verified, duplicate or wont-fix. Search SHALL score the word overlap of a report with each bug's title, symptom and trigger, adding 0.25 when they share a criterion, requirement or case; it SHALL list matches from 0.2 and refuse a new bug only from 0.5, naming `chough bug update <id> --observation <id> --note <text>` and `--distinct <why this is a different defect>`. A distinct reason SHALL NOT be blank and SHALL be kept in the new bug's notes. A duplicate or related link SHALL name another existing bug: a link to the bug itself or to an unknown bug, and an update or report of an unknown bug, SHALL be refused with nothing written, naming `chough bug list --all`. Changing the status of a duplicate bug SHALL clear its `duplicate_of` link and record that in its notes, so a mistaken duplicate can be reopened. Marking a bug verified without a linked failure and a later linked pass SHALL name `chough bug update <id> --observation <id>`. The report candidate SHALL show every stored field, and `none recorded` (for steps, `Not recorded.`) for each empty one. Trace: FLOW-8, LIFE-5.

Acceptance Criteria
- Each classification, scope and status is accepted on add or update and recorded; any other value is refused with the allowed list and the record stays byte-identical.
- Of every scope and status, only an open bug with scope blocking is listed as blocking and stops leaving test; making it wont-fix releases the gate.
- A report sharing only a requirement with a bug is listed by search at 0.25 and added without `--distinct`; one sharing its words is refused, naming both ways forward; a blank distinct reason is refused with nothing written, and a given one is kept in the new bug's notes.
- Linking a bug to itself or to an unknown bug, as duplicate or related, and updating or reporting an unknown bug, are refused with nothing written and name `chough bug list --all`, which lists the bugs.
- A bug marked duplicate by mistake is reopened with `chough bug update <id> --status open`, which clears `duplicate_of` and notes it; marking it verified without a linked failure names `chough bug update <id> --observation <id>`, and linking a failure and a later pass then verifies it.
- The report shows every stored field of a fully recorded bug, and `none recorded` (for steps, `Not recorded.`) for each empty one.

### REQ-cli-055

Chough doctor. `chough doctor` SHALL report the Chough debt of REQ-history-012 as JSON with `schema_version` 1, `mode` (`report`, `dry-run` or `fix`) and `findings`, each with `kind`, `target`, `fix` (the Chough command that fixes it), `decision` (whether the fix is a decision) and `status` (`found`, `fixed` or `skipped`, with a `reason` when skipped). With no flag, or with `--dry-run`, it SHALL change nothing and report each finding as `found`. With `--fix` it SHALL apply each fix that is not a decision. It SHALL apply a decision fix only when `--reason <text>` is given, and SHALL otherwise report it as `skipped` with a fix that names `--reason`. It SHALL exit 0 when no finding is left `found` or `skipped`, and 1 otherwise. It SHALL make no native read, and SHALL never record a human decision, approval, review or consent. Trace: DRIVE-17.

Acceptance Criteria
- With no flag and with `--dry-run`, the doctor reports the same findings, each with a `chough` fix, exits 1, and leaves every file under `.chough/` unchanged.
- `--fix` without `--reason` applies the fixes that are not decisions and reports each decision fix as `skipped`, naming `--reason`; a project with no debt reports no findings and exits 0.
- No doctor run calls SpecSync.

### REQ-cli-056

Projects in a subdirectory. When the project root is inside a Git repository but is not its top level, the evidence snapshot SHALL cover the project's own files, under the project root, and SHALL take the repository's HEAD as its revision. The snapshot is the one `cases run`, `cases record`, `evidence add`, `validate run`, `status` and `next` record or compare. A working-tree change outside the project SHALL NOT make the snapshot dirty or change its fingerprints. The same project SHALL have the same content fingerprint at the top level and in a subdirectory, and at the top level the snapshot SHALL be unchanged. A command run there SHALL NOT refuse with advice to use the repository root. Trace: DRIVE-18.

Acceptance Criteria
- In a project in `app/` of its repository, `cases run` and `evidence add` succeed and record the repository's HEAD, and `status` reports a known snapshot.
- An edit outside the project leaves the snapshot clean with unchanged fingerprints; an edit inside it makes the snapshot dirty and changes its content fingerprint.
- The same project has the same content fingerprint at the repository root and in a subdirectory.

### REQ-cli-057

SpecSync change steps through Chough. `chough change new <description> [--kind <kind>] [--spec <spec>]... [--path <path>]...` SHALL run `specsync change new <description>` with the same options and `--json`. `chough change answer <id> <question> <answer>` SHALL run `specsync change answer <id> <question> <answer> --json`. `chough change check <id>` SHALL run `specsync change check <id> --commit`. `chough change status [<id>]` SHALL run `specsync change status [<id>] --json`. Each SHALL run in the project root given by `--root`, with the `specsync` that Chough's native reads use. Each SHALL pass SpecSync's standard output, standard error and exit code through unchanged, and SHALL write nothing under `.chough/`. `status` is a read and SHALL keep the native read bound (120 s unless `CHOUGH_NATIVE_TIMEOUT_SECONDS` sets it); past the bound it SHALL refuse with nothing recorded and name the retry with a longer bound. `new`, `answer` and `check` write, and SHALL never be stopped by that bound. Stopping Chough with SIGINT, SIGTERM or SIGHUP SHALL stop the SpecSync process it started (FLOW-14). When `specsync` cannot be started, each SHALL refuse with exit 1 and nothing run, naming the install command from the README's toolchain section and the same step run again.

Acceptance Criteria
- Each of `chough change new`, `answer`, `check` and `status` runs SpecSync with exactly the arguments above, in the `--root` directory, and its standard output, standard error and exit code match SpecSync's byte for byte, including a SpecSync refusal's.
- No `chough change` command changes any file under `.chough/`.
- `chough change check` completes after the native read bound has passed, while `chough change status` past the bound refuses with nothing recorded, and the retry it names succeeds.
- When `specsync` cannot be started, each command refuses with exit 1, naming the install command and the step to run again; with SpecSync installed, that step succeeds.
- A SIGTERM to `chough change check` stops the SpecSync process it started.

### REQ-cli-059

Ship through Chough. `chough change ship <id> [--close-gate <gate> --decided-by <who> --words <words> --where <where> --recorded-by <agent>]` SHALL, before running anything and with nothing written, refuse unless the active session's native change is `<id>` and its stage is accept, and unless every open human gate is the one `--close-gate` names. The relay values SHALL be checked as `chough checkpoint --close-gate` checks them (REQ-cli-050). Each refusal SHALL name the command that recovers. It SHALL then run `specsync change ship <id>` in the project root with no time bound, and pass SpecSync's standard output and standard error through unchanged. When SpecSync exits non-zero, it SHALL exit with that code and write nothing. When SpecSync succeeds, it SHALL close the named gate as `chough checkpoint --close-gate` would, a relayed claim, and then move the session to done as `chough stage advance --to done` would, printing each step's report after SpecSync's output. When either step refuses after a successful ship, it SHALL exit 1, say that the change was shipped, and name each remaining command. Stopping Chough with SIGINT, SIGTERM or SIGHUP SHALL stop the SpecSync process it started (FLOW-14).

Acceptance Criteria
- With the review gate's relay, `chough change ship` runs `specsync change ship <id>` once, closes the gate with the four values as a relayed claim, and moves the session to done.
- A SpecSync refusal's standard output, standard error and exit code pass through, and nothing under `.chough/` changes.
- An open human gate without its relay, a wrong or incomplete relay, another change or a stage other than accept are each refused before SpecSync runs, with nothing written, and the command each names succeeds.
- When the move into done is refused after a successful ship, the exit is 1, the output says the change was shipped and names `chough stage advance --to done`, and that command then succeeds.
- `chough change ship` completes after the native read bound has passed.
- The walkthrough and the skill's closeout run `chough change new`, `chough change check` and `chough change ship`; review stays `specsync change review`.

