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
- [ ] Write the failing-first regressions of testing.md's bug table that can fail on 0.5.0: #190, #76, BUG-0020, BUG-0022, BUG-0023, the vaulted adopter and the timing budget; the #12 untracked-receipt and write-after-check cases in the 0.5.x layout without the exclusion; run the #12 closeout and the #132 stress case on 0.5.0 and keep what they show.

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

- [ ] `src/migrate.rs`: preview with its plan digest, apply only that plan, receipt, mapping, aliases, removed IDs, redaction of every rewritten file, leak findings for files kept byte for byte, the archive validity check with restore-and-refuse, interrupted apply.
- [ ] Migration fixtures captured from every release, 0.1.0 to 0.5.0, each built from its tag (#213), plus the synthetic shapes in plan.md.
- [ ] Fail-closed reader for every command but migrate, doctor, skill, help and version.
- [ ] Doctor findings: unmigrated, leak, unfinished archive, timeline rewritten, alias conflict; `--fix --redact`.
- [ ] tests/migrate.rs and tests/doctor.rs.

## 10b. The remaining bug fixes

- [ ] #12 (REQ-records-014): only Chough's records under `.chough/sessions/`, `archive/` and `bugs/`; the `records_in_verification` doctor finding, the `chough init` advice, the migration follow-up and the `chough next` wording; the #1045 and #212 regression cases, the case-definition control and the real native closeout test, with SpecSync at the CI pin; the accept and lifecycle references say when Chough writes are safe and keep the documented order for projects without the entries, with their doc pins updated.
- [ ] #190: gate summaries count from the merge base, leaving out Chough's records, the knowledge directory and SpecSync bookkeeping.
- [ ] #76 and BUG-0023: the next-action wording at init, test and accept, and the summary first in `status`.
- [ ] BUG-0001 and BUG-0022: the config comment in the version agreement check; the `toolchain` task first in the verify lane.
- [ ] BUG-0020: the delta guard compares with the effective, digest-matching approval.
- [ ] #148: every reader of open changes skips an empty change directory.
- [ ] #132 and the macOS lock flake: the holder's process ID in busy refusals, a bounded wait in the in-process lock test, per-child wait reporting, the stress case.
- [ ] Timing: the native budget starts at the first native read; the trace test under the stress run.

## 10c. Retired criteria and the case map

- [ ] `hi retire` every KNOW criterion #212 marks superseded (planned: KNOW-2, 7, 8, 9, 10, 11, 13, 16, 17, 18, 19, 20, 21, 22, 23, 25, 26, 27, 28 and 29), each in the commit that removes its requirements and cases; reword KNOW-24 to the approved wording (every kind of file has a stated rule, finished stories stay for good, working files go when the work finishes, machine-only files are never committed); then `hi index` and `hi check`.
- [ ] Apply plan.md's case map: remove 102 cases with their behaviour, re-link 3, rewrite 71 and port 128, naming the requirement or criterion in each commit; `fledge run cases` passes.

## 11. Docs and release preparation

- [ ] docs/cli.md, docs/architecture.md, docs/FRD.md, docs/lifecycle.md, docs/walkthrough.md, docs/ci.md (the leak scan and the receipt size bound), docs/integrations.md and docs/FRD.md platform rows (Windows best-effort through WSL), docs/diagrams (regenerate), docs/exercises/handoff-cold-start/seed.sh, README.md, AGENTS.md, CLAUDE.md.
- [ ] CHANGELOG.md: a 0.6.0 section that says to upgrade every agent before a project migrates, since 0.5.x cannot read 0.6.0 records (and 0.5.0 already cannot read main's evidence inventory field), and lists removed and new commands.
- [ ] docs/DECISIONS.md: stop adding D-numbers; keep it as an index of past decisions linking where each lives now, and mark D-010 (session.json as the only checkpoint) as replaced.
- [ ] One roadmap: merge docs/ROADMAP.md into docs/roadmap-1.0.md, link the delivered history, update it for the one-change 0.6.0 shape. Remove docs/NEXT_CHANGE.md (addressed; Git and D-018 hold it).
- [ ] Flakes live as bug files: replace any per-flake notes file under docs/ from #212's test plan with links to the bug files (#217), and add the migration fixtures (Q7) to docs/test-plan-1.0.md.
- [ ] One home for cli lessons: specs/cli/cli.3md stays the lesson source (REQ-cli-002); move the one lesson only it holds into place and make specs/cli/context.md link to it instead of copying.
- [ ] Skill bundle and its installed copy: SKILL.md (resume reads session.md and .chough/README.md), references checkpoint, plan, accept and lifecycle (a decision at a gate is the human's; approval or review on authority given in advance names the agent as `<agent>-<terminal>` and quotes the human, a refused agent actor is reported and never retried as the human, FLOW-24), bugs, librarian, testing, validate, design and stages, templates bug-report.md, cases.toml and prototype/COMPANION.md; the closeout's two commits (story, then `doctor --fix`); update the skill release manifest pin.
- [ ] `.gitattributes`, `.gitignore`, `fledge.toml` (`leaks` step in verify and ci), `.github/` workflow step; examples and exercises.
- [ ] specs: materialize the deltas; update `tests/spec_split.rs` pins for REQ-history-001 to 004 and REQ-reporting-001 naming this change; spec files lists for new and moved files.
- [ ] Cargo version 0.6.0.

## 12. Dogfood

- [ ] Migrate this repository with the 0.6.0 build, as its own commit with the receipt, adding `.chough/sessions/`, `.chough/archive/` and `.chough/bugs/` to `verification_ignored_paths` in `.specsync/sdd.json` in the same commit.
- [ ] Close BUG-0002 and BUG-0003 as `fixed-upstream`, each note citing t37b's check on spec-sync main 46f55652 (SpecSync 6.2.0).

Then, as plain steps (the release gate in plan.md applies): drive this session with the 0.6.0 build from that commit on;
run and record every case in testing.md; `fledge lanes run verify` on macOS and
`fledge run linux-verify`; CI including archive authentication; Leif's acceptance
review; the move into done in 0.6.0; close #233, #210 and #208 as folded in, #211
as superseded by KNOW-30, and #237, #165, #12, #190, #76, #14 and #132 and the
nine local bugs as their cases show (a flake that never reproduced stays open with
its diagnostics, saying so). No 0.6.0 tag or release until the dogfood is complete and `chough bug list` shows zero open Chough bugs; a flake that will not reproduce goes to Leif, never closed to clear the gate. Publication waits for Leif's go. After release: refresh the shared skill copy the hc agents load with `chough skill update`.
