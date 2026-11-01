# CLI testing

Product checks run through `fledge run test`; structural checks through `fledge run
spec`; HI structure through `fledge run intent`; native lifecycle coherence through
`fledge run audit`; case definitions through `fledge run cases`; the end-to-end
sample through `fledge run exercise`. Structural success cannot prove the behaviors
below. Stable case definitions are in `.chough/cases/cli.cases.toml`. TC-cli-0001 to
TC-cli-0037 bind the original integration tests one to one, with later automated
cases extending that coverage. TC-cli-0101 to TC-cli-0104 cover the exercise, cold
start, wording review and timing. TC-cli-0105 is the BUG-0020 archived-guidance
regression, and TC-cli-0106 is the BUG-0021 retrospective Kimi guidance review.

| Requirement | Verification responsibility | Cases |
|-------------|-----------------------------|-------|
| REQ-cli-001 | Init repeated around edited files and partial installed bundles | TC-cli-0001 |
| REQ-cli-002 | Focused ordered output and session resumption in isolated repo | TC-cli-0002 |
| REQ-cli-003 | Native status passthrough and unavailable tool states; archived native guidance for next and status | TC-cli-0003, 0004, 0105 |
| REQ-cli-004 | Current → changed content → stale; environment mismatch/unchecked; content identity kept across Chough-only, empty and already-present commits and SpecSync bookkeeping and archive moves, stale on any scope or source edit; legacy records stay revision-bound | TC-cli-0005 to 0008, 0067 to 0073 |
| REQ-cli-005 | Unknown criterion/requirement and duplicate observation rejected | TC-cli-0009 |
| REQ-cli-006 | Traversal/symlink escape, unsupported schema and bounded output | TC-cli-0010 to 0014 |
| REQ-cli-007 | Schema 2 checkpoint, cross-process round trip, explicit migration; checkpoint resumption, paused and awaiting-human precedence, safe missing-native fallback and unchanged session guidance; TC-cli-0106 retrospectively reviews the original Kimi cold-start record and guidance without a new host session | TC-cli-0015 to 0017, 0102, 0105, 0106 |
| REQ-cli-008 | Every transition kind, history, native gates, stage contract parity | TC-cli-0018 to 0020, 0032, 0101 |
| REQ-cli-009 | Definition validation, location in errors, position-free digests | TC-cli-0022 to 0024 |
| REQ-cli-010 | Focused selection with tracks and omissions | TC-cli-0025, 0101 |
| REQ-cli-011 | Failing, not-run, missing-output, timeout and build-failure outcomes; verdicts; stop and resume | TC-cli-0026, 0027, 0034, 0101, 0103 |
| REQ-cli-012 | Reproduce, deduplicate, regression baseline, blocking scope; verified only by a pass observed after the latest failure | TC-cli-0029 to 0032, 0076 to 0079, 0101 |
| REQ-cli-013 | Preferences separate from consent; authorization for validate | TC-cli-0021, 0033, 0034, 0103 |
| REQ-cli-014 | Release only when authorized; dry-run kind | TC-cli-0021, 0028, 0103 |
| REQ-cli-015 | Snapshots keep sources; index lists open bugs; symlinks refused | TC-cli-0035, 0036 |
| REQ-cli-016 | Per-host install; TC-cli-0037 proves byte-identical installation and safe update mechanics that preserve edits, while TC-cli-0106 reviews the Kimi mapping and documentation without a new host session; 2756dc2 references known-outdated | TC-cli-0037, 0075, 0106 |
| REQ-cli-017 | Schema 2 observations; schema 1 still read; optional content identity validated | TC-cli-0026, 0028, 0073, 0074 |

Unix-only tests are gated with `cfg(unix)` because they use shell stubs for native
tools; their cases list only macOS and Linux environments. Windows runs the portable
tests and the build. The content-identity freshness tests (TC-cli-0067 to 0074, with
a CRLF `change.md` in TC-cli-0070) and the bug-ordering tests (TC-cli-0076 to 0079)
are portable: their `hi` and `specsync` stand-ins are compiled programs on Windows, so
Windows CI exercises the `/`-separated path and change keys and CRLF front matter; the
exec-bit test stays Unix-only.

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
  `target/tmp/ci-tools`; final checks require those versions with SpecSync 6.1.1.
- Safe cold resume exposed BUG-0020, and absent mapping guidance exposed BUG-0021.
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

## Observed evolution verification — 2026-10-31

Environment: macOS arm64, rustc 1.95.0, hi 0.8.0, fledge 1.7.2, specsync 6.1.1 (the
versions CI uses). Product tip for the observations below: 5074711, after the scoped
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

Not covered: Windows execution of the Unix-gated tests (shell stubs), Cursor, Gemini
and Grok discovery and cold starts, any post-fix Kimi run, real releases, live
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
