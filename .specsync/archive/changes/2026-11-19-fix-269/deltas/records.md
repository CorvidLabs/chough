# records delta

## ADDED

(none)

## MODIFIED

### REQUIREMENT REQ-records-004

The session record. An open session's `session.md` front matter SHALL be its checkpoint and SHALL give `kind: session`, `id`, `aliases`, `objective`, `intent_scope`, `modules`, `stage`, `stage_status`, `iteration`, `callers`, `native_change`, `pr`, `criteria`, `requirements`, `cases`, `design`, `files`, `observations`, `bugs`, `blockers`, `waiting_on_human` (each gate with `gate`, `holds` and `opened`, REQ-cli-030), `run_consents` and `next`. Its body SHALL be the story, in these sections in this order: `# <title>` (the objective), `## Goal`, `## Where it stands`, `## Decisions and why`, `## What was verified`, `## Open threads` and `## Timeline` (REQ-history-002), whose entries carry the fields that requirement lists and the reason a gate was withdrawn (REQ-history-012). A decision recorded with `chough checkpoint --decision` SHALL be one list item appended to Decisions and why, naming who decided when the decision is a human's (REQ-cli-050) and linking the gate's timeline entry; completed work SHALL be appended to Where it stands; an open question SHALL be one list item in Open threads, resolved by appending its answer to that item's line through `chough checkpoint --resolve-question`; an evidence entry referenced by the session SHALL be one list item in What was verified that links its file. These sections SHALL be the only place the session keeps decisions, completed work and open questions, and `chough next` and `chough status` SHALL read them there (REQ-cli-021), so no field repeats them. Chough SHALL write the Goal from the objective when the session is created and SHALL NOT rewrite the Goal or Where it stands paragraphs afterwards, with one exception: while the `# <title>` line or the Goal still holds, untouched, the placeholder objective `chough session new` writes when none is given, the next save after the objective is set SHALL fill each such one with the objective, once, so the story and its journal post carry the objective and never the placeholder; a title or Goal a person wrote SHALL never be rewritten. Trace: DRIVE-24, DRIVE-25, DRIVE-26, FLOW-13.

Acceptance Criteria
- `chough session new` writes a `session.md` whose front matter carries every listed key and whose body has the seven parts in order with the Goal taken from the objective.
- A decision, a completed item, an open question and an evidence reference each appear once, in their section, and `chough next` shows the same decision and question from a fresh process.
- Resolving an open question keeps its line and appends the answer, and `chough next` no longer lists it as open.
- A person's rewrite of the Goal paragraph is kept after a decision and a stage move.
- A session started without an objective, given one with `chough checkpoint --objective`, has that objective as its title and Goal; a later objective change rewrites neither, its journal post and archived story carry the objective and not the placeholder, and a Goal a person wrote over the placeholder is kept byte for byte.

## REMOVED

(none)
