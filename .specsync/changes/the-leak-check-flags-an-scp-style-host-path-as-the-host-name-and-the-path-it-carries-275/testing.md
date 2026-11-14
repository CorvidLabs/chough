---
change: the-leak-check-flags-an-scp-style-host-path-as-the-host-name-and-the-path-it-carries-275
artifact: testing
---

# Testing

`leaks::tests::an_scp_style_remote_path_is_checked_like_the_path_it_carries` uses a synthetic host and user. Before the fix, `brp:/Users/SYNTHETIC-person/project` yielded only `home-path`; after it yields `host-name` and `home-path`, and the refusal text, the `--redact` replacement and doctor's per-line scan agree. Case `TC-records-an-scp-style-host-path-is-checked-like-the-path-it-carries` binds it.
