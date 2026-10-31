# CLI testing

Product checks run through `fledge run test`; structural checks through `fledge run
spec`; HI structure through `fledge run intent`; native lifecycle coherence through
`fledge run audit`; case definitions through `fledge run cases`; the end-to-end
sample through `fledge run exercise`. Structural success cannot prove the behaviors
below. Stable case definitions are in `.chough/cases/cli.cases.toml` (TC-cli-0001 to
TC-cli-0037 bind the integration tests one to one; TC-cli-0101 to TC-cli-0103 cover
the exercise, the cold start and the wording review).

| Requirement | Verification responsibility | Cases |
|-------------|-----------------------------|-------|
| REQ-cli-001 | Init repeated around edited files and partial installed bundles | TC-cli-0001 |
| REQ-cli-002 | Focused ordered output and session resumption in isolated repo | TC-cli-0002 |
| REQ-cli-003 | Native status passthrough and unavailable tool states | TC-cli-0003, 0004 |
| REQ-cli-004 | Current → changed content → stale; environment mismatch/unchecked | TC-cli-0005 to 0008 |
| REQ-cli-005 | Unknown criterion/requirement and duplicate observation rejected | TC-cli-0009 |
| REQ-cli-006 | Traversal/symlink escape, unsupported schema and bounded output | TC-cli-0010 to 0014 |
| REQ-cli-007 | Schema 2 checkpoint, cross-process round trip, explicit migration | TC-cli-0015 to 0017, 0102 |
| REQ-cli-008 | Every transition kind, history, native gates, stage contract parity | TC-cli-0018 to 0020, 0032, 0101 |
| REQ-cli-009 | Definition validation, location in errors, position-free digests | TC-cli-0022 to 0024 |
| REQ-cli-010 | Focused selection with tracks and omissions | TC-cli-0025, 0101 |
| REQ-cli-011 | Failing, not-run, missing-output, timeout and build-failure outcomes; verdicts; stop and resume | TC-cli-0026, 0027, 0034, 0101, 0103 |
| REQ-cli-012 | Reproduce, deduplicate, regression baseline, blocking scope | TC-cli-0029 to 0032, 0101 |
| REQ-cli-013 | Preferences separate from consent; authorization for validate | TC-cli-0021, 0033, 0034, 0103 |
| REQ-cli-014 | Release only when authorized; dry-run kind | TC-cli-0021, 0028, 0103 |
| REQ-cli-015 | Snapshots keep sources; index lists open bugs; symlinks refused | TC-cli-0035, 0036 |
| REQ-cli-016 | Per-host install; update keeps edits | TC-cli-0037 |
| REQ-cli-017 | Schema 2 observations; schema 1 still read | TC-cli-0026, 0028 |

Unix-only tests are gated with `cfg(unix)` because they use shell stubs for native
tools; their cases list only macOS and Linux environments. Windows runs the portable
tests and the build.

No physical deployment, live monitoring, authenticated human acceptance, APS,
Magpie or other unbuilt integration is covered by this test contract.

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
