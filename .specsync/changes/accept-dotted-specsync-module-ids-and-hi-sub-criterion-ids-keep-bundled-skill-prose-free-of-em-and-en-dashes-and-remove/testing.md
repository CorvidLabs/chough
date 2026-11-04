---
change: accept-dotted-specsync-module-ids-and-hi-sub-criterion-ids-keep-bundled-skill-prose-free-of-em-and-en-dashes-and-remove
artifact: testing
---

# Testing

- `tests/ids.rs`: dotted modules through init, checkpoint, context, next, status and timing; a checkpoint holding `chat.service`, `room.service`, `linked-wallet.service`, `UX-12.a`, `CHAT-34.a` and `BOTS-4.g` read by every view; bug links to `UX-12.a` and `REQ-chat.service-1`; a `chat.service.cases.toml` listed and checked; IDs neither tool accepts still refused with nothing saved.
- `tests/cli.rs` or `tests/ids.rs`: `--remove-completed` removes an exact entry, corrects with `--completed` in one call, refuses a missing entry with the session byte-identical.
- `tests/prose.rs`: no U+2013 or U+2014 in any bundled skill file.
- Failing first: the ID and remove tests fail with `src/` at main 8298e5a; the prose test passes there because the dashes were already removed.
- Proof on Raven at 74e43005: `chough next`, `status`, `timing` read Raven's real checkpoint instead of refusing it.
- `fledge lanes run verify` passes.
