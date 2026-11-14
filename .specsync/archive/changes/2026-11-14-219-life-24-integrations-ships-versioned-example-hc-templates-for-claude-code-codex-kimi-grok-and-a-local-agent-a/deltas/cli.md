## ADDED

### REQUIREMENT REQ-cli-065

Example hc templates. The repository SHALL hold an optional `integrations/` folder with a README and, for each of `claude-code`, `codex`, `kimi`, `grok` and `local`, an hc template at `integrations/<host>/hc-template.md`. Each template:
- SHALL have a front matter with exactly `name`, `description` and `command`;
- SHALL have a body that tells the agent to run `chough next --environment local` and names one `chough init --host` value that `chough init` accepts;
- SHALL hold no absolute or home-folder path.

`integrations/templates.toml` SHALL list each template with its host, file, name, init host, status and an ordered list of its versions, each with the SHA-256 of the file at that version. The local template's status SHALL be `unqualified` until a local agent setup is qualified (#220); every other template's status SHALL be `supported`. The repository's tests SHALL fail while a template's bytes differ from its newest listed digest, while its versions do not run 1, 2, 3 in order, or while a digest repeats, so a changed template needs the next version appended with its digest. The tests do not detect an in-place edit of a listed entry: a change appends the next version and never edits a listed entry, and review checks that in the diff.

Chough SHALL never start hc, Herdr or Rune: no program Chough runs is one of them, and neither `src/` nor the shipped skill names Herdr or Rune. The README SHALL say that the folder is optional and that Chough has no runtime dependency on hc, Herdr or Rune, and that Rune is a separate Fledge plugin. Trace: LIFE-24, LIFE-22.

Acceptance Criteria
- Each of the five templates has exactly hc's three front-matter keys, runs `chough next --environment local`, names a `chough init --host` value that `chough init --help` lists and that equals its manifest entry, and holds no absolute or home-folder path.
- `integrations/templates.toml` lists exactly the five templates, each with an ordered versions list starting at 1. The local one is `unqualified` and the rest are `supported`. Each template's newest digest equals its file's SHA-256, so changing one byte fails the test until the next version is appended with the new digest. Versions out of order, or a repeated digest, also fail.
- No `Command::new` in `src/` starts hc, Herdr or Rune, and neither `src/` nor `skills/` names Herdr or Rune.
- The README names every template, says the folder is optional, and says Chough has no runtime dependency on hc, Herdr or Rune.
