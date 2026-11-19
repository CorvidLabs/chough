---
change: fix-314
artifact: testing
---

# Testing

- `cargo test --locked --test special_files macos_metadata_files_in_chough_stop_no_reader` puts AppleDouble `._<name>.md` and `.DS_Store` files into `.chough/bugs/`, a session's `evidence/` and `.chough/archive/sessions/` and requires `next`, `status` and `bug list` to succeed. It requires `cases check` to pass with both kinds in `.chough/cases/`. It requires `doctor` to report only a committed malformed ordinary record as `unreadable`, not a committed `._` file. It fails on main.
- `cargo test --locked --test special_files a_leak_in_a_binary_metadata_file_is_found_and_left_unrewritten` commits a binary AppleDouble file holding an email. `doctor --leaks` must report it, and `doctor --fix --redact` must leave it byte-identical with the finding `skipped`. It fails on main.
- `cargo test --locked --test migrate migrate_skips_macos_metadata_beside_0_5_records` takes the plan digest of a 0.5.x fixture, adds AppleDouble and `.DS_Store` files beside its bugs, sessions, archived sessions, inbox entries, evidence, history events and knowledge notes, and requires the same digest from the preview, a successful apply, and every one of those files still there after it. It fails on main. Its knowledge and kept-after parts also failed on the first recreation, before the `library.rs` skip and the empty-folder rule.

Case `TC-cli-macos-metadata-files-stop-no-reader` binds the first, `TC-cli-a-leak-in-a-binary-metadata-file-is-found` the second, and `TC-records-migrate-skips-macos-metadata` the third.
