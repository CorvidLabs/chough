---
change: fix-267
artifact: testing
---

# Testing

`cargo test --locked --bin chough leaks::tests::a_token_glued_to_a_hyphen_is_found` checks the matcher on a GitHub, a GitLab and an AWS form after a hyphen, and that a letter or underscore before the prefix still finds nothing. `cargo test --locked --test leaks a_hyphen_glued_token_is_refused_at_the_write_and_found_by_the_doctor` checks that a decision and a session key holding `x-ghp_<token>` refuse as `refused:leak` with every file unchanged, and that a committed file holding it gives one `leak` finding of kind `token` from `chough doctor --leaks`. Both fail on main. Case `TC-records-a-hyphen-glued-token-is-a-leak` binds the second test. The verify lane's `leaks` step scans this repository's own records with the changed matcher.
