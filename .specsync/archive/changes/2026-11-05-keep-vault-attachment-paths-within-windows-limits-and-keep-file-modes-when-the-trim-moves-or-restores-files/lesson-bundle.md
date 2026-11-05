# Lesson bundle: keep-vault-attachment-paths-within-windows-limits-and-keep-file-modes-when-the-trim-moves-or-restores-files

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Keep vault attachment paths within Windows limits and keep file modes when the trim moves or restores files
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: src/attachments.rs, src/evidence.rs, src/library.rs, tests/reduce.rs, docs/cli.md, docs/FRD.md, CHANGELOG.md
- **Acceptance**: Vault attachments of an archive whose ID has more than 24 characters go under a stable folder of at most 25 characters, and no attachment path passes 220 characters (a key item that would stays, protected with that reason), so the repository checks out on Windows. Moved and restored files keep their permissions. The new tests fail first, Windows CI checks the branch out, fledge lanes run verify passes on Linux, and the trim reruns on this repository at closeout.

## Evidence

- Verification commit: `4a7ca2b80729e9bb26004093644a96d3f529aa40`
- Base commit: `8a91c5c491321e1ddd5575e99412459c7bdec7f4`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

Three defects in #109, found by CI and by corvid-agent's review at 6e0a835:

1. **Windows checkout.** The closeout trim moved 165 key items to `knowledge/attachments/<archive id>/<path under .chough/>`. This repository's archive IDs run to about 120 characters, so attachment paths reached 246, and Product / windows-latest failed in checkout ("unable to create file knowledge/attachments/claude-host-installs-…"). A tracked path the tool generates must stay short enough to clone.
2. **File modes.** A refused apply restores each removed file with create-new, which gives it the default mode. The refused first closeout trim (76fb917) turned two executable scripts into 0644. The vault copies of key items use `fs::write` and lose the mode the same way. KNOW-10 says removed files are restored, and a restore that changes the mode is not faithful.
3. **CI identity.** tests/reduce.rs `commit_aged` runs `git commit` without the identity the fixture's other Git calls pass, so it fails on CI runners with no global Git identity.

Leif chose "Yes, record and merge" for #109 and its fixes, and "Widen KNOW-19" for corvid-agent's wording point (2026-11-05). This branch's trim was undone in 8a91c5c, so the next move into done reruns it under the fixed layout.

## From the change's design.md

# Design

- `attachments::attachment_folder(id)` names an archive's folder. It is the ID when the ID has at most 24 characters. Otherwise it is the first 16 characters, cut back to before the last hyphen among them, then `-` and 8 hex digits of the ID's SHA-256. It is at most 25 characters and stable. `attachment`, `attachments_of` and the archive note's links (src/library.rs) all use it, so moves, notes and lookups agree. Short IDs keep their readable folder.
- The worst case in this repository is 22 (`knowledge/attachments/`) + 25 + 1 + 161 (the longest path under `.chough/`) = 209 characters. `MAX_ATTACHMENT_PATH` is 220, which leaves 39 characters for the folder a clone sits in under Windows' 260-character limit. `summary_plan` takes the knowledge directory: a candidate with any planned move longer than that is reported protected with the reason, and nothing of it moves or is removed. Nothing is lost, and the next run sees the same.
- The apply reads each file's permissions before removing it, sets them on the vault copy after writing it, and sets them on a restored file after the create-new write. The `held` list carries them.
- `commit_aged` passes the same `user.name`, `user.email` and `commit.gpgsign` settings as the fixture's `git` helper.

## From the change's testing.md

# Testing

Each new test fails before the fix:

- tests/reduce.rs `key_items_of_a_long_archive_id_move_under_a_short_folder`: cold a1 renamed to a 61-character ID; its screenshot moves under `a1-a-long-<8 hex>/` and the note links it there.
- tests/reduce.rs `a_key_item_whose_attachment_path_is_too_long_stays`: a failure-first log whose attachment path would be 244 characters stays, and `artifact:x` is reported protected with the 220-character reason.
- tests/reduce.rs `a_moved_key_item_keeps_its_mode`: an executable failure-first script is 0755 in the vault.
- tests/reduce.rs `a_refused_apply_restores_files_with_their_modes`: a SpecSync stand-in reports changed validity once an executable loose script is gone; the refused apply restores it as 0755 with its bytes.
- src/attachments.rs `long_archive_ids_get_a_short_stable_folder`: short IDs are kept; long ones are cut at a hyphen (or at 16 characters) with the expected digest.

`commit_aged` gets the fixture's Git identity, so `loose_artifacts_of_a_cold_change_are_trimmed_and_current_or_bug_held_ones_stay` runs on CI runners without one. `fledge lanes run verify` runs on Linux, and the trim reruns on this repository at closeout. Windows CI must then check the branch out.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-cli-046 | tests/reduce.rs `key_items_of_a_long_archive_id_move_under_a_short_folder`, `a_key_item_whose_attachment_path_is_too_long_stays`, `a_moved_key_item_keeps_its_mode`, `a_refused_apply_restores_files_with_their_modes`; src/attachments.rs `long_archive_ids_get_a_short_stable_folder` |

## Where these lessons go

- `specs/cli/context.md`
