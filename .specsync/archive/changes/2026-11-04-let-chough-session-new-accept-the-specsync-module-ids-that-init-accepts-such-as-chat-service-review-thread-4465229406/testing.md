---
change: let-chough-session-new-accept-the-specsync-module-ids-that-init-accepts-such-as-chat-service-review-thread-4465229406
artifact: testing
---

# Testing

`session_new_accepts_the_module_ids_init_accepts`: `session new` with `--module chat.service --module identity-wipe.service` succeeds and stores both; `a/b`, `..`, `trailing.` and ` lead` are refused with "invalid module name" and no session file. It fails before the fix with the old message.
