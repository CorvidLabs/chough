---
change: one-librarian-default-in-the-requirements-req-cli-013-defers-to-req-cli-015-on-124-d8
artifact: context
---

# Context

Issue #124, audit finding D8. The audit was of main `700dad96` and published 0.3.0.

Two current requirements disagree on the librarian's default:
- REQ-cli-013 says preferences default to "ask, ask, on-request, capture-only and off", which makes the librarian off.
- REQ-cli-015 says "The librarian SHALL be on by default".

`chough pref show` reports `on`, with origin `default`, in a fresh directory. That matches REQ-cli-015 and the walkthrough, and `standing_preferences_need_a_source_and_stay_separate_from_consent` asserts it. No passing test can satisfy both requirements.

## Leif's plan

The runtime default stays on (95% plan, item C2, 2026-11-05). This is a contract repair. It is not authorization to change behavior.
