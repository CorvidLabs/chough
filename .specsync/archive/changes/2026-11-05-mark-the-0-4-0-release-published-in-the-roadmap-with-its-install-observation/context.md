---
change: mark-the-0-4-0-release-published-in-the-roadmap-with-its-install-observation
artifact: context
---

# Context

Leif approved all Chough 0.4.0 release steps (2026-11-05 11:26 MST, and "Yes, run them now", through the orc). The release prep merged in #116 as 69950c7a, with main CI green there, MSRV included. The orc tagged `v0.4.0` at 69950c7a, published `chough` 0.4.0 to crates.io from a fresh clone of the tag, and created the GitHub release. A clean `cargo install chough --version 0.4.0 --locked` into an empty root on this Mac (cargo 1.95.0, macOS 26.5.2 arm64) exited 0, and `chough --version` printed `chough 0.4.0`. Step 6 of docs/releasing.md marks the release published in the roadmap.
