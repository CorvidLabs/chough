# Lesson bundle: show-the-latest-failed-native-check-in-next-and-say-when-it-has-not-passed-for-this-tree-issue-86

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Show the latest failed native check in next and say when it has not passed for this tree (issue 86)
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: src/native_check.rs, src/main.rs, tests/native_check.rs, docs/cli.md, docs/FRD.md, CHANGELOG.md
- **Acceptance**: After a failed native attempt next shows its time and commit and failing commands with exit codes and a later passing attempt replaces it. With the change approved or implementing next says the native check has not passed for this tree and that SpecSync keeps no reason for refusals and names specsync change check. A malformed attempts record is unknown and next makes no extra native read. Each new test fails on main 91a1fdd and fledge lanes run verify passes on Linux.

## Evidence

- Verification commit: `ac7047b8935cb51e05629174ba116d4783023406`
- Base commit: `91a1fdd82a4e314e5037c6256f17199ef0d604f0`
- Verified by: `specsync check --spec cli`

## From the change's context.md

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

## From the change's design.md

# Design

- New `src/native_check.rs`: `view(root, change, state)` reads
  `.specsync/changes/<change>/verification-attempts.json` read-only within the native
  record bound (no native command). It returns `null` when no native change is recorded
  or its workspace is not under `.specsync/changes/` (an archived change), otherwise
  `{"last_attempt", "not_passed_for_tree", "note", "unknown"}`:
  - `last_attempt`: the attempt with the greatest `timestamp` (ties: the later one in
    the file), as `passed`, `at_unix_seconds`, `commit` (12 characters) and
    `failed_commands` (`{"total", "items": [{"command", "exit_code"}]}`, at most 3,
    commands cut to 512 bytes); `null` when there is no attempts file.
  - `not_passed_for_tree`: true when the native state `next` already read is
    `approved` or `implementing`, or `verifying` with a `handoff.readiness` other than
    `safe` (the tree moved after the last check; #91 review); a `verifying` reply with no
    readiness gives an "unknown" note instead; `note` then says SpecSync keeps no reason when it
    refuses before verifying and names `specsync change check <id>`.
  - `unknown`: the reason when the file cannot be read, is oversized or malformed;
    `last_attempt` is then `null`, never "no failure".
- src/main.rs: the next view adds `native_check`; the text form prints a
  `Native check:` line (failed with time, short commit, commands and exit codes;
  passed with time and commit; no attempt; or unknown) and the note when not passed.

## From the change's testing.md

# Testing

- tests/native_check.rs, with stub `hi` and `specsync`:
  - `a_failed_native_attempt_is_shown_in_next`: a failed attempt (two failing
    commands and one passing) is shown in text and JSON with its time, commit, failing
    commands and exit codes; a later passing attempt replaces it.
  - `next_says_when_the_native_check_has_not_passed_for_this_tree`: at state
    `implementing` with no attempts file, `next` says so, that SpecSync keeps no reason
    for refusals, and names `specsync change check <id>`; at `verifying` it does not.
  - `an_unreadable_attempts_record_is_unknown_and_adds_no_native_read`: a malformed
    file is reported as unknown, and `next` runs the stub `specsync` once either way.
- #91 review: `a_verifying_change_whose_tree_moved_has_not_passed_for_this_tree`,
  `a_verifying_change_without_readiness_reports_freshness_unknown` and
  `a_failed_attempt_without_a_failing_command_says_so` fail at 0b4c5d7's src; the original
  `verifying` assertion now uses a reply with readiness `safe`.
- Each new test fails on main 91a1fdd; `fledge lanes run verify` passes on Linux.

## Where these lessons go

- `specs/cli/context.md`
