## ADDED

### REQUIREMENT REQ-cli-041

Trimming at done. When `chough stage advance` moves into done and its save succeeds, and the librarian is on (the default) and the `retention` preference is neither `manual` nor a recorded `off` (the unset default `off` does not stop it), the helper SHALL apply the post-archive summary of REQ-cli-033 with the recorded retention authority, or else with the librarian default as the standing preference (KNOW-13), keeping its rules: the ledger is written first, a removal that changes an affected archive's terminal evidence validity is restored and refused (KNOW-10), and nothing is removed while any native change is open. It SHALL report the result as `applied.librarian_trim` with status `trimmed`, `skipped` (librarian off, `retention manual` or a recorded `retention off`) or `refused` with its reason, and a refusal or failure SHALL NOT undo or refuse the move into done. `retention` SHALL accept `manual` beside `off` and `auto`. Trace: KNOW-13, KNOW-11, KNOW-7, KNOW-5.

Acceptance Criteria
- With defaults, `stage advance` into done reports `trimmed` and prunes the cold archived proof that `library archive` would, with a ledger entry whose authority names KNOW-13.
- With `retention` set to `manual`, a recorded `retention off`, or the librarian off, it reports `skipped` and removes nothing.
- With a native change open, it reports `refused` naming it, removes nothing, and done is still recorded.
