---
id: fix-339
state: implementing
type: bug_fix
base_commit: af8ca8901b2a23affcd40ac74865d99637448346
---

# Fix 339

## Intent

fix 339

## Affected Canonical Specs

- `records`
- `cli`

## Acceptance Criteria

- A conflict quoted inside a fenced code block in a record is not a marker, so the record saves. A lone ======= with no <<<<<<< or >>>>>>> around it is never refused; chough doctor reports it as an advisory possible_conflict_separator finding, 'possible leftover conflict separator at line N', that does not change its exit status. A committed case definition holding markers gives doctor conflict_markers findings, and cases check names the file, line and marker. The migration preview says the apply refuses and the apply refuses, each naming the 0.5.x source file first, with nothing written.

## No-spec Rationale

Not applicable
