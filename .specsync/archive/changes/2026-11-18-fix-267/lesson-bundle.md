# Lesson bundle: fix-267

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Fix 267
- **Kind**: BugFix
- **Specs**: records
- **Paths**: src/leaks.rs, tests/leaks.rs, .chough/cases/records.hyphen-glued-token.cases.toml, changelog.d/2026-11-18-fix-267.md
- **Acceptance**: A token form that directly follows a hyphen, such as x-ghp_<token> in a checkpoint question, a session key or a raw log name, refuses the write as refused:leak naming the field and the kind token, with every file unchanged; chough doctor reports the same text in a committed record as a leak finding. The same token after a space, colon or dot is still refused.

## Evidence

- Verification commit: `2b76da4e88583469531ffcbaaa44f839760cb524`
- Base commit: `8abb5bb89b5244e72327e1ea6abfc7e66634ebe1`
- Verified by: `specsync check --spec records`

## From the change's context.md

# Context

GitHub #267: a GitHub token glued to a hyphen, `x-ghp_<token>`, is accepted in a checkpoint question, a session key and a raw log name and is committed byte for byte, and `chough doctor --leaks` does not report it. The same token after a space, colon or dot is refused. Slugs, IDs and CI run names produce exactly this hyphen-glued shape. REQ-records-006 already refuses a credential token form in every value Chough writes and has the doctor scan for the same patterns. On main at 8abb5bb the matcher skips a token prefix when the byte before it is a letter, digit, underscore or hyphen, so a hyphen hides the token. This change makes a hyphen a boundary, in the prefixed forms and in the AWS key form alike. A letter, digit or underscore before the prefix still continues a word, as before. It changes no requirement text. #268 (the --file write check) is a separate, parked issue and is not touched.

## From the change's testing.md

# Testing

`cargo test --locked --bin chough leaks::tests::a_token_glued_to_a_hyphen_is_found` checks the matcher on a GitHub, a GitLab and an AWS form after a hyphen, and that a letter or underscore before the prefix still finds nothing. `cargo test --locked --test leaks a_hyphen_glued_token_is_refused_at_the_write_and_found_by_the_doctor` checks that a decision and a session key holding `x-ghp_<token>` refuse as `refused:leak` with every file unchanged, and that a committed file holding it gives one `leak` finding of kind `token` from `chough doctor --leaks`. Both fail on main. Case `TC-records-a-hyphen-glued-token-is-a-leak` binds the second test. The verify lane's `leaks` step scans this repository's own records with the changed matcher.

## Where these lessons go

- `specs/records/context.md`
