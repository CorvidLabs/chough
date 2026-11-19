---
change: fix-268
artifact: testing
---

# Testing

`cargo test --locked --test leaks a_token_in_a_file_or_design_path_is_refused_at_the_write` checks that `chough checkpoint --file` and `--design` given a path that holds a GitHub token refuse as `refused:leak: file holds token` and `refused:leak: design holds token`, without echoing the token and with every file unchanged, that `--redact` stores `<redacted:token>` in the path and says it redacted, and that a clean path is still saved. It fails on main. Case `TC-records-a-file-or-design-path-passes-the-leak-guard` binds it. The verify lane's `leaks` step scans this repository's own records.
