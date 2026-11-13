---
change: 0-6-1-record-polish-a-bug-s-actual-line-stands-apart-migrated-front-matter-names-the-new-entries-and-a-fresh-project-s
artifact: context
---

# Context

Chough 0.6.0's fourth fresh-eyes round (t38n) found three record polish items, deferred to
0.6.1 by Leif:
- F8: a bug body renders "**What happened:** <symptom>" with "Actual: <a>" on the next
  line, which Markdown joins into one paragraph.
- F9: migrated records keep 0.5.x observation IDs in front matter while the body links
  the new file.
- F11: a fresh project's `.chough/README.md` lists `artifacts/` as "kept from Chough
  0.5.x", and the README's init paragraph carries a 10-line parenthetical about hosts.

On 2026-11-13T02:03Z Leif chose "Yes, start (Recommended)" for 0.6.1 as six small PRs;
this is one. Bugs: `.chough/bugs/2026-11-12-a-bug-body-runs-its-symptom-and-its-actual-line-into-one.md`,
`.chough/bugs/2026-11-12-migrated-records-keep-old-observation-ids-in-their-front.md` and
`.chough/bugs/2026-11-12-a-fresh-project-s-records-readme-names-artifacts-as-kept-from.md`.
