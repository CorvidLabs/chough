---
change: a-host-label-equal-to-the-local-user-name-does-not-count-as-a-host-name-leak-as-a-plain-path-component-while-urls-user
artifact: tasks
---

# Tasks

- [x] Failing tests first, each red on main: with a host label equal to the user name,
      a path component is not refused and `chough doctor` reports nothing, while the
      same name in a URL, as `<user>@<host>` and as an scp-style host is still refused.
- [x] `src/leaks.rs`: the host rule skips a label that equals the local user name
      as a path component only; the name check keeps its rule, as the requirement
      scopes the exception to path components.
- [x] The changelog fragment `changelog.d/2026-11-23-leak-host-equals-user.md`.

Human gates (not checkboxes): scope approval, implementation review, merge.
