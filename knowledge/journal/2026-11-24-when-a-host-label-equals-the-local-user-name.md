---
id: 2026-11-24-when-a-host-label-equals-the-local-user-name
title: When a host label equals the local user name
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-24
time: 2026-11-24T03:10:28Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-23-leak-host-equals-user.md
  - .specsync/archive/changes/2026-11-24-a-host-label-equal-to-the-local-user-name-does-not-count-as-a-host-name-leak-as-a-plain-path-component-while-urls-user/
record: .chough/archive/sessions/2026-11-23-leak-host-equals-user.md
actor:
  - terminal: t14z
---
# When a host label equals the local user name

When a host label equals the local user name, as this Mac's 'Tob' for user 'tob', the host-name rule must not re-flag the plain folder the user-name rules deliberately let through; REQ-records-006 gains that exception

Why: Intent review (product, QA, engineering, architecture passes, 2026-11-23): the exception is path-component context only, so a URL to the bare label, <user>@<host> and scp-style hosts still refuse; the dotted and .local names keep their existing rules. QA edge accepted: an absolute path whose first component is that label is also exempt, consistent with the user rule exempting system-root first components.

Verified: cargo test: full suite 1040 passed, 0 failed; new unit test a_host_label_equal_to_the_user_name_is_not_a_leak_as_a_path_component red on main, green with the fix; the built binary's doctor --leaks on…

Read more: [the story](../../.chough/archive/sessions/2026-11-23-leak-host-equals-user.md) · [the archived change](../../.specsync/archive/changes/2026-11-24-a-host-label-equal-to-the-local-user-name-does-not-count-as-a-host-name-leak-as-a-plain-path-component-while-urls-user/)
