---
change: fix-295-the-init-case-runs-the-real-preservation-test
artifact: testing
---

# Testing

`chough cases run TC-cli-walk-ready-init-preserves --environment local-macos-arm64` runs the cargo test `init_preserves_user_files_and_installs_a_complete_skill`. That test keeps edited session and skill bytes, then deletes one bundle file and checks a later init adds it without rewriting the edited neighbor.
