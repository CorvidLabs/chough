---
change: move-the-pinned-specsync-toolchain-to-6-2-0-every-specsync-action-step-at-the-v6-2-0-release-commit-with-an-explicit
artifact: design
---

# Design

No product design change. Each pin keeps its current place and form; only the version
and the Action commit change. `tests/package.rs::tool_and_rust_version_pins_agree`
continues to enforce agreement, with its mutation cases moved to the 6.2.0 strings.
