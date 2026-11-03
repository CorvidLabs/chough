## MODIFIED

### REQUIREMENT REQ-cli-016

Host skill installation. The helper SHALL install the complete skill bundle once for each distinct supported host directory requested, defaulting to `.agents/skills`, with the claude host sharing `.agents/skills`; SHALL report each installed file as current, edited, known-outdated or missing; SHALL update only missing files and files byte-identical to a known earlier bundle; SHALL report an installed legacy `.claude/skills/chough-workflow/` copy as legacy without writing or deleting it during init, status or update; and SHALL remove legacy files only through an explicit remove-legacy command that requires the shared copy, removes only files byte-identical to the current or a known earlier bundle and keeps every other file. Trace: FLOW-1, LIFE-4, LIFE-12, LIFE-13, LIFE-14.

Acceptance Criteria
- Installing for the claude host, alone or together with openai, creates only `.agents/skills/chough-workflow/` with the complete bundle and no `.claude/` directory, and repeating the install preserves edits.
- A bundle file byte-identical to one from a known earlier build is updated, while an edited file is reported and left unchanged.
- An installed `.claude/skills/chough-workflow/` copy is reported as legacy by status and update and stays byte-identical; remove-legacy refuses with nothing removed while the shared copy is missing, and otherwise removes current and known-outdated legacy files and keeps edited or unknown ones.
