---
id: make-the-knowledge-directory-an-obsidian-style-vault-yaml-front-matter-on-every-note-iso-dates-standard-links-and
state: archived
type: feature
base_commit: 8fbbbe71951f9145b91313c7cbde6524d709734a
---

# Make the knowledge directory an Obsidian-style vault: YAML front matter on every note, ISO dates, standard links, and module, intent and rules notes that connect each archived change to what it touched

## Intent

Make the knowledge directory an Obsidian-style vault: YAML front matter on every note, ISO dates, standard links, and module, intent and rules notes that connect each archived change to what it touched

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- Every generated knowledge note starts with YAML front matter (id, title, type, tags, and for an archived change status, ISO 8601 finalized date, modules, requirements, HI criteria traced by those requirements and commits). Archived-change notes link their modules and criteria. modules/<module>.md links the module spec, requirements, context and lesson count and the archived changes that touched it; intent/<family>.md links the HI file and, per criterion ID, the archived changes whose requirements trace it, without copying criterion text; rules.md links the rules, skills and decisions that exist. Every relative Markdown link in the vault resolves, regeneration is byte-identical, and no gate reads it. KNOW-14 to KNOW-19 are captured in hi. New tests fail first and fledge lanes run verify passes on Linux.

## No-spec Rationale

Not applicable
