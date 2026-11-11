---
change: chough-0-6-0-human-first-records-one-markdown-file-with-yaml-front-matter-per-record-date-slug-session-folders-with-an
artifact: tasks
---

# Tasks

Before build: Leif's scope approval of this change (a human gate, not a task), and a
rebase onto main once #212 has merged, so this is the only open native change.

## 1. Red first

- [ ] Write the #165 regression in the 0.5.x layout (seven archived sessions, `knowledge/` neither tracked nor ignored, accept to done); it fails on 0.5.0.
- [ ] Write the #237 regression on 0.5.0 (a gate for release refuses build to test).

Record both failures with `chough cases run` and link each to a local bug (plain step).

## 2. Record layer

- [ ] Add specs/records (spec, requirements, context, tasks, testing) and move `src/identity.rs` and `tests/ids.rs` into its files list.
- [ ] `src/markdown.rs`: front matter subset reader and writer, section appends, bounds, CRLF, refusals naming file, line and rule.
- [ ] Date and slug names with the 6-hex clash suffix and aliases in `src/identity.rs`.
- [ ] `src/leaks.rs`: patterns, refuse and `--redact`, automatic redaction for failing lines, configured human names allowed.

## 3. Sessions and timeline

- [ ] `session.md` checkpoint and story sections; `checkpoint` flags write the story sections; `--resolve-question`.
- [ ] Timeline entries with entry IDs and structured fields, written in the same atomic replace; delete event files, cursors, adoption, history prune and checkpoint merge.
- [ ] Active pointer, lock for every record writer, `init --objective` creating the first session.
- [ ] `agent/` folder and inbox entries as Markdown.
- [ ] Finish into one story with receipts, the committed-bytes rule, screenshots beside the story; `session close`, `session shelve`, `session new` and `use` archiving finished sessions with the kept_open rules.
- [ ] Port tests/sessions.rs, tests/history.rs, tests/stages.rs, tests/stop.rs, tests/next.rs, tests/relay.rs and tests/common/mod.rs to the new layout.

## 4. Evidence and cases

- [ ] Evidence entries, local logs, key failing lines, external artifacts by neutral ID, `--screenshot` with metadata stripping and bounds.
- [ ] Freshness over entries and receipts; `cases record` and `evidence add` responses unchanged in shape.
- [ ] Port tests/evidence.rs, tests/cases.rs, tests/verification.rs and tests/evidence_inventory_compatibility.rs.

## 5. Bugs

- [ ] Markdown bugs with an append-only Log, aliases, receipts after archive; `bug merge` for `.chough/bugs/*.md`.
- [ ] Port tests/bugs.rs.

## 6. Gates and approval as the human

- [ ] `--holds` and `--drives` on gates; refusals by held stage (option A, unless Leif picks B at approval).
- [ ] `src/humans.rs`: identity configuration; `--close-gate`, `gate record` and `change ship` run approve or review as the configured human, refusing unknown identities.
- [ ] Port tests/gates.rs, tests/change.rs and tests/workflow_authority.rs; add tests/approval.rs with a SpecSync stub.

## 7. Actor and corrections

- [ ] Actor on timeline entries, evidence and bugs; fixture clears the five variables.
- [ ] `chough correct` for open sessions and archived stories; next, status and timing show corrections.

## 8. Knowledge

- [ ] `knowledge/index.md` generator linking every lesson, the journal as the timeline and `.specsync/archive/changes/`.
- [ ] `src/journal.rs`: post at done, `journal add`, Atom feed; `library index --check` covers both.
- [ ] Delete trims, the ledger and its rotation, attachments, roll-up, session, bug, archive, module, intent, rules and timeline notes, `library archive` and `library snapshot`, with their tests (tests/librarian_trim.rs, tests/reduce.rs and the retired parts of tests/vault.rs and tests/knowledge_default.rs), naming the retired requirement in each removal.

## 9. Timing

- [ ] Read timelines and receipts; remove lineage and orphan handling; port tests/timing.rs.

## 10. Migration and doctor

- [ ] `src/migrate.rs`: preview, apply, receipt, mapping, aliases, removed IDs, interrupted apply; fixtures for every 0.5.x shape.
- [ ] Fail-closed reader for every command but migrate, doctor, skill, help and version.
- [ ] Doctor findings: unmigrated, leak, unfinished archive, timeline rewritten, alias conflict; `--fix --redact`.
- [ ] tests/migrate.rs and tests/doctor.rs.

## 11. Docs and release preparation

- [ ] docs/cli.md, docs/architecture.md, docs/FRD.md, docs/lifecycle.md, docs/walkthrough.md, docs/DECISIONS.md, README.md, AGENTS.md, CLAUDE.md, CHANGELOG.md (0.6.0 section with the upgrade note).
- [ ] Skill bundle (SKILL.md, checkpoint, plan, lifecycle, accept, bugs, librarian, testing, stages references, cases template) and its installed copy; update the skill release manifest pin.
- [ ] `.gitattributes`, `.gitignore`, `fledge.toml` (`leaks` step in verify and ci), `.github/` workflow step; examples and exercises.
- [ ] specs: materialize the deltas; update `tests/spec_split.rs` pins for REQ-history-001 to 004 and REQ-reporting-001 naming this change; spec files lists for new and moved files.
- [ ] Cargo version 0.6.0.

## 12. Dogfood

- [ ] Migrate this repository with the 0.6.0 build, as its own commit with the receipt.

Then, as plain steps: drive this session with the 0.6.0 build from that commit on;
run and record every case in testing.md; `fledge lanes run verify` on macOS and
`fledge run linux-verify`; CI including archive authentication; Leif's acceptance
review; the move into done in 0.6.0; close #233, #210 and #208 as folded in and #211
as superseded by KNOW-30. Publication to crates.io waits for Leif's go.
