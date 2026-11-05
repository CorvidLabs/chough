---
change: keep-the-checkpoint-writer-lock-in-the-project-so-every-temp-directory-shares-it-name-the-file-being-written-when-busy
artifact: context
---

# Context

Issue https://github.com/CorvidLabs/chough/issues/83 collects follow-ups from the Chough
bot approval of #77 (this branch is stacked on #77 at 75a732d; it retargets to main when
#77 merges):

- a. The checkpoint writer lock from #70 lives in the system temp directory, keyed on
  the canonical `.chough` path, so processes with different `TMPDIR` values, or a temp
  cleaner deleting it while held, each get their own lock; a reviewer's repro kept 11 of
  20 concurrent updates.
- b. When busy, the refusal names `.chough/session.json` even in sessions mode, where
  the write goes to `.chough/sessions/<key>.json`.
- c. The serialized read-modify-write and the legacy writer's refusal after `session
  adopt` are not literally specified; the wording had to come from Leif through HI.
- d. `legacy_writers_racing_adopt_end_in_one_mode` asserts only that one mode remains.

Leif chose on 2026-11-04, from offered options, to keep the lock in `.chough/local/`
(Git-ignored, one per worktree) and confirmed the wording captured as DRIVE-11. The
Git common directory and keeping the temp directory were offered and not chosen.

During the first build, two #77 tests failed: `a_project_without_sessions_is_unchanged`
asserts `.chough/local/` never appears in a project without sessions, and an evidence
retention snapshot spans a `consent grant`. Leif chose on 2026-11-04 ("Lock everywhere,
relax #77") to keep the lock in `.chough/local/` in every project: #77's test checks no
tracked or session file changes instead, and a new REQ-history-007 states what
`.chough/local/` holds. Sessions-only and the Git directory were offered and not chosen.
So this change covers the cli and history modules.

Authority: Leif's 2026-11-04 standing approval for bounded Chough bug fixes, his brief to
fix issues 81 to 88 one PR each, and his #83 choice.
