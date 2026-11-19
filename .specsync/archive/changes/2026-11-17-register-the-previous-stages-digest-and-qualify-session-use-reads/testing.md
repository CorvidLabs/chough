---
change: register-the-previous-stages-digest-and-qualify-session-use-reads
artifact: testing
---

# Testing

`cargo test --locked --test docs` passed, 21 tests, including `stages_from_main_0fd9fd22_update_and_an_edited_copy_stays`. `chough cases run TC-cli-previous-stages-0fd9fd22 --environment local-macos-arm64` passed. `cargo fmt --check` and `cargo clippy --all-targets -- -D warnings` passed.

The regression checks three things: the fixture hashes to `36522cbfa49c0d29b2505c552ff15d3743b9bca7c5985db0fb002d6b61974727`, a pristine copy is known-outdated and `chough skill update` replaces it with the current bundle, and an edited copy stays byte-identical with action `preserved-edit`. The same test checks that `docs/architecture.md` no longer says `session use` reads only the session it activates, and that it names the exact-ID, bare-key, and archived-story reads.
