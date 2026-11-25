---
id: fix-441
state: verifying
type: bug_fix
base_commit: 0d5988f804155e7b3c285cace6cc0c52f540e035
---

# Fix 438

## Intent

fix 438

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- A refused grouped reply leaves every named Chough gate pending and the persisted session and Timeline byte-identical. Exercise the 4096-entry Timeline and 1048576-byte session boundaries.
- A successful grouped reply closes each selected gate once with the exact words and the correct list and summary digest. Unselected gates stay open. A safe retry after a refusal can close the whole group.
- A selection spanning two sessions returns the documented other-session refusal and leaves both session files byte-identical.
- For a valid group with one native-driving gate, check all local closures before running the native step. Capacity refusal runs no native step. Native refusal closes no Chough gate. A retry does not duplicate a completed native action.
- Record earlier direct native writes in order before the grouped closes, settle them once after a successful save, and preserve the single-gate checkpoint behavior.

## No-spec Rationale

REQ-cli-061 already requires that a grouped gate record closes all selected gates or none. Restore that behavior and document the native boundary without changing the requirement.
