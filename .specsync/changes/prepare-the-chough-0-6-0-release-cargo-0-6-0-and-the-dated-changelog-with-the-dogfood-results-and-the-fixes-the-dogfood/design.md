---
change: prepare-the-chough-0-6-0-release-cargo-0-6-0-and-the-dated-changelog-with-the-dogfood-results-and-the-fixes-the-dogfood
artifact: design
---

# Design

D1: the active-session pointer resolves through aliases. `sessions::pointed` returns
the pointer's session ID: the pointer itself when a session folder or story has that
ID, otherwise the one open session whose alias or bare key it is. `active()` and
doctor's `stale_pointer` check both use it, so a pointer an older layout or a
`session use <alias>` left keeps naming its session, and a pointer naming nothing
is still `stale_pointer`. The pointer file is machine-local and is not rewritten.

D2: the gate summary's base is the change's own `base_commit`, unless the merge base
of HEAD with the default branch is not an ancestor of it (default-branch work merged or
rebased in after the change began, #190), or the change records none; a merge base that
is HEAD itself is never used, and with no usable base the summary says so. One more Git
call, no native read.

The migrate preview: when a native change is open, the plan says up front that
`--apply` will refuse, naming each open change. The preview still writes nothing,
and the apply's refusal is unchanged.

The journal: the session record gains `title` (one line, at most 72 bytes) and
`summary` (a list of lines, at most 900 bytes), set by `chough checkpoint --title` and
`--summary` and kept in the story. The post at the move into done uses them; without
them it derives a title from the objective's first clause and a body from the story's
Goal, first decision, newest verified entry and first open thread. `chough journal add
--record <story>` writes an archived story's post with the agent's title and body,
dated by the story's `closed` and timed by its latest Timeline entry on that date, and
refuses a story that already has a post. The backfill runs it once per story, with a
title and summary the agent writes from reading that story. 0.6.0's own post is
removed with Git (Chough never removes a post) and written again with `--record`.

Lessons are notes under `knowledge/lessons/` as REQ-cli-044 already defines them
(`id`, `title`, `type: lesson`, `tags`, `date`, `applies_to`, `source`), each from
recorded evidence; no new command.

Leaks: doctor's scan skips a system-path match under `/tmp/` or `/private/tmp/` in a
`.chough/cases/*.cases.toml` file. Every other kind still counts there, and every other
file is scanned as before.

Everything else is configuration, evidence or text: the Cargo version, the CHANGELOG
section, a test helper's name, and case links under `.chough/cases/`.
