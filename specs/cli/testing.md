# CLI testing

Product checks execute through `fledge run test`; structural checks through
`fledge run spec`; HI structure through `fledge run intent`. Structural success
cannot prove the behaviors below. Final observed results are recorded after the
implementation is exercised.

| Requirement | Verification responsibility |
|-------------|-----------------------------|
| REQ-cli-001 | Init repeated around edited files and partial installed bundles |
| REQ-cli-002 | Focused ordered output and session resumption in isolated repo |
| REQ-cli-003 | Native status passthrough and unavailable tool states |
| REQ-cli-004 | Current → changed content → stale; environment mismatch/unchecked |
| REQ-cli-005 | Unknown criterion/requirement and duplicate observation rejected |
| REQ-cli-006 | Traversal/symlink escape, unsupported schema and bounded output |

No physical deployment, external agent runtime, authenticated human acceptance,
or unbuilt integration is covered by this test contract.

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
