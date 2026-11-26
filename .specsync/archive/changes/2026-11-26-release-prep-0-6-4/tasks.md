---
change: release-prep-0-6-4
artifact: tasks
---

# Tasks

- T1: bump Cargo.toml to 0.6.4, refresh Cargo.lock — done at verification
- T2: fledge run changelog -- --release 0.6.4 --date 2026-11-26; verify heading + link
- T3: chough library index; fledge run release-check until green
- T4: cargo test --locked green at 0.6.4
- T5: commit, push branch release-prep-0-6-4, open PR to main
