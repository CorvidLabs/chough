---
change: accept-dotted-specsync-module-ids-and-hi-sub-criterion-ids-keep-bundled-skill-prose-free-of-em-and-en-dashes-and-remove
artifact: design
---

# Design

- `records::valid_module_name` ports SpecSync 6.1.2 `validate_module_name`: one portable path segment, no separator, Windows-invalid or control character, no `.`/`..`, no edge space, no trailing dot, no reserved device basename, at most 247 bytes. Used for checkpoint and init modules, case file stems and the owning-module hint.
- `records::valid_criterion_id` ports hi 0.8.0 `Id::parse`: family `A-Z` then `A-Z0-9_`, then dot-separated levels alternating number and lowercase letters, starting with a number, no leading zero, 32-bit numbers. Used for checkpoint criteria and bug criteria.
- `records::valid_reference_id` is `valid_name` plus single inner dots, for requirement and case IDs such as `REQ-chat.service-1` and `TC-chat.service-sends-hybrid`. Observation, bug, run and consent IDs keep `valid_name`.
- `checkpoint --remove-completed <text>` (repeatable) removes one exact entry before additions are applied, so remove plus `--completed` corrects a typo in one call; an entry that is not present refuses the whole call and nothing is saved.
- `tests/prose.rs` walks `skills/chough-workflow` and fails on U+2013 or U+2014.
