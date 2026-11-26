---
change: release-prep-0-6-4
artifact: testing
---

# Testing

Structural checks (no product behavior change — version bump + record assembly):

- `cargo test --locked` at 0.6.4 — all suites pass.
- `chough library index --check` — exit 0, index current.
- `fledge run release-check` — exit 0.
