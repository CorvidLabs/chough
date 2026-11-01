# Native lifecycle and evidence

Installed `specsync change status <id> --json`, `specsync change ship-status <id>` and
command help are authoritative for the current state and recovery path. This
reference follows SpecSync 6.1; do not hardcode it over another version's output.
Match the SpecSync version used by lifecycle writers and CI.

## Shape of a change

```text
new → interview → artifacts and delta → one human scope approval → implement →
check (when requirements change) → cases run → gate → commit with .chough/ →
check --commit → push, CI → human implementation review → review → ship/finalize →
clear gate, stage advance → push archive, CI → merge
```

Between `check --commit` and finalize, nothing writes to `.chough/` and nothing is
committed: SpecSync's verification digest covers `.chough/`. SpecSync 6.1.2 ships an
opt-in exclusion, `verification_ignored_paths`, but Chough's own repository has not
adopted it, because entries are directory prefixes and covering `.chough/session.json`
would also hide `.chough/cases/`. Unless a project lists only safe directories, any
Chough write in that window forces another `change check`. See [accept.md](accept.md)
for the full order.

| Native state | Chough stage | Gate |
|---|---|---|
| draft | intent, design, plan | build, test, accept and done refused |
| approved, implementing, verifying, with `approval_valid` true | build, test, accept | build, test and accept allowed; done refused |
| archived, with authenticated `terminal_evidence` | test, accept, done | done allowed; build refused (new work needs a new change) |

Chough reads `summary.state`, `summary.approval_valid`, `summary.correction_valid` and,
for archived changes, `summary.terminal_evidence.validity`. The same prerequisite
applies whichever command moves the stage (advance, skip, back, reset, return or
release → done). It refuses when status is missing, cannot be decoded or reports an
invalid approval or a correction reported invalid (`correction_valid` false), when no
native change is recorded in the checkpoint (`refused:no-native-change`; record one
with `chough checkpoint --native-change <id>`), and when `.specsync/sdd.json` is not a
valid SpecSync policy. Two cases are not gated at all: with no SpecSync configuration
the gate reads `ungated:not-configured`, and with no `.specsync/sdd.json` or one with
`"enabled": false` it reads `ungated:sdd-disabled`. It never runs `approve`, `review`
or `finalize` for you.

## Approval

Record `specsync change approve <id> --actor "<human>"` only after a real human
decision about this concrete definition. A general instruction to keep working is
not approval of a new definition. Actor text is a portable claim, not identity.

## Verification

`specsync change check <id>` materializes the approved delta; run it before cases
that link requirements the delta adds or modifies. `specsync change check <id>
--commit` also records targeted structural verification; run it after committing the
source with `.chough/`. Stage new source files first. It does not run the project's
tests; product checks are separate observations. It refuses an edited delta and
incomplete tasks. Fix real failures; if evidence is stale, follow the exact recovery
action status names.

## Review, archive, merge

Push only after the applicable local gates and observe required CI at the actual tip.
After the human reviews the change package, diff, delta and evidence:

```sh
specsync change review <id> --reviewer "<human>"
specsync change ship <id>
```

The reviewer may be the scope approver. Do not commit between review and ship or
finalize. Archive on the same pull request, push the archive commit, wait for its
checks, and merge only when every change on that pull request is archived. Review and
finalize several changes one at a time.

Merge, package publication, release, deployment and live health are separate
observations; see [release.md](release.md).

## Evidence records

`chough cases run` and `chough evidence add` record with an observation the Git
revision, a working content identity, the environment, artifacts and, for cases, the
definition digest. Freshness compares the content identity, not the revision: it
covers every tracked or untracked non-ignored file except `.chough/`, `target/` and
SpecSync lifecycle bookkeeping (a change's `state.json`, `approvals.json`,
`verification.json`, `verification-attempts.json`, `review.json`,
`review-attempts.json`, `finalization.json`, `accepted-state.json`,
`lesson-bundle.md` and the front-matter `state:` value of `change.md`). So committing
only Chough records, an empty commit, `change check --commit` and finalize keep an
observation current, while a change to source, specs, configuration or a change's
deltas and other artifacts makes it stale. A record without a content identity, from
an earlier Chough, keeps the revision-bound comparison and is stale after any commit.
The records do not authenticate a witness or verify a remote artifact. A changed
input, artifact, case definition or environment makes the observation stale.
