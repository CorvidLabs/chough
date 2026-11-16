---
change: the-leak-check-flags-an-scp-style-host-path-as-the-host-name-and-the-path-it-carries-275
artifact: tasks
---

# Tasks

- [x] `names_a_host` treats `<host>:/<path>` as a host context (`src/leaks.rs`).
- [x] Unit test `leaks::tests::an_scp_style_remote_path_is_checked_like_the_path_it_carries`, failing before the fix.
- [x] REQ-records-006 text and acceptance criterion; `specs/records/records.spec.md` change log; `docs/cli.md`; CHANGELOG.
- [x] Case `TC-records-an-scp-style-host-path-is-checked-like-the-path-it-carries` in its own `.chough/cases/records.scp-host-path.cases.toml`.

Human gates (not checkboxes): scope approval, implementation review, merge.
