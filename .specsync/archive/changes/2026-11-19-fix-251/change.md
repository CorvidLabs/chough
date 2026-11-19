---
id: fix-251
state: archived
type: bug_fix
base_commit: 66efea4dd777077ef9bbd412cf0fac8f7ea93b41
---

# Fix 251

## Intent

fix 251

## Affected Canonical Specs

- `records`
- `cli`

## Acceptance Criteria

- With source_dirs = ["."] in .specsync/config.toml, chough init prints advice that names it, says SpecSync refuses the record folders in verification_ignored_paths as overlapping and counts the installed skill bundle as uncovered source, and advises narrowing source_dirs to the code folders such as ["src"]; with a policy whose meaningful_paths holds "." too, chough doctor's records_in_verification finding names both settings, advises narrowing and keeping "." out of meaningful_paths, then listing the missing entries; once both are narrowed the advice is the list again. Neither file is ever written.

## No-spec Rationale

Not applicable
