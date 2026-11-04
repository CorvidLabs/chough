---
id: name-the-change-in-the-bounded-read-test-s-specsync-stand-in-reply-review-thread-on-71
state: implementing
type: bug_fix
base_commit: 2d4ba2592011b3be5f5b8f5cc54aad95f3309ed3
---

# Name the change in the bounded-read test's SpecSync stand-in reply (review thread on #71)

## Intent

Name the change in the bounded-read test's SpecSync stand-in reply (review thread on #71)

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- native_reads_end_by_their_deadline_and_retain_bounded_output passes with its assertions unchanged, its checkpoint stand-in reply naming bounded-change.

## No-spec Rationale

Test fixture only: the bounded-read test's stand-in reply named no change ID, which the #71 review-thread fix now refuses by design; the assertions are unchanged.
