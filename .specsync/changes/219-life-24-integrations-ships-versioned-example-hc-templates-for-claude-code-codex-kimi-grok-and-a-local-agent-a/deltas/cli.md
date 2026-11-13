## ADDED

### REQUIREMENT REQ-cli-065

Example hc templates. The repository SHALL hold an optional `integrations/` folder with a README and, for each of `claude-code`, `codex`, `kimi`, `grok` and `local`, an hc template at `integrations/<host>/hc-template.md`. Each template:
- SHALL have a front matter with exactly `name`, `description` and `command`;
- SHALL have a body that tells the agent to run `chough next --environment local` and names one `chough init --host` value that `chough init` accepts;
- SHALL hold no absolute or home-folder path.

`integrations/templates.toml` SHALL list each template with its host, file, name, version, init host, status and the SHA-256 of its file. The local template's status SHALL be `unqualified` until a local agent setup is qualified (#220); every other template's status SHALL be `supported`. A template whose bytes differ from its listed digest SHALL fail the repository's tests until its digest and version are updated.

Chough SHALL never start hc, Herdr or Rune: no program Chough runs is one of them, and neither `src/` nor the shipped skill names Herdr or Rune. The README SHALL say that the folder is optional and that Chough has no runtime dependency on hc, Herdr or Rune, and that Rune is a separate Fledge plugin. Trace: LIFE-24, LIFE-22.

Acceptance Criteria
- Each of the five templates has exactly hc's three front-matter keys, runs `chough next --environment local`, names a `chough init --host` value that `chough init --help` lists and that equals its manifest entry, and holds no absolute or home-folder path.
- `integrations/templates.toml` lists exactly the five templates, each with a version of at least 1. The local one is `unqualified` and the rest are `supported`. Each digest equals its file's SHA-256, so changing one byte fails the test.
- No `Command::new` in `src/` starts hc, Herdr or Rune, and neither `src/` nor `skills/` names Herdr or Rune.
- The README names every template, says the folder is optional, and says Chough has no runtime dependency on hc, Herdr or Rune.
