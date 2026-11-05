---
change: make-the-knowledge-directory-an-obsidian-style-vault-yaml-front-matter-on-every-note-iso-dates-standard-links-and
artifact: context
---

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
