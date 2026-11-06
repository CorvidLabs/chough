## ADDED

### REQUIREMENT REQ-cli-056

Projects in a subdirectory. When the project root is inside a Git repository but is not its top level, the evidence snapshot SHALL cover the project's own files, under the project root, and SHALL take the repository's HEAD as its revision. The snapshot is the one `cases run`, `cases record`, `evidence add`, `validate run`, `status` and `next` record or compare. A working-tree change outside the project SHALL NOT make the snapshot dirty or change its fingerprints. The same project SHALL have the same content fingerprint at the top level and in a subdirectory, and at the top level the snapshot SHALL be unchanged. A command run there SHALL NOT refuse with advice to use the repository root. Trace: DRIVE-18.

Acceptance Criteria
- In a project in `app/` of its repository, `cases run` and `evidence add` succeed and record the repository's HEAD, and `status` reports a known snapshot.
- An edit outside the project leaves the snapshot clean with unchanged fingerprints; an edit inside it makes the snapshot dirty and changes its content fingerprint.
- The same project has the same content fingerprint at the repository root and in a subdirectory.
