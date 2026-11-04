---
change: let-chough-session-new-accept-the-specsync-module-ids-that-init-accepts-such-as-chat-service-review-thread-4465229406
artifact: context
---

# Context

Review thread 4465229406 on #77 (opened by Leif's agent, reproduced by the review loop): `chough session new --module chat.service` is refused with "module names use letters, digits, hyphens, underscores" while `chough init --module chat.service` works. src/sessions.rs checked modules with `valid_name`, the session-key rule, instead of `records::valid_module_name`, the SpecSync 6.1.2 module-ID rule that init and the other commands adopted in #56.
