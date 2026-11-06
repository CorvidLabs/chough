---
id: native-check-ac2-only-a-verifying-change-with-handoff-readiness-safe-shows-no-note-126-d10
state: draft
type: bug_fix
base_commit: ec830cfb001cc5f71f4f83809ca3294d685f42a4
---

# Native check AC2: only a verifying change with handoff readiness safe shows no note (126, D10)

## Intent

Native check AC2: only a verifying change with handoff readiness safe shows no note (126, D10)

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- REQ-cli-039 AC2 says a verifying change shows no note only with handoff readiness safe, and any other or missing readiness keeps a note, matching its body and last criterion; every REQ-cli-039 criterion clause that names the verifying state names its readiness; next text and JSON give approved and implementing changes the same exact not-passed note naming specsync change check <id>; the runtime is unchanged. The new wording test fails first and fledge lanes run verify passes on Linux.

## No-spec Rationale

Not applicable
