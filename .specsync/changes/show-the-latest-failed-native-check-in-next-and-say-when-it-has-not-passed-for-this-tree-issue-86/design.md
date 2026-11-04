---
change: show-the-latest-failed-native-check-in-next-and-say-when-it-has-not-passed-for-this-tree-issue-86
artifact: design
---

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
    `approved` or `implementing`; `note` then says SpecSync keeps no reason when it
    refuses before verifying and names `specsync change check <id>`.
  - `unknown`: the reason when the file cannot be read, is oversized or malformed;
    `last_attempt` is then `null`, never "no failure".
- src/main.rs: the next view adds `native_check`; the text form prints a
  `Native check:` line (failed with time, short commit, commands and exit codes;
  passed with time and commit; no attempt; or unknown) and the note when not passed.
