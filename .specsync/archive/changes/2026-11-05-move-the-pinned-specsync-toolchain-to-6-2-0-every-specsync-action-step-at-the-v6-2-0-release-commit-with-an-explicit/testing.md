---
change: move-the-pinned-specsync-toolchain-to-6-2-0-every-specsync-action-step-at-the-v6-2-0-release-commit-with-an-explicit
artifact: testing
---

# Testing

- `cargo test --locked --test package tool_and_rust_version_pins_agree` with the pins
  moved, and the full `fledge lanes run verify` on macOS with HI 0.8.0, Fledge 1.8.0 and
  SpecSync 6.2.0.
- `python3 .github/scripts/test-specsync-archive.py` (CI gate fixtures; needs real
  published SpecSync 6.2.0).
- A grep shows no current 6.1.x pin outside recorded history.
- Hosted PR checks are observed only after they run; SpecSync step durations are read
  with `gh run view --json jobs` for the PR run and the most recent main run.
