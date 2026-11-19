---
change: fix-301
artifact: testing
---

# Testing

No product test beyond the help-command test. Verify `chough checkpoint --help` prints help text for each of the eleven list-valued flags, that `docs/cli.md` documents them, and that `chough help --help` exits 0 with help text (covered by a test).
