---
change: scale-independent-archive-authentication-workers-with-the-runner-s-cpus-so-the-archive-check-fits-its-unchanged-budget
artifact: testing
---

# Testing

- `python3 .github/scripts/test-specsync-archive.py`:
  - A new worker-count test maps CPU counts 1, 3, 4, 64 and None to 2, 3, 4, 8 and 2.
  - The overlap test runs with 3 CPUs: exactly three reads overlap, no fourth starts until one is released, every ID is read once and the peak is 3.
  - The new tests fail against main 1ac2ae2.
  - The existing error-order, drain and real native 6.1.2 fixture tests pass unchanged.
- The check is timed on the #71 head (base 1ac2ae2) with 2, 4 and 8 workers.
- `fledge lanes run verify` passes.
