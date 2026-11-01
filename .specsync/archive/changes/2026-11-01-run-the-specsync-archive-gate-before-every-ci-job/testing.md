---
change: run-the-specsync-archive-gate-before-every-ci-job
artifact: testing
---

# Testing

- `ci.yml` parses and every job other than `archive-gate` needs it.
- `python3 .github/scripts/test-specsync-archive.py` passes.
- On the pull request, `Archive` runs first and passes on the archived tip; every other job runs after it.
