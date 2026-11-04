---
change: keep-the-checkpoint-writer-lock-in-the-project-so-every-temp-directory-shares-it-name-the-file-being-written-when-busy
artifact: plan
---

# Plan

## Approach

Move the checkpoint writer lock from the system temp directory into the project, at
`.chough/local/checkpoint.lock` (Git-ignored), so every process working on a project
shares it whatever its `TMPDIR`; name the file being written in the busy refusal; and
specify the serialized writers as REQ-cli-040 under DRIVE-11.

## Out of scope

Which commands lock, the 30 s wait and the refusal behavior (unchanged); the inbox add
lock (a separate finding); Windows-specific lock semantics beyond fs4's; any HI wording
other than DRIVE-11.

## Steps

1. Failing-first tests: different-`TMPDIR` writers serialize (tests/cli.rs); the busy
   refusal names the session file in sessions mode (unit test, 300 ms wait); a new adopt-race
   test asserts saved-or-refused per writer (tests/cli.rs).
2. `checkpoint_lock_path` returns `.chough/local/checkpoint.lock`, creating
   `.chough/local/.gitignore` (`*`) create-only; drop the temp-directory digest.
3. The busy refusal names the file from `sessions::checkpoint_path` (src/sessions.rs is
   unchanged, so the history module's contract is not touched).
4. Update tests/common/mod.rs's lock path helper and the exercise script's cleanup.
5. Docs, `specsync change check --commit`, Linux verify, cases, PR stacked on #77.

## Risks

An older Chough on the same project still locks in the temp directory and does not
serialize with a new one (CHANGELOG says so); creating `.chough/local/` in a legacy
project adds an ignored directory (no tracked change).

## Constraints consulted

DRIVE-11 and DRIVE-3; REQ-cli-037's resolve-once rule; #70's lock design (released on
exit, read-only commands take none); specs/cli/context.md lessons on deterministic
records.
