---
change: recognize-pristine-skills-from-pinned-released-bundles-without-replacing-edits
artifact: requirements
---

# Requirements

## Modified REQ-cli-016


Host skill installation. The helper SHALL install the complete skill bundle once for each distinct supported host directory requested, defaulting to `.agents/skills`, with the claude host sharing `.agents/skills`; SHALL report each installed file as current, edited, known-outdated or missing; SHALL update only missing files and files byte-identical to a known earlier bundle; SHALL report an installed legacy `.claude/skills/chough-workflow/` copy as legacy without writing or deleting it during init, status or update; and SHALL remove legacy files only through an explicit remove-legacy command that requires the shared copy, removes only files byte-identical to the current or a known earlier bundle and keeps every other file. Trace: FLOW-1, LIFE-4, LIFE-12, LIFE-13, LIFE-14.

Acceptance Criteria
- Installing for the claude host, alone or together with openai, creates only `.agents/skills/chough-workflow/` with the complete bundle and no `.claude/` directory, and repeating the install preserves edits.
- A bundle file byte-identical to one from a known earlier build is updated, while an edited file is reported and left unchanged.
- An installed `.claude/skills/chough-workflow/` copy is reported as legacy by status and update and stays byte-identical; remove-legacy refuses with nothing removed while the shared copy is missing, and otherwise removes current and known-outdated legacy files and keeps edited or unknown ones.

- For the exact source bundle bytes pinned to v0.1.0 c03d41956ef1d9f8380f02034f03d42bf52bc174, v0.2.0 edbc3c1adecebf6a1378a731965c3bfe8c8217df, v0.3.0 4d65542ff583ee2478f7d98c1dd4434a960e9dc1, v0.4.0 69950c7a9253230aeeee3bac0d0d6b580e291aad and v0.5.0 455cb643cd91a9c804afdf5566e3889abf1bde49, each of the24 manifest-named files reads current or known-outdated and skill update refreshes its pristine copy. This criterion covers those pinned bundles only, not unlisted future release contents.
- Recognition matches both the manifest file name and exact byte SHA256; user-edited bytes, including one-byte/newline changes, stay edited and retain exact bytes and modes after update. Host sharing/routing, missing/current behavior and explicit legacy handling above remain unchanged.

No new requirement or human-intent ID is allocated. Existing full normative wording and original acceptance criteria are preserved verbatim before the narrow pinned-release additions.
