---
change: show-the-latest-failed-native-check-in-next-and-say-when-it-has-not-passed-for-this-tree-issue-86
artifact: context
---

# Context

Issue https://github.com/CorvidLabs/chough/issues/86 (field test 2026-11-03, chough
a33c747, SpecSync 6.1.2): when `specsync change check <id>` fails, for example with
"tasks.md contains incomplete tasks" or "no requirement evidence for REQ-...",
`chough next` shows `Last failure: none` and "Continue build...", so the reason has to
be found again by rerunning the check.

Probed on SpecSync 6.1.2 (2026-11-04, throwaway clone of this repo):

- A failing verification command (here `specsync check --spec cli` on a missing source
  file) appends an attempt with `passed: false` and each command with `success` and
  `exit_code` to `.specsync/changes/<id>/verification-attempts.json`; no reason text.
- A refusal before verification starts (incomplete tasks, missing requirement
  evidence: both of the issue's examples) records nothing: no attempt, no state change.

So Chough can show recorded failed attempts but cannot know a refusal's reason. Leif
chose on 2026-11-04 ("Show + honest gap + upstream"): show the latest failed attempt,
say plainly when the check has not passed for this tree and that SpecSync keeps no
reason for refusals, name `specsync change check <id>`, and file a spec-sync issue
asking SpecSync to persist refusals with their reason. Running the check from Chough
was offered and not chosen (too slow).

Criteria: LIFE-7 ("I can see missing or stale evidence...") and LIFE-3.
Authority: Leif's 2026-11-04 standing approval for bounded Chough bug fixes and his
brief to fix issues 81 to 88 one PR each.
