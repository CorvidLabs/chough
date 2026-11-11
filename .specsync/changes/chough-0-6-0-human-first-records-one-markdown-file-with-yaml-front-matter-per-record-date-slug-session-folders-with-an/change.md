---
id: chough-0-6-0-human-first-records-one-markdown-file-with-yaml-front-matter-per-record-date-slug-session-folders-with-an
state: draft
type: feature
base_commit: 250148ce1480ac4eaf4cb9682e8558c0d2a789f8
---

# Chough 0.6.0 human-first records: one Markdown file with YAML front matter per record, date+slug session folders with an append-only timeline that fold into one archived story at finish, a knowledge vault that links and never copies, no local facts or credentials in committed records, a human's gate approval drives the SpecSync approval as that human, and chough migrate converts 0.5.x projects with a digest receipt

## Intent

Chough 0.6.0 human-first records: one Markdown file with YAML front matter per record, date+slug session folders with an append-only timeline that fold into one archived story at finish, a knowledge vault that links and never copies, no local facts or credentials in committed records, a human's gate approval drives the SpecSync approval as that human, and chough migrate converts 0.5.x projects with a digest receipt

## Affected Canonical Specs

- `cli`
- `history`
- `reporting`
- `records`

## Acceptance Criteria

- Records (DRIVE-25, DRIVE-26): every session, evidence entry and bug Chough writes is one Markdown file with YAML front matter, decisions are entries in their session's file, and no JSON twin exists. After chough migrate --apply on a 0.5.x fixture and on this repository, .chough/ holds no session.json, transitions.jsonl, history/, sessions/*.json, evidence/*.json or bugs/*.json, and every command works from the Markdown files alone
- Sessions (DRIVE-24, DRIVE-25): each session is .chough/sessions/<date-slug>/session.md, whose front matter carries the checkpoint and whose append-only Timeline section carries every stage move, gate and correction, with an agent/ folder for working files. Two branches that advance different sessions merge with no conflict. At finish the session becomes one .chough/archive/sessions/<date-slug>.md story with its evidence receipts folded into What was verified, and its folder, agent/ included, leaves the tree
- Names (DRIVE-27): new records are named YYYY-MM-DD-<slug>, never by a sequence number; a same-day clash gets a short suffix at creation; every 0.5.x ID keeps resolving as an alias in links, bug commands, observation, case and checkpoint references
- Guides (DRIVE-28): .chough/README.md and knowledge/index.md orient a new reader, and case definitions stay TOML with their format unchanged
- Vault (KNOW-30 to KNOW-33): knowledge/ holds only index.md, journal/ posts with a generated feed.xml, lessons/ for lessons that span projects, and pre-existing library/ and briefs/ content as is. knowledge/intent/, knowledge/archives/, attachments and generated session, bug and module notes are gone, no Chough step copies or moves a tracked record into the vault, the move into done writes a short journal post that links the archived story, and library index --check passes
- No leaks (LIFE-26, FLOW-22, issues 234 and 208): a write whose text holds a home path, the local user or host name, a token, key or credential is refused, or redacted with --redact, naming the field and the kind but never the value. External evidence is recorded by a neutral id and its fingerprint, with its path only in the git-ignored .chough/local/. chough doctor and the verify lane scan committed records and fail on any finding
- Approval as the human (FLOW-23, issue 236): closing a scope or acceptance gate with a human's decision records the matching SpecSync approval or review as that human, whose identity comes from configuration or the approving channel, with their exact words, where and when, and the relaying agent. With no identity configured for that human it refuses rather than guess, and an agent acting on its own records as itself
- Who and corrections (LIFE-17, LIFE-18, PRs 233 and 210): each timeline entry, evidence entry and bug records the acting agent, model and hc terminal when known and never fails when they are unknown; chough correct appends a correction linked to the original entry, also on an archived story, and next and status show the corrected text
- Data loss regression (issue 165): with earlier archived sessions present and knowledge/ not committed, advancing accept to done leaves every earlier archived session file byte-identical in place and adds only the new story. The 0.5.x reproduction is recorded failing first and linked to a local bug
- Migration (DRIVE-27, LIFE-20): chough migrate previews by default and writes nothing; --apply writes the 0.6.0 tree create-new only, never replacing a file, removes the 0.5.x files only after every new file is written and read back, and writes a receipt with the digest of every file before and after. A second apply reports nothing to do, the 0.6.0 reader refuses an unmigrated tree with refused:unmigrated naming chough migrate, and chough doctor lists what is left
- Gates (issue 237): an open human gate holds only the stage it names, so a gate for a later decision such as a release no longer refuses unrelated stage moves, and chough next still lists every open gate
- Dogfood: this repository is migrated with the 0.6.0 build, this change's own session continues through test, accept and done in the new format, and fmt, lint, the full tests, the cases this change adds and fledge lanes run verify pass on macOS and Linux; no other repository changes

## No-spec Rationale

Not applicable
