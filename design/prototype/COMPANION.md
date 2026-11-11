# Chough records prototype companion

Canonical prototype: `index.html` in this folder. It's written without `<html>`/`<head>`
wrappers so it can also be published as a private Artifact page for review:
https://claude.ai/artifact/1gaHcJYeiULDxWtKMGUZim (same file). Earlier versions are in
Git history; there is no other current version.

Status: approved on 2026-11-10 by Leif ("Approve the design", in the coordinator session)

## Source of the decisions

Leif's 2026-11-10 interview, recorded on chough #235 (body plus two comments), #236 and
#234, and the earlier #14/#165/#167/#201/#208 decisions. hi criteria are not captured yet:
they go in during the intent pass, using Leif's words, before plan.

## Mappings

| Screen or state | Decided by Leif | Proposal (to confirm) | Issue |
|---|---|---|---|
| One-source-of-truth table | hi/ intent only; specs/ contracts; .specsync/ lifecycle; .chough/ sessions, decisions, bugs, evidence; fledge tasks; everything else links | — | #235 |
| `.chough/README.md` | Humans and agents are both first-class readers | A one-screen guide at the top of .chough/ | #235 |
| Active session folder and `session.md` | One unique date + slug folder per session, committed for handoff; active and archive like SpecSync; the story holds goal and outcome, decisions and why, what was verified, open threads | The checkpoint lives in the session's frontmatter, and the timeline is an append-only list in the same file, replacing session.json and transitions.jsonl | #214, #235 |
| Waiting-on-human state | Gates are open and easy: answer in chat or with the Chough CLI | The wording and placement of the gate banner | #235, #236 |
| Evidence receipt | A one-line claim plus a receipt (command, result, digest); raw logs in CI; date + slug names | Evidence lives inside the session that produced it | #235 |
| External evidence, 'not checkable here' | No path, file name or username in committed records; the real path stays in git-ignored local/ | — | #208, #234 |
| Bug file | Markdown with frontmatter, one file each, the source of truth; date + slug | An append-only log section | #235 |
| Archived session | Finished sessions move whole into archive/ | Stories stay as written, plus a closing line | #214 |
| `cases/*.cases.toml` | — | Case definitions stay small TOML (configuration) | — |
| `local/` | Machine-local facts never committed | Raw logs may also sit here locally | #208, #234 |
| `knowledge/index.md` | The vault holds only what has no other home and links to the rest | A short reading guide for newcomers | #235 |
| Journal posts and feed | Organic, automatic, editable posts when something ships, intent or a spec changes, or something breaks or is learned; one per repo; posts plus a generated feed; the approved style | — | #235 |
| `knowledge/lessons/` | Lessons with no other home | A module's lessons stay with its spec; only lessons that span projects live here | #235 |
| Removed items | `knowledge/intent/` and `knowledge/archives/` are deleted; no JSON twins; no sequential IDs | — | #235 |

## Open questions for Leif

- The proposals marked amber in the prototype (listed above).
- Gate UX: what "open and easy to use" looks like beyond the banner.
- Migration of existing BUG-0001-style IDs: aliases kept so old links work (proposed).

## Rendered review

| Date | Viewport | States checked | Method | Observation | Limits |
|---|---|---|---|---|---|
| 2026-11-10 | — | — | not yet run | — | Rendered review is pending Leif's first read; no browser QA recorded yet |
