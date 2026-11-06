---
change: read-requirements-with-crlf-line-endings-in-the-librarian-default-test-131-windows
artifact: testing
---

# Testing

- tests/optional.rs `the_requirements_give_the_librarian_one_default_and_it_is_on` runs `librarian_default_contradictions` on the requirements in LF and in CRLF form, and asserts that both are empty. It fails first on the CRLF form on Linux.
- The hosted `Product / windows-latest` job on the PR head must pass.
- `fledge lanes run verify` passes on Linux.
