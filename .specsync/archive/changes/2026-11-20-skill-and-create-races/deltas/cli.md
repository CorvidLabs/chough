# cli delta

## ADDED

(none)

## MODIFIED

### REQUIREMENT REQ-cli-016

Host skill installation. The helper SHALL install the complete skill bundle once for each distinct supported host directory requested, defaulting to `.agents/skills`, with the claude host sharing `.agents/skills`; SHALL report each installed file as current, edited, known-outdated or missing; SHALL update only missing files and files byte-identical to a known earlier bundle, where a copy whose bytes equal a bundle file's once each CRLF line break is read as LF counts as that bundle file, current or known-outdated, and `chough skill update` SHALL write its replacement with CRLF line breaks, so a checkout that converts line endings is still recognised and updated (#373); SHALL report an installed legacy `.claude/skills/chough-workflow/` copy as legacy without writing or deleting it during init, status or update; and SHALL remove legacy files only through an explicit remove-legacy command that requires the shared copy, removes only files byte-identical to the current or a known earlier bundle and keeps every other file. While an installed skill file is byte-identical to a known earlier bundle, `chough doctor` SHALL report it as `skill_outdated`, naming the file and `chough skill update`, with no fix of its own, since doctor never writes the skill, and `chough next` SHALL show one line, ``Skill files are outdated: run `chough skill update` ``, and give the count as `skill_outdated`; neither SHALL report a current file, a missing file or a file a person edited. A host's skill directory or a skill file that cannot be inspected, such as one reached through a symbolic link, SHALL be skipped; doctor SHALL report such a directory as the advisory finding `skill_uninspected`, and it SHALL NOT hide an outdated file elsewhere. `chough skill status` and `chough skill update` without `--host` SHALL skip such a directory too, report it as `uninspected` with its directory relative to the project and why, and report and update every other host, exiting 0; given `--host` for that host they SHALL refuse, naming the host and that directory, never an absolute path (#403). Several `chough skill update` runs at once SHALL each succeed: a missing bundle file that another run writes first counts as added when its bytes are the bundle's, and is preserved and reported as edited otherwise (#386); a file it cannot write SHALL be refused naming the file, relative to the project, and the reason, never internal assertion text (#373). Trace: FLOW-1, LIFE-4, LIFE-12, LIFE-13, LIFE-14.

Acceptance Criteria
- Installing for the claude host, alone or together with openai, creates only `.agents/skills/chough-workflow/` with the complete bundle and no `.claude/` directory, and repeating the install preserves edits.
- A bundle file byte-identical to one from a known earlier build is updated, while an edited file is reported and left unchanged.
- An installed `.claude/skills/chough-workflow/` copy is reported as legacy by status and update and stays byte-identical; remove-legacy refuses with nothing removed while the shared copy is missing, and otherwise removes current and known-outdated legacy files and keeps edited or unknown ones.

- For the exact source bundle bytes pinned to v0.1.0 c03d41956ef1d9f8380f02034f03d42bf52bc174, v0.2.0 edbc3c1adecebf6a1378a731965c3bfe8c8217df, v0.3.0 4d65542ff583ee2478f7d98c1dd4434a960e9dc1, v0.4.0 69950c7a9253230aeeee3bac0d0d6b580e291aad and v0.5.0 455cb643cd91a9c804afdf5566e3889abf1bde49, each of the24 manifest-named files reads current or known-outdated and skill update refreshes its pristine copy. This criterion covers those pinned bundles only, not unlisted future release contents.
- Recognition matches both the manifest file name and exact byte SHA256; user-edited bytes, including a one-byte change and a line break added or removed, stay edited and retain exact bytes and modes after update, while a copy whose only difference is CRLF line breaks reads as the bundle file it equals. Host sharing/routing, missing/current behavior and explicit legacy handling above remain unchanged.
- A copy of each bundle file with CRLF line breaks reads current or known-outdated as its LF bytes do, and `chough skill update` refreshes a known-outdated one with CRLF line breaks (#373).
- With `.codex/skills` a symbolic link to `../.agents/skills`, plain `chough skill status` and `chough skill update` exit 0, report the codex directory `uninspected`, naming `.codex/skills` and why, and update an outdated `.agents/skills` copy; `chough skill update --host codex` refuses naming the host and `.codex/skills`; no output holds an absolute path (#403).
- Eight `chough skill update` runs at once over missing bundle files all exit 0, every file ends current, and no output holds `Condition failed` or `File exists`; a missing file in a folder that cannot be written is refused naming the file (#386, #373).

### REQUIREMENT REQ-cli-001

The helper SHALL initialize a resumable session and discoverable workflow skill
without replacing existing files. It SHALL write neither of SpecSync's files, `.specsync/config.toml` and `.specsync/sdd.json`, and SHALL print the advice of REQ-records-014 while they leave Chough's records in SpecSync's verification, the advice to narrow a whole-repository `source_dirs` included. Several `chough init` runs at once in one project SHALL each succeed or refuse only as a run on its own would: a file another run wrote first is kept and reported preserved, as one a person made is, and an error SHALL name the file it concerns, relative to the project (#386). Trace: LIFE-4, LIFE-6.

Acceptance: repeated init preserves edited session and skill bytes; missing bundle
files can be added without overwriting neighbors. A session whose files were
edited since init is left byte-identical on a later init, and init never removes
or rewrites a file the user or a previous run created.

Acceptance Criteria
- repeated init preserves edited session and skill bytes
- missing bundle files can be added without overwriting neighbors
- a session whose files were edited since init stays byte-identical on later init
- init in a project whose SpecSync `source_dirs` is `["."]` leaves `.specsync/config.toml` byte-identical and prints the narrowing advice of REQ-records-014
- Six `chough init --objective` runs at once in a fresh repository each exit 0, or refuse only because a session is already open, naming `chough session new`; none fails on a file another run wrote, and no output holds `File exists` (#386).

## REMOVED

(none)
