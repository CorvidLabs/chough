---
change: make-the-knowledge-directory-an-obsidian-style-vault-yaml-front-matter-on-every-note-iso-dates-standard-links-and
artifact: design
---

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
