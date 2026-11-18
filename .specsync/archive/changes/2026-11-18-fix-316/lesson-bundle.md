# Lesson bundle: fix-316

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Fix 316
- **Kind**: BugFix
- **Specs**: records
- **Paths**: src/leaks.rs, tests/leaks.rs, .chough/cases/records.sk-after-a-hyphen.cases.toml, changelog.d/2026-11-18-fix-316.md
- **Acceptance**: A hyphen before sk- continues a word again, as in 0.6.0: chough bug add --title 'Fix flaky sk-learn tests in CI pipeline' writes its bug, a committed 0.6.0 bug whose id holds -sk- followed by 20 or more letters, digits and hyphens accepts chough bug update, and chough doctor --leaks reports no leak in it. sk- followed by 20 or more such bytes at the start of a value or after a space is still refused as a token, and a GitHub, GitLab, Slack or AWS token form directly after a hyphen, such as x-ghp_<token>, is still refused.

## Evidence

- Verification commit: `d52fe25e13a32daf616006d8f418dbe018eb2a4a`
- Base commit: `66543c2bf509627ec2a165414acae0bc8ab59c4a`
- Verified by: `specsync check --spec records`

## From the change's context.md

# Context

GitHub #316, a regression from #303 (the fix for #267) and a release blocker for 0.6.1. #303 made a hyphen a token boundary before every token prefix. For `sk-`, the model-provider form, that catches ordinary slugs: any value with `-sk-` followed by 20 or more letters, digits and hyphens reads as a key, including IDs Chough generates from a title. So `chough bug add --title "Fix flaky sk-learn tests in CI pipeline"` refuses with `refused:leak: id holds token`, even with `--redact`. And a bug that 0.6.0 committed as `2026-11-18-use-sk-learn-for-the-classifier-model` can no longer be updated, and `chough doctor --leaks` fails on it. The macOS swarm tester found it, and the planning session reproduced it on main.

Leif's decision (2026-11-18, relayed by hc t4c): "Narrow it: not for sk-". Keep the hyphen boundary for the distinctive prefixes (the GitHub forms, `glpat-`, `xox*-`, `AKIA`/`ASIA`), and go back to 0.6.0's rule for `sk-` only, where a hyphen before the prefix continues a word. `AIza` keeps #303's boundary, since only `sk-` is narrowed.

REQ-records-006 names model-provider token forms without fixing the byte before a prefix, so no requirement text changes. Not in scope, and filed in #316 as a follow-up: `doctor --fix --redact` should never rewrite a record's ID into one that no longer matches its file name.

## From the change's testing.md

# Testing

`cargo test --locked --bin chough leaks::tests::sk_after_a_hyphen_continues_a_word` checks the matcher: `sk-` with 20 or more token bytes after a hyphen is not a token (a generated bug ID such as `2026-11-18-fix-flaky-sk-learn-tests-in-ci-pipeline`, and `x-sk-<24 bytes>`), while the same `sk-` run at the start of a value or after a space is, and `x-ghp_<token>`, `run-glpat-<token>`, `team-xoxb-<token>` and `ci-AKIA<16>` after a hyphen still are. `cargo test --locked --test leaks a_slug_holding_sk_is_not_a_key_and_a_glued_token_still_is` checks that `bug add --title "Fix flaky sk-learn tests in CI pipeline"` writes its bug, that a committed 0.6.0 bug whose ID holds `-sk-learn-…` accepts `bug update` and gives no `leak` finding from `chough doctor --leaks`, and that a decision holding `x-ghp_<token>` is still refused. Both fail on main. Case `TC-records-sk-after-a-hyphen-is-not-a-key` binds the second; `TC-records-a-hyphen-glued-token-is-a-leak` keeps covering #267.

## Where these lessons go

- `specs/records/context.md`
