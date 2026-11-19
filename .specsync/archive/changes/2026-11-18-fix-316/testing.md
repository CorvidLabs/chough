---
change: fix-316
artifact: testing
---

# Testing

`cargo test --locked --bin chough leaks::tests::sk_after_a_hyphen_continues_a_word` checks the matcher: `sk-` with 20 or more token bytes after a hyphen is not a token (a generated bug ID such as `2026-11-18-fix-flaky-sk-learn-tests-in-ci-pipeline`, and `x-sk-<24 bytes>`), while the same `sk-` run at the start of a value or after a space is, and `x-ghp_<token>`, `run-glpat-<token>`, `team-xoxb-<token>` and `ci-AKIA<16>` after a hyphen still are. `cargo test --locked --test leaks a_slug_holding_sk_is_not_a_key_and_a_glued_token_still_is` checks that `bug add --title "Fix flaky sk-learn tests in CI pipeline"` writes its bug, that a committed 0.6.0 bug whose ID holds `-sk-learn-…` accepts `bug update` and gives no `leak` finding from `chough doctor --leaks`, and that a decision holding `x-ghp_<token>` is still refused. Both fail on main. Case `TC-records-sk-after-a-hyphen-is-not-a-key` binds the second; `TC-records-a-hyphen-glued-token-is-a-leak` keeps covering #267.
