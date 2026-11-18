---
id: register-the-previous-stages-digest-and-qualify-session-use-reads
state: archived
type: bug_fix
base_commit: ab0d421149f12d9ad2984c833cfe1d9346d3989e
---

# Register the previous stages digest and qualify session-use reads

## Intent

Register the previous stages digest and qualify session-use reads

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- A pristine copy of the workflow stages file from main 0fd9fd22 is known-outdated and chough skill update replaces it with the current bundle. An edited copy of that file stays byte-identical. docs/architecture.md says an exact open session ID is returned without reading another session, a bare key or alias reads each open session, and a name that matches none also reads archived stories.

## No-spec Rationale

FLOW-18 already requires the docs and skill to say what current main does. REQ-cli-016 already updates a file byte-identical to a known earlier bundle and leaves an edited copy. This change registers main 0fd9fd22's stages.md digest and qualifies one architecture sentence. No requirement text changes.
