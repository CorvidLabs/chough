---
id: read-requirements-with-crlf-line-endings-in-the-librarian-default-test-131-windows
state: archived
type: bug_fix
base_commit: 217680e292733680b377daee33b37dcde14b9424
---

# Read requirements with CRLF line endings in the librarian-default test (131 Windows)

## Intent

Read requirements with CRLF line endings in the librarian-default test (131 Windows)

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- The librarian-default test reads the requirements correctly with LF and with CRLF line endings, checking both; it fails first on the CRLF form; the hosted Windows product job passes; fledge lanes run verify passes on Linux.

## No-spec Rationale

The #124 test split sentences after joining only LF lines, so a Windows CRLF checkout failed it; test repair only, no requirement text changes.
