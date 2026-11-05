# Lesson bundle: make-the-knowledge-directory-an-obsidian-style-vault-yaml-front-matter-on-every-note-iso-dates-standard-links-and

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Make the knowledge directory an Obsidian-style vault: YAML front matter on every note, ISO dates, standard links, and module, intent and rules notes that connect each archived change to what it touched
- **Kind**: Feature
- **Specs**: cli
- **Paths**: src/library.rs, src/vault.rs, src/main.rs, src/skill.rs, tests/vault.rs, tests/evidence.rs, tests/next.rs, tests/optional.rs, tests/knowledge_default.rs, tests/librarian_trim.rs, hi/knowledge.md, INTENT.md, skills/chough-workflow/references/librarian.md, docs/cli.md, docs/FRD.md, CHANGELOG.md
- **Acceptance**: Every generated knowledge note starts with YAML front matter (id, title, type, tags, and for an archived change status, ISO 8601 finalized date, modules, requirements, HI criteria traced by those requirements and commits). Archived-change notes link their modules and criteria. modules/<module>.md links the module spec, requirements, context and lesson count and the archived changes that touched it; intent/<family>.md links the HI file and, per criterion ID, the archived changes whose requirements trace it, without copying criterion text; rules.md links the rules, skills and decisions that exist. Every relative Markdown link in the vault resolves, regeneration is byte-identical, and no gate reads it. KNOW-14 to KNOW-19 are captured in hi. New tests fail first and fledge lanes run verify passes on Linux.

## Evidence

- Verification commit: `a3fe73e2e0320ad8127ecb263abc6190e7cf2943`
- Base commit: `8fbbbe71951f9145b91313c7cbde6524d709734a`
- Verified by: `specsync check --spec cli --spec history`

## From the change's context.md

# Context

After #106 the knowledge directory exists and stays current, but its notes are plain
Markdown. They have no front matter, give dates as Unix seconds and link only from the
index to entries. Leif asked on 2026-11-04 for "the Obsidian kind of like vault header,
front matter and linking". He chose "Front matter + MD links" over [[wikilinks]], so the
vault reads in Obsidian and on GitHub alike, and said: "we need to learn, grow, rules,
skills, and compound knowledge of all this project and all things.. spec-sync lessons,
hi.. changes.. etc all things!!"

This change captures his choices as KNOW-14 to KNOW-19. It implements KNOW-14 (the
format) and KNOW-15 (compounding links). KNOW-16 to KNOW-19, the wider reduction of
`.chough/` and the attachments, come in a later change, because they remove proof and
need their own safety review.

## From the change's design.md

# Design

- **Front matter:** `vault::front_matter` writes `key: value` lines in a fixed order.
  Strings are JSON-quoted, lists are YAML block lists, and empty lists are written as
  `[]`.
- **Archived-change note:** `type: archived-change`, `status`, `finalized`, `modules`,
  `requirements`, `criteria`, the three commits and tags
  (`chough/archive`, `module/<m>`). The body adds Modules and Criteria lines that link
  `../modules/<m>.md` and `../intent/<file>.md`.
- **Traced criteria:** `vault::traces` parses each `specs/<m>/requirements.md`
  `### REQ-` block for its `Trace:` list. An archive's criteria are the sorted union
  over its requirements.
- **Module, intent and rules notes:**
  - `modules/<m>.md`, for each directory under `specs/`, links the spec, requirements,
    context and lessons (counting date-led entries under `## Lessons`) and the archived
    changes that touched the module.
  - `intent/<file>.md`, for each `hi/*.md` except `AGENTS.md` and `CLAUDE.md`, lists
    criterion IDs only, each with the archived changes that trace it.
  - `rules.md` lists the rule, skill, decision and policy files that exist.
- **Index and README:** INDEX.md gains front matter and links the rules, module and
  intent notes. The README template gains front matter, and this repository's README is
  regenerated from it.

## From the change's testing.md

# Testing

tests/vault.rs, with a fixture of two archives over cli and history, requirements with
`Trace:` lines, two HI files, a lesson source, AGENTS.md and docs/DECISIONS.md:
- Front matter on every note.
- Archived-change properties and links.
- Module notes.
- Intent notes, with no criterion text copied.
- The rules note and the index links.
- Every relative link resolves.
- Byte-identical regeneration. This one already held before the change and guards it.

The other six fail first. Cases bind all seven, and `fledge lanes run verify` runs on
Linux.

## Where these lessons go

- `specs/cli/context.md`
