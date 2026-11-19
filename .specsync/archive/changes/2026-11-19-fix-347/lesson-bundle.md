# Lesson bundle: fix-347

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Fix 347
- **Kind**: BugFix
- **Specs**: cli, history
- **Paths**: src/bugs.rs, src/doctor.rs, tests/bugs.rs, docs/cli.md, specs/cli/requirements.md, specs/cli/cli.spec.md, specs/history/requirements.md, specs/history/history.spec.md, .chough/cases/cli.bug-evidence-links.cases.toml, changelog.d/2026-11-18-fix-347.md
- **Acceptance**: A new bug's Log names each piece of linked evidence as a Markdown link relative to the bug's file that opens the open session's entry, bug update and the automatic failure capture do the same, and the front matter keeps the stored form. After that session is archived, chough doctor reports the bug as stale_bug_links, naming the entry and the story, and --fix appends one Log entry linking the story with every earlier Log line unchanged; the bug is then not reported again.

## Evidence

- Verification commit: `49734b4bffa346e4656f4e566f2ecd36fb4dd9a7`
- Base commit: `dbd0b1f7c772ae17e4b0ff59bd92a7024bfffe2b`
- Verified by: `specsync check --spec cli --spec history`

## From the change's context.md

# Context

GitHub #347, two follow-ups from the tester's check of #341 (#277). Leif chose
both.

1. A new bug's Log named its evidence as `<session>/<evidence>` with no link,
   while a migrated bug's Log links it since #341. Every bug should read the
   same.
2. A link to an open session's evidence entry goes stale when that session is
   archived: the folder is removed and the entry lives on as a receipt of the
   story. The issue allows either repointing at archive time or having
   `chough doctor` report and fix stale links, and asks for the smaller honest
   one.

For 2 this change takes the doctor route. A bug's Log is append-only
(REQ-cli-012), so an earlier line's link cannot be rewritten. `chough doctor`
reports the bug as `stale_bug_links`, and `--fix` appends one Log entry that
links the archived story. Archiving itself is unchanged: it still touches no
record but the session's own. The cost is that the link is stale until doctor
runs.

The front matter keeps the stored `<session>/<id>` form, which already resolves
to the story's receipt after archiving.

## From the change's design.md

# Design

Nothing visual changes. A new bug's Log reads:

```text
- 2026-11-18T10:00:00Z · Reported (manual report); evidence: [2026-11-18-probe-failure](../sessions/2026-11-18-probe/evidence/2026-11-18-probe-failure.md)
```

After the session is archived and `chough doctor --fix` runs, one entry is added:

```text
- 2026-11-19T09:00:00Z · Evidence now in its archived session: [2026-11-18-probe-failure](../archive/sessions/2026-11-18-probe.md)
```

## From the change's testing.md

# Testing

Two new tests in `tests/bugs.rs`, both failing on main:

- `a_new_bugs_log_links_its_evidence_relative_to_the_bug`: `bug add --observation`
  writes the Reported line with a link to the entry, the link opens a file,
  `bug update --observation` links what it adds, and the front matter keeps the
  stored form with no link in it.
- `doctor_relinks_a_bugs_evidence_after_its_session_is_archived`: after the
  session is closed the entry file is gone; doctor reports one `stale_bug_links`
  finding for the bug, naming the entry and the story, not a decision; `--fix`
  appends one Log entry linking the story, every earlier Log line is unchanged
  and in order, the story file exists, and the bug is not reported again.

`doctor_relinks_only_what_the_archived_story_holds_as_a_receipt` (added after the
tester's findings on #360): a note that links an entry never recorded, and a
hand-edited link whose entry name holds a credential, are not relinked; only the
entry the story holds as a receipt is, and the credential is never written again.
It fails without the receipt check.

`a_bug_too_near_its_bound_gets_advice_and_no_relink_fix` (added after the swarm's
finding 62-F9 on #360): a bug 60 bytes below its bound gets a finding whose fix
starts `none:` and names the story, and `--fix` leaves it byte for byte. It fails
without the fit check, where the finding offers `chough doctor --fix` for ever.

`a_qualified_id_of_archived_evidence_links_the_story` (added after the reviewer's
finding on #360): after a session is archived, its entry given by the qualified
`<session>/<entry>` ID to `bug add` and `bug update` links the story, as the bare ID
does. It fails with the earlier code, which linked the removed session folder.

Two existing tests that pinned the bare `<session>/<id>` text in a Log line now
expect the link. The link text is built from evidence names the bug already
holds, so no new free-text input is added.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-cli-012 | the first test and its case |
| REQ-history-012 | the second test and its case |

## Where these lessons go

- `specs/cli/cli.3md`
- `specs/history/context.md`
