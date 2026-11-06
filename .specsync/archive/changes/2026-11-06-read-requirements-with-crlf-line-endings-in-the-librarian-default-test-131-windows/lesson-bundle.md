# Lesson bundle: read-requirements-with-crlf-line-endings-in-the-librarian-default-test-131-windows

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Read requirements with CRLF line endings in the librarian-default test (131 Windows)
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: tests/optional.rs
- **Acceptance**: The librarian-default test reads the requirements correctly with LF and with CRLF line endings, checking both; it fails first on the CRLF form; the hosted Windows product job passes; fledge lanes run verify passes on Linux.

## Evidence

- Verification commit: `0a143163eef68f70ba8b1c75ad36a12306c0977c`
- Base commit: `217680e292733680b377daee33b37dcde14b9424`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

PR #131 (C2, #124) was reviewed (Leif, "Record and merge", 19:50 MST) and closed out. Its hosted `Product / windows-latest` job then failed in `the_requirements_give_the_librarian_one_default_and_it_is_on`, at run 42153579452 on head `217680e2`.

Windows checks files out with CRLF line endings. The test replaced only `\n` before splitting sentences on ". ", so sentences ending ".\r" were never split. The resulting chunks named the librarian, a default and "off" together, and the check failed on them. Linux and macOS passed. This is a defect in the test, not in the requirement or the product.

## From the change's testing.md

# Testing

- tests/optional.rs `the_requirements_give_the_librarian_one_default_and_it_is_on` runs `librarian_default_contradictions` on the requirements in LF and in CRLF form, and asserts that both are empty. It fails first on the CRLF form on Linux.
- The hosted `Product / windows-latest` job on the PR head must pass.
- `fledge lanes run verify` passes on Linux.

## Where these lessons go

- `specs/cli/context.md`
