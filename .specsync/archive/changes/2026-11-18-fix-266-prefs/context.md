---
change: fix-266-prefs
artifact: context
---

# Context

A follow-up to fix-266 (#330), from Leif's P2 review comment on #330: `prefs::pref` writes `.chough/preferences.json` directly, without the writer lock. Mid-merge, with `.chough/README.md` unmerged, `chough pref set librarian off --source …` exited 0 and changed `.chough/preferences.json`, while `session new` refused. That breaks REQ-cli-069's promise that every Chough-record writer refuses while a path under `.chough/` is unmerged, with no files changed.

Leif's answer, relayed by hc t4c on 2026-11-18, was "Fix first, you resolve": a small follow-up change on #330's branch in which `pref set` and `pref unset` take the writer lock, with a regression test that is red first. His P2 comment and that answer are this change's scope approval.

The other writers were checked:
- `journal add` takes the lock itself.
- Evidence, logs and the external-log index are written inside commands that already refuse.
- Skill files and the knowledge index are not under `.chough/`.
