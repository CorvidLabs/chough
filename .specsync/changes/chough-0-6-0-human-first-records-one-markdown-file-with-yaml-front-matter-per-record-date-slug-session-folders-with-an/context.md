---
change: chough-0-6-0-human-first-records-one-markdown-file-with-yaml-front-matter-per-record-date-slug-session-folders-with-an
artifact: context
---

# Context

## Proposal, in plain words

Chough 0.6.0 makes Chough's own records readable by people, not only by agents.
Every piece of work, bug and test result becomes one Markdown file, with a short
header that agents read and plain sentences that people read, named by its date and
a short title instead of a number, so parallel branches stop fighting over IDs. Each
piece of work gets its own folder while it is in progress, and when it finishes it
becomes one short story file that stays in the archive for good. The knowledge
folder stops copying things and only links to them, plus a short journal post and a
feed for each finished piece of work. Committed records never hold local paths,
user names or credentials: logs go to CI, the record keeps the command, the key
failing lines and a fingerprint, and a few small screenshots can be committed beside
a story. When Leif approves or accepts at a Chough gate, Chough records the matching
SpecSync approval in Leif's name, from an identity in configuration rather than one
built into the code. A one-time `chough migrate` converts an existing project, keeps
a receipt of every file before and after, and removes nothing Git does not still
hold. It also closes a real data-loss bug where finishing one session deleted an
older archived one. Everything ships together as Chough 0.6.0 and is tried on the
chough repository before any other project changes.

The same release fixes every open Chough bug: a pending human decision no longer
holds up unrelated steps, Chough's records and SpecSync's checks stop making each
other look out of date, review summaries count only the change's own files, `chough
next` guides a new agent from start to acceptance and `chough status` leads with a
short summary, version notes and local tools are checked against CI, a leftover
empty folder no longer looks like open work, and the intermittent test failures get
fixes or, where they will not reproduce, the diagnostics to catch them. Two bugs that
were really SpecSync's are already fixed there, and are closed as fixed upstream.

## What led here

- Leif's 2026-11-10 interview and design review (issues #235, #234, #236, #214),
  captured as hi criteria in #212: DRIVE-24 to DRIVE-28, KNOW-30 to KNOW-33, FLOW-21
  to FLOW-23, LIFE-17 to LIFE-26. Intent approved by Leif ("Approve it"); the design
  prototype approved by Leif ("Approve the design"). Both are recorded on this
  session as relayed decisions.
- Three safeguards of those nine criteria are kept here as spec rules, as the
  coordinator asked: only committed bytes ever leave the tree (REQ-records-012, from
  KNOW-7), `knowledge/index.md` links every lesson, per module and cross-project
  (REQ-cli-043, from KNOW-25), and the journal and feed are the timeline of
  archived changes (REQ-records-011, REQ-cli-043, from KNOW-28).
- Leif widened 0.6.0 to fix every open Chough bug (coordinator chat, 2026-11-10: he
  picked all four groups offered and added "all bugs?"). testing.md lists each with
  its fix and regression case. BUG-0002 and BUG-0003 describe SpecSync's behaviour; a
  spec-sync worker found neither reproduces on spec-sync main 46f55652 (6.2.0), so
  0.6.0 closes both as fixed upstream, citing that check.
- #212 marks KNOW-2, 7, 18, 19, 22, 23, 25, 28 and 29 as superseded but does not retire
  them, because live requirements and cases still depend on them. This change retires
  all nine with `hi retire` and removes or rewrites what implements them; plan.md maps
  every affected requirement, criterion and case.
- Leif's two later answers, relayed by the coordinator on 2026-11-10 and captured
  here as KNOW-34 and KNOW-35: "Keep all (Recommended)" for finished-session stories
  (nothing trims the archive) and "Split them (Recommended)" for proof files (logs to
  CI; the record keeps key failing lines, the command and a fingerprint; a few small
  key screenshots are committed beside the story with a size cap).
- Shipping decision (Leif, 2026-11-10): one native change, released as 0.6.0,
  dogfooded on the chough repository only; other repositories update only after
  0.6.0 is released at 95 percent confidence or better.
- Windows is best-effort through WSL (FLOW-17 read with FLOW-21).

## Folded in

- PR #233 (actor identity, LIFE-17): REQ-records-007. Its REQ-cli-063 never landed;
  the PR closes as folded in when this change lands.
- PR #210 (append-only corrections, LIFE-18): REQ-history-015. Its REQ-history-015
  draft never landed; this change takes the ID. The PR closes as folded in.
- PR #208 (external artifacts by fingerprint, FLOW-22): REQ-records-005, changed so
  the committed record holds a neutral ID and fingerprint, never the absolute path
  (#234). The PR closes as folded in.
- PR #211 (#165 copy-never-move): closes as superseded by KNOW-30 when this change
  lands. It is not closed now.
- Issue #165 has a new real repro (algochat, 2026-11-10): accept to done deleted an
  earlier archived session with no prune record. REQ-history-006 carries the
  regression for that exact path, recorded failing first on 0.5.0.
- Issue #237 (a gate blocks every advance): decided at scope approval, see below.

## Decisions Leif makes at scope approval

1. **Gates (#237).** Should an open human gate block only the stage it names, so a
   pending release decision no longer stops build, test and accept? Recommended:
   yes (option A in design.md). The other option keeps every gate blocking every
   advance and adds a way to park later decisions.
2. **Screenshot cap (proposal, as Leif asked).** Is "PNG or JPEG, metadata removed, at most 512 KB each, 5 per
   piece of work and 2 MB in total, kept in a folder beside its story" the right cap?
   Recommended: yes; anything bigger stays on the machine that took it, recorded by
   fingerprint.
3. **The rest of the old cleanup rules.** 0.6.0 drops the proof-cleanup commands
   (`evidence prune`, `history prune`, `library archive` and `snapshot`) and the
   cleanup ledger, because evidence lives with its work and nothing large is
   committed. Should the cleanup criteria still in `hi/` (KNOW-8 to 11, 13, 16, 17,
   20 and 21) be retired the same way, keeping their safeguards as spec rules?
   Recommended: yes. Until they are, the plan reads them as satisfied by the fold at
   finish and REQ-records-012.
4. **Review summaries (#190).** Should a gate's "what changed" line count only the
   change's own files, measured from where the branch left main and leaving out
   Chough's records? Recommended: yes, rather than counting only the change's
   declared paths, which are often broad enough to hide stray edits.
5. **Guidance for a fresh agent (#76).** Right after `chough init`, `next` suggests
   `hi check` and then `chough context`; at test it names the cases to select and
   run; at acceptance it says the human reviews next and how to record their
   decision, never suggesting an agent review. Is that wording right? Recommended: yes.

Before this goes to Leif, an independent audit (chough-060-audit) classifies every
existing issue, PR, bug, criterion, requirement, case file, doc and the skill bundle
as keep, rework, remove, elsewhere or ask Leif; its verdicts are folded in before the
scope gate opens.

## For a session picking this up

- Read the deltas first; they are frozen at approval. design.md explains them.
- The chough repository's own records are still 0.5.x until the dogfood step; drive
  this session with the 0.5.0 build until the migration commit, then with the 0.6.0
  build (plan.md, Dogfood).
- `tests/spec_split.rs` pins the bodies of REQ-history-001 to 004 and
  REQ-reporting-001 to 003. This change modifies or removes REQ-history-001, 002,
  003, 004 and REQ-reporting-001, so those pins are updated naming this change.
- Ruled out: a YAML crate (strict subset instead, minimal dependencies); a sidecar
  JSON index for speed (a twin); one file per decision (the approved design keeps
  decisions in the session story); keeping the event-file history beside the
  timeline (two sources); trimming archived stories (Leif: keep all).
