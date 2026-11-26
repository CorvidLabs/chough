---
change: fix-435
artifact: plan
---

# Plan

## Approach

Rewrite `publish_entry` (src/sessions.rs) to write the entry whole or not
at all without a hard link, by delegating to
`crate::records::write_new_bytes(root, file, content.as_bytes())` after
the existing INBOX_LIMIT check. Keep the `Ok(false)`-when-exists
contract that callers rely on (write_new_bytes returns false when the
path already exists). Update the function's doc comment. Extend
tests/no_hard_links.rs so the ignored exFAT test also runs `chough inbox
add` and asserts the entry file exists and parses.

## Out of scope

The other writers (already fixed by #260/#424), the inbox limit, entry
format, and any contract text.

## Steps

1. Change publish_entry to use write_new_bytes; adjust its doc comment.
2. Extend the ignored exFAT test with inbox add.
3. Changelog fragment changelog.d/2026-11-23-fix-435.md.
4. Run the regular session/inbox tests plus fmt and lint; the exFAT test
   runs by hand with -- --ignored when a mount is safe.

## Risks

write_aside_then_place writes its temporary in `.chough/local/`, not the
inbox directory; the inbox directory may not exist yet for the first
entry (write_new_bytes creates parents). An `Unsupported` path writes
in place with create-new, which preserves never-replace.

## Constraints consulted

REQ-records-001 (records whole or not at all), the #313 revert note in
tests/no_hard_links.rs, AGENTS.md trailing-newline rule.