# CLI testing

Product checks run through `fledge run test`; structural checks through `fledge run
spec`; HI structure through `fledge run intent`; native lifecycle coherence through
`fledge run audit`; case definitions through `fledge run cases`; the end-to-end
sample through `fledge run exercise`. Structural success cannot prove the behaviors
below. Stable case definitions are in `.chough/cases/cli.cases.toml`. TC-cli-0001 to
TC-cli-0037 bind the original integration tests one to one, with later automated
cases extending that coverage. TC-cli-0101 to TC-cli-0104 cover the exercise, cold
start, wording review and timing. TC-cli-0105 retains the BUG-0024 archived-resume
scenarios and covers the BUG-0025 integrated new-cycle regression. TC-cli-0106 is
the BUG-0021 retrospective Kimi guidance review. TC-cli-0126 to TC-cli-0133 cover the
fresh-agent handoff: the archived-not-done action, the decisions, last failure and
lesson pointer blocks of `next`, their cap and byte-identical output, the fb6e487
skill digests, the skill wording review and the graded cold-start protocol in
`docs/exercises/handoff-cold-start/`.

## Planned slug identity verification

The approved change `use-slug-identities-for-all-new-chough-records` adds four
curated cases: `TC-cli-slug-observation-allocation`,
`TC-cli-slug-validation-allocation`, `TC-cli-slug-snapshot-allocation` and
`TC-cli-slug-case-compatibility`. Their direct regressions run first against the
unfixed tip in independent roots and for identical repeated runs, then unchanged
after implementation. Failure-first records and strictly later passes must remain
separate observations linked to the same deduplicated local bugs. The old case
validator's rejection of a slug definition is not a behavioral `not-run` result;
the direct failed test supplies that pre-fix evidence instead.

The planned checks cover lowercase curated case syntax and module-aware bounds,
global duplicates and unchanged legacy digests; random OBS, VAL and SNAP forms and
independent-root and repeated-run distinctness; optional explicit new-form
observations and old-style new-write refusal with omit-ID guidance; exact inherited
artifact paths; random-source, same-ID different-content, create-new race and write
failure preservation; mixed legacy and new reads, links, status, native manifests,
retention, pruning and timing; and legacy and new validation resume and librarian
indexes. Random-looking output is not a deterministic randomness proof. Snapshot
manifest-last publication is not claimed as a crash-proof multi-file transaction.
Bug known-answer vectors and transition, preference, consent, stop-marker and
pruning-ledger entry shapes remain unchanged.

After materialization and implementation, run the four cases without explicit
observation IDs and record only actual returned IDs, outcomes, environments, linked
bugs and limitations. No slug identity observation or result is recorded by this
planning update. Hosted results, human implementation review, finalization and merge
remain unclaimed until separately observed.

## Observed checkpoint merge verification

The approved change `make-checkpoint-merges-safe-across-parallel-changes` covers
ordered exact-string collections, atomic current-cycle selection, fail-closed
validation and refusal preservation in `tests/stages.rs`. Real Git fixtures use
`.chough/session.json merge=chough-session` and the documented per-clone driver for
compatible merges and unresolved current-cycle conflicts, with the explicit command
as the no-driver fallback. `tests/cli.rs` proves init leaves Git configuration
unchanged. `tests/package.rs` and `tests/optional.rs` prove known-outdated reference
updates and edited-copy preservation. No new case number was allocated.

- Failed observation `checkpoint-merge-real-git-pre-fix` was recorded before
  implementation on `local-macos-arm64`. Its artifact retains both the ordinary Git
  conflict and the unsupported driver result.
- The configured real Git regression passed after implementation and review fixes.
  Passed observation `checkpoint-merge-real-git-post-fix` was recorded strictly
  later and linked to the same local bug, which is now verified.
- Focused checks passed 33 stage tests, 26 CLI tests, 11 package tests and 7
  optional tests, plus formatting and warnings-denied Clippy. The scoped native
  check materialized REQ-cli-007. The complete Fledge verify lane passed all 9
  steps in 221.890 seconds, the full Rust suite also passed with Rust 1.88.0,
  and pre-push passed all 5 steps in 5.422 seconds. These are local results,
  not hosted CI or human review.

| Requirement | Verification responsibility | Cases |
|-------------|-----------------------------|-------|
| REQ-cli-001 | Init repeated around edited files and partial installed bundles | TC-cli-0001 |
| REQ-cli-002 | Focused ordered output and session resumption in isolated repo; the lesson source pointer (`<module>.3md`, else the `context.md` Lessons line) with archived bundles and no lesson text; the skill's certainty labels and decision-outranks-lesson rule; the resume step that states the handoff before the first edit | TC-cli-0002, 0129, 0132, TC-cli-skill-asks-for-the-handoff-before-the-first-edit |
| REQ-cli-003 | Native status passthrough and unavailable tool states; archived native guidance stays visible without replacing the new-cycle action | TC-cli-0003, 0004, 0105 |
| REQ-cli-004 | Current → changed content → stale; environment mismatch/unchecked; content identity kept across Chough-only, empty and already-present commits and SpecSync bookkeeping and archive moves, stale on any scope or source edit; legacy records stay revision-bound | TC-cli-0005 to 0008, 0067 to 0073 |
| REQ-cli-005 | Unknown criterion/requirement and duplicate observation rejected | TC-cli-0009 |
| REQ-cli-006 | Traversal/symlink escape, unsupported schema and bounded output | TC-cli-0010 to 0014 |
| REQ-cli-007 | Schema 2 checkpoint, cross-process round trip, explicit migration; checkpoint resumption, paused and awaiting-human precedence, native-failure closed-gate precedence and unchanged session guidance; TC-cli-0106 retrospectively reviews the original Kimi cold-start record and guidance without a new host session | TC-cli-0015 to 0017, 0102, 0105, 0106 |
| REQ-cli-008 | Every transition kind, history, native gates, stage contract parity | TC-cli-0018 to 0020, 0032, 0101 |
| REQ-cli-009 | Definition validation, location in errors, position-free digests; curated slug syntax and bounds, global duplicates and unchanged legacy IDs and digests | TC-cli-0022 to 0024, TC-cli-slug-case-compatibility |
| REQ-cli-010 | Focused selection with tracks and omissions | TC-cli-0025, 0101 |
| REQ-cli-011 | Failing, not-run, missing-output, timeout and build-failure outcomes; verdicts; stop and resume; automatic OBS and VAL allocation, optional explicit new-form IDs, old-style write refusal and inherited artifacts | TC-cli-0026, 0027, 0034, 0101, 0103, TC-cli-slug-observation-allocation, TC-cli-slug-validation-allocation |
| REQ-cli-012 | Reproduce, deduplicate, regression baseline, blocking scope; verified only by a pass observed after the latest failure; TC-cli-0125 retains `tc-cli-0125-collision-pre-fix`, the failed `local-macos-arm64` observation of independent-root manual and automatic ID collisions, and the strictly later `tc-cli-0125-collision-post-fix` pass for distinct new-form IDs, schema 1 legacy compatibility and cross-form duplicate and related links. Bug generation and known answers stay unchanged | TC-cli-0029 to 0032, 0076 to 0079, 0101, 0125 (failed then passed on `local-macos-arm64`) |
| REQ-cli-013 | Preferences separate from consent; authorization, stop, pause and resume for legacy and new validation runs | TC-cli-0021, 0033, 0034, 0103, TC-cli-slug-validation-allocation |
| REQ-cli-014 | Release only when authorized; dry-run kind | TC-cli-0021, 0028, 0103 |
| REQ-cli-015 | Snapshots keep sources; index lists open bugs and mixed legacy/new snapshots; symlinks refused; random SNAP allocation and timestamp chronology; closeout works with the librarian off | TC-cli-0035, 0036, 0132, TC-cli-slug-snapshot-allocation |
| REQ-cli-016 | Per-host install and safe updates preserve edits; Kimi mapping reviewed without a new host session; 2756dc2, fb6e487 and identity-main predecessors are known-outdated | TC-cli-0037, 0075, 0106, 0131 |
| REQ-cli-017 | Schema 2 observations; schema 1 still read; optional content identity validated; mixed legacy/new IDs and exact artifact inheritance | TC-cli-0026, 0028, 0073, 0074, TC-cli-slug-observation-allocation |
| REQ-cli-021 | Focused next/status parity, bounded forms and one native read; archived-done and archived-not-done actions with precedence; reason-bearing reset/back; bounded decisions/failure/lesson blocks ordered by recorded fields; graded cold-start handoff | TC-cli-0105, 0120 to 0122, 0126 to 0130, 0133 |
| REQ-cli-023 | Independent-root/repeated-run allocation; random-source/collision refusal; atomic preservation; legacy reads/resume; native manifests, retention, timing and unchanged non-ID shapes | All four slug cases plus cases, optional, evidence, bugs, CLI, timing and package suites |
| REQ-cli-042 | HI family exports over 16 KiB validate criteria and still refuse retired ones; archived `finalization.json` and `change.md` and `.specsync/sdd.json` over 64 KiB are read; display cuts point to the complete source; the skill says Chough bounds never justify editing HI, specs or SpecSync files | TC-cli-hi-export-over-16-kib-validates, TC-cli-archived-finalization-over-record-bound, TC-cli-archived-change-md-over-record-bound, TC-cli-sdd-policy-over-record-bound, TC-cli-display-cut-points-to-source, TC-cli-skill-bounds-never-justify-native-edits |
| REQ-cli-053 | Warm archived report reuse with live lifecycle reads; complete cache invalidation, anti-spoofing and negative inputs; cold-timeout cleanup and retry; exact concurrent snapshot identities | TC-cli-archived-report-reuse, TC-cli-archive-cache-invalidation, TC-cli-archive-cache-timeout, TC-cli-snapshot-worker-identity, TC-cli-specsync-symlink-reuse |

Unix-only tests are gated with `cfg(unix)` because they use shell stubs for native
tools; their cases list only macOS and Linux environments. Chough supports macOS and
Linux, and Windows through WSL 2 with Ubuntu (FLOW-17), so hosted CI runs no Windows
job; the Ubuntu jobs cover WSL 2 as Linux. The content-identity freshness tests
(TC-cli-0067 to 0074, with a CRLF `change.md` in TC-cli-0070) and the bug-ordering
tests (TC-cli-0076 to 0079) still write their CRLF front matter and `/`-separated keys
explicitly and run on Linux and macOS. Their `cfg(windows)` compiled stand-ins remain
but are not built by CI. Earlier case definitions that list `ci-windows` keep it as a
label only. `TC-cli-supported-platforms` keeps the CI matrix, the Linux exercise and
the platform statements in agreement.

No physical deployment, live monitoring, authenticated human acceptance, APS,
Magpie or other unbuilt integration is covered by this test contract.

## Observed Kimi cold start, 2026-11-01

Source: [concise Kimi log](../../.chough/artifacts/kimi-cold-start.log), original base
`5af3f78`, Kimi Code CLI 0.41.0 on macOS arm64 in `local-macos-arm64`. The supplied
task named the repository, objective, `AGENTS.md` and canonical skill. Kimi also
independently listed and loaded `.agents/skills/chough-workflow/SKILL.md` as a
Project skill. Discovery passed in this one run, not as a general host-matrix result.

- `cargo build --locked` passed in 10 seconds. `chough next` emitted 11,567 JSON
  lines in 16 seconds with stage `done` and native state `archived`, but stale review
  and ship guidance. Native status confirmed archived state and its own next action.
  `chough status --json` emitted about 11,567 lines in 36 seconds with the same
  contradiction.
- Skill status was current, case definitions were valid, and the local baseline
  verify lane passed 9 steps in 114 seconds. The baseline used HI 0.5.0 and Fledge
  1.8.0 with SpecSync 6.1.1, so it is not final CI-toolchain or hosted CI evidence.
  Release-verified HI 0.8.0 and Fledge 1.7.2 were later placed under
  `target/tmp/ci-tools`; the final checks for that historical change used those
  versions with SpecSync 6.1.1.
- Safe cold resume exposed BUG-0024, and absent mapping guidance exposed BUG-0021.
  BUG-0022 records the initial tool drift and BUG-0023 the output size; both remain
  open and out of scope.
- Original TC-cli-0106 observation `tc-cli-0106-kimi-original` failed in
  `local-macos-arm64` because mapping guidance and consistent corrected documents
  were absent at base. Its discovery and preserved run-fact outcomes do not make the
  overall cold-resume objective a pass.

Separately from the original Kimi run, failure-first TC-cli-0105 observation
`tc-cli-0105-1793551352` failed in `local-macos-arm64` on the pre-fix archived-action
regression. Any later TC-cli-0106 review uses `repository-text` and is
documentation-only, not another host session; its outcome must come from its own
actual observation. No fresh post-fix Kimi run is recorded.

### Integrated new-cycle verification

The merged native-routing override violated main's newer REQ-cli-021 behavior. This
is BUG-0025, separate from the earlier BUG-0024 archived-resume defect and main's
open BUG-0020. Removing only the override restored the shared `stage::assess` action.

- At revision `f38e70f`, after updating the TC-cli-0105 definition and regression but
  before editing `src/main.rs`, `cargo run --locked -- cases run TC-cli-0105
  --environment local-macos-arm64 --observation-id tc-cli-0105-bug-0025-pre-fix`
  recorded a failed observation. The case command exited 3 and the product test
  exited 101 because `next` returned `Continue done: stub next action for archived`
  instead of the new-cycle reset. The project bug command linked it to BUG-0025,
  whose derived state became `reproduced`.
- After the fix, `cargo test --locked --test stages` passed 24 tests and
  `cargo test --locked --test next` passed 3 tests. Passing observations are
  `tc-cli-0105-bug-0025-post-fix`, `tc-cli-0121-new-cycle-post-fix`, and
  `tc-cli-0122-reset-next-action-post-fix`, all in `local-macos-arm64`.
- Linking the later TC-cli-0105 pass with `cargo run --locked -- bug update BUG-0025
  --observation tc-cli-0105-bug-0025-post-fix --status verified` changed BUG-0025 to
  `verified` with derived state `reproduced-and-later-passed`. The earlier failure
  remains linked and historical.
- With HI 0.8.0, Fledge 1.7.2 and SpecSync 6.1.1, `fledge lanes run verify` passed
  all 9 steps and `fledge lanes run pre-push` passed all 5 steps on macOS arm64.

These are local structural and product-test observations, not hosted checks, human
implementation review, another Kimi cold start, release, or deployment. Original
observations and frozen archives were not edited.

## Identity takeover verification - 2026-11-01

The identity implementation was resumed from Kimi's uncommitted work at
`6800b2dadebec358ab1a6fdb18b5d78136eb6631` in an independent clone on the same
PR branch; the original workspace was preserved.

- All four slug cases passed on local-macos-arm64 using automatic observation
  allocation. Their original failed observations, artifact bytes and bug links are
  retained. Each bug links its strictly later pass; case records name the generated
  OBS IDs and inherited artifact logs.
- The initial takeover passed 181 tests on Rust 1.99.1 and Rust 1.88.0 with exactly
  `both_forms_stay_within_the_cap_with_totals_for_large_records` excluded. TC-cli-0120
  separately retained that fixture's failure: it supplied obsolete `current-one` to
  a new observation write, which the approved identity policy correctly refused.
- 0xLeif then approved the separate one-file native definition
  `update-compact-next-fixture-for-slug-observations`. The fixture now allocates
  automatically and references the returned observation ID. Every original report
  assertion and stale fixture remains unchanged. The full tests/next.rs suite
  passed all 3 tests with Rust 1.99.1, including the original failing regression.
- After this fix, the unfiltered Rust 1.88.0 suite passed all 182 tests, with no
  ignored or filtered tests. The raw focused and full-suite logs are retained with
  the takeover's Chough observations.
- The unfiltered Rust 1.99.1 suite also passed all 182 tests, with no ignored or
  filtered tests. Formatting, warnings-denied all-target Clippy, HI, strict spec
  checks, case-definition validation, the real CLI exercise and build passed.
- The full verify lane stopped at native audit, step 6: tests/next.rs was reported
  uncovered because its approved follow-up depends on the identity change being
  accepted first. A direct native follow-up check explicitly refused while that
  parent is verifying. This accepted-first edge is unsuitable for co-reviewed
  fixture work; it is a planning/ordering blocker, not another test failure.
  The full lane is not passed and the fixture native change is not verified.
- Tool PATH selects HI 0.8.0, Fledge 1.8.0 and SpecSync 6.1.2. Raw results and the
  ordering refusal remain in `.chough/evidence/` and `.chough/artifacts/`.
  A structural native check never substitutes for product checks or acceptance.

Original failures and prior partial results remain retained. Hosted platform results,
human implementation review, native archive and merge are separate evidence and gates.

## Observed evolution verification — 2026-10-31

Environment: macOS arm64, rustc 1.95.0, hi 0.8.0, fledge 1.7.2, specsync 6.1.1 (the
versions CI used for these observations). Product tip for the observations below: 5074711, after the scoped
native check.

- Fledge tasks: fmt, lint (Clippy, all targets, warnings denied), test (37 integration
  tests across tests/cli.rs, stages.rs, cases.rs, bugs.rs and optional.rs), intent (20
  active criteria, no problems), spec (strict, 100% file and LOC coverage), audit (1
  active change, passed after BUG-0001's out-of-scope edit was reverted), cases (40
  definitions valid), exercise (EXERCISE PASSED) and build all passed.
- `specsync change check --commit` verified the change and materialized REQ-cli-007
  to REQ-cli-017; native state `verifying`; human review pending.
- `chough cases run` recorded TC-cli-0001 to TC-cli-0037 and TC-cli-0101 as passed in
  `local-macos-arm64`, each with its captured log; `chough cases record` recorded the
  agent-assisted TC-cli-0102 (cold start) and TC-cli-0103 (wording) as passed with
  their limits. Records: `.chough/evidence/`, logs and notes: `.chough/artifacts/`.
  Those records were written before observations carried a content identity, so
  they keep the revision-bound comparison and read stale-revision-or-inputs after any
  commit. Records written since keep their freshness across commits that change no
  fingerprinted content (REQ-cli-004 as modified by
  keep-evidence-current-across-commits-and-specsync-lifecycle-bookkeeping-and-verify-bugs-only-by-a-pass-observed-after);
  hosted CI reruns the product tests at the tip.
- Exercises: docs/exercises/feature-bug-loop.md, visual-design.md, intent.md and
  cold-start.md, with diagrams verified in docs/verification.md.

Not covered: native Windows (unsupported), anything run inside WSL itself (WSL 1, a
checkout on a Windows drive, Windows-side Git), Cursor, Gemini and Grok discovery and
cold starts, any post-fix Kimi run, real releases, live
monitoring, and any human acceptance.

## Observed bootstrap verification — 2026-10-31

On macOS arm64, `fledge lanes run verify` passed formatting, all-target Clippy,
14 Rust integration tests, HI structure (7 active criteria, no problems), native
SpecSync strict structure (active contract, no warnings, 100% eligible file/LOC
coverage), and the build. The native structural pass did not execute product tests.
Source installation also succeeded with `cargo install --path . --locked --offline`
into an isolated temporary prefix; the installed executable reports chough 0.1.0.
Unix-specific adapter/symlink/mode tests are gated accordingly; four portable
record/init tests are defined for Windows. Hosted CI results are separate.

An independent agent used the actual skill, compiled helper, HI 0.8.0, Fledge
1.7.2 and SpecSync 6.1.0 in an isolated checkout fixture. It executed three existing
acceptance tests: the exact $50 shipping threshold failed; adjacent values passed.
A standalone boundary reproduction confirmed the equality defect in 3/3 runs.
It classified/deduplicated the finding against an existing native task, drafted a
local report, recorded two failed product-test observations tied to CHECKOUT-1 /
REQ-checkout-001, and resumed with report/evidence paths and explicit missing-source
markers. Both observations were current in the recorded environment; mismatched
environment queries correctly reported stale-environment. Tracked source, tests,
intent and native records stayed unchanged; nothing was posted or committed.

The fixture is a deliberately defective sample, not a defect claimed in Chough.
Temporary artifacts: `/tmp/chough-skill-eval.XrU81o/bugs/shipping-at-50.md`,
`bugs/chough-workflow-evaluation.md`, and its `.chough/session.json`/evidence records.
They are local evaluation artifacts, not durable repository dependencies. The
supported startup-order and scope wording friction found by that exercise was
corrected in the skill and default session guidance.

All three Mermaid diagrams were rendered and visually inspected; the separate
documentation record is docs/verification.md. No authentic human acceptance,
full approved/archived native lifecycle, release or deployment was exercised.

## Checkpoint writer capacity refusals

REQ-cli-032 writer regressions in tests/cli.rs exercise observation overflow,
pretty-serialized byte overflow and the first history head's serialized growth.
They snapshot every managed record and retain exact-bound successful controls,
clear-before-open gate order, selected history and legacy cursor-free behavior.
Run with `fledge run test -- --locked --test cli checkpoint_writer_ -- --nocapture`.
The immutable failure-first inputs and logs are under
.chough/artifacts/pr51-writer-atomicity/before/. Later executions and their actual
source identities are separate observations in that followup's qualification
package; they never restamp the original failure or historical archive.

## Pinned released skill coverage (#102, REQ-cli-016)

`tests/skill_releases.rs` validates the exact offline manifest digest, tag commits/archive identities,120 mappings/51 blobs and24 current bundle identities before invoking the binary. All5released versions and6host routes are covered; source files and tests keep filename plus exact byte matching, edited byte/mode preservation, no newline normalization, current/missing/shared-host and legacy rules.

Retained runtime receipts: source6bb513b t2ns ran7,5passed/2failed at recognition/update (54 expected-known-outdated rows each were edited/preserved-edit); sourced9a9d924 t2p7 passed SAME7/7, existing optional4/4, cli3/3 and package16/16. Declared fmt/lint passed last. Product source/test/corpus inputs are unchanged by subsequent documentation/native bookkeeping; no fresh case run is claimed by recording those receipts. Full/site/Trust/hosted/human acceptance are not established.

## Diagnostic inventory reader compatibility (acceptance pending)

Retained actual t2sm atbe17 compiled the original6:1PASS/5unknown-input_inventory FAIL after valid baselines; t2vg ata7b2543 executed only2alias tests:0PASS/2intended invalid-alias-accepted FAIL,6filtered. Source fix rejects noncanonical spelling via unchanged canonical_path before raw classification, without rewriting records. One independently confirmed uppercase digest oracle was corrected to preserve existing is_digest compatibility.

Actual t2vj atd0125 passed8compatibility (original6 with ONE uppercase oracle correction +SAME2aliases),14 existing evidence module and30 evidence integration tests=52PASS. All9 original records remain historical/partial with2failed7passed/exact values and bytes; legacy/null/strictcaps/advisory/no-promotion and every alias-loop assertion executed. Declaredfmt failed only test formatting; lint thenNOTRUN. Pinned standalone canonical formatting at4b35 is byte-equal to rustfmt(before), with no assertion edit. Actual t2vn at4b35 declaredfmt0/alltargetlint0; NO behavioral rerun. Prior52PASS is source-equivalent, not fresh4b35execution.

Source/test/corpus/DUT/tool/argv/fences and raw failures are retained in the external program evidence/chough-inventory-compat-* packets. These are agent-assisted retained receipts, not authenticated test execution or human acceptance. Full/Trust/site/hosted/platform/humanreview are not established. Inventory is diagnostic only, never aggregate freshness/path attribution/authority. Known inherited issue102 next_owner is preserved and separate; tested own next read0 here.
