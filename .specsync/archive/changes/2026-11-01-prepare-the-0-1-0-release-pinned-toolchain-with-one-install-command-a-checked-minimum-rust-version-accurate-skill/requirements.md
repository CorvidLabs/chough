---
change: prepare-the-0-1-0-release-pinned-toolchain-with-one-install-command-a-checked-minimum-rust-version-accurate-skill
artifact: requirements
---

# Requirements

Modified in `deltas/cli.md`; no requirement is added or removed:

- REQ-cli-008 (stage routing): acceptance adds the test and accept routes, which the
  statement and src/stage.rs already gate. Statement unchanged.
- REQ-cli-016 (host skill installation): acceptance no longer names a 0.1.0 bundle,
  which does not exist, but a file byte-identical to a known earlier build. Statement
  unchanged.
- REQ-cli-019 (installable package): the package declares its minimum Rust version,
  checked in CI, and the README gives one command installing the HI, SpecSync and
  Fledge versions CI uses, kept in agreement by a test.

Trace: LIFE-4, LIFE-6, LIFE-9. These HI criteria exist and are only linked; no HI
criterion is added or reworded (see open question 2 on a toolchain criterion).
