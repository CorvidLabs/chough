# Deferred proposal: reusable observations after metadata commits

**Status (2026-11-01): addressed for content-identical commits** by native change
`keep-evidence-current-across-commits-and-specsync-lifecycle-bookkeeping-and-verify-bugs-only-by-a-pass-observed-after`
(D-018, superseding D-016). Freshness now compares a working content identity, and
the revision is recorded but not compared, so committing only `.chough/` records, an
empty commit or SpecSync lifecycle bookkeeping keeps an observation current. That
change adds no ancestry check and no separate reuse label; those ideas below remain
unapproved history.

**Earlier status: unapproved and deferred in favor of lifecycle work.** The human redirected the
next implementation discussion to the [agreed lifecycle evolution](lifecycle.md).
This proposal remains design history; it was neither approved nor rejected. No
scope approval, implementation review or native archive has been recorded for it.
Its commands below are conditional examples, not the current implementation task.
The initial baseline remains the authorized bootstrap. The lifecycle work it was
deferred for is native change `evolve-chough-into-a-resumable-stage-workflow`.

## Problem and product decision

The first helper includes HEAD in its input fingerprint. Any new commit makes an
observation stale, even when that commit only saves `.chough/` records. This is a
conservative, documented limitation: it prevents evidence silently following an
unverified revision, but makes shared evidence awkward to reuse.

If this proposal is revisited, decide whether a later slice should recognize
**matching observed inputs at a descendant commit**, while retaining the original observed revision and explicitly
avoiding a claim that a test ran at the new revision. Keeping exact-revision-only
freshness is also a valid product choice. No implementation of reuse exists yet.

Trace: LIFE-2, LIFE-3, LIFE-7; REQ-cli-004 and REQ-cli-005;
[CLI contract](../specs/cli/cli.spec.md), [freshness semantics](walkthrough.md#interpret-freshness-precisely).

## Proposed acceptance criteria

- An observation retains its original revision, environment, outcome and artifact
  bindings. Existing records are not rewritten to claim a later test run.
- An exact revision/input match stays distinct from reusable matching content at a
  descendant revision. Neither label grants acceptance or authentic human approval.
- A commit changing only excluded Chough records may make the old observation
  reusable only if input equivalence and ancestry are actually established.
- Changing a tracked or untracked nonignored source, test, skill, contract or
  executable bit remains stale. Changed artifacts or environment remain stale.
- Unavailable history, unrelated/rewritten history, unsupported record versions,
  missing artifacts or uncertain input identity remain unknown/stale, with a reason.
- A migration decision preserves existing records and their meaning. The native
  definition must specify the record version and exact output labels before approval.

## Non-goals

No automatic test execution, criterion acceptance inference, signing policy, model
client, Rune coordination, hosted service, release or deployment. Fledge continues
to execute tests; HI IDs and SpecSync lifecycle ownership stay unchanged.

## Prepare the actual native definition

After the human selects this outcome, use a new branch and the installed native
CLI. The following creates a draft; it does not approve one:

```sh
specsync change new "Reuse observations after metadata commits" \
  --spec cli --path src/main.rs --path tests/cli.rs \
  --path docs/cli.md --path docs/walkthrough.md --json
```

Use the returned ID and interview questions; complete the selected artifacts and
contract delta, including the migration/output-label decision. Present that exact
definition for a genuine human scope decision. Only then, if approved, record
`specsync change approve <returned-id> --actor <actual-human>` and follow native
status through implementation, product checks, `check --commit`, CI, genuine scoped
review, same-PR ship/finalize, archive commit/CI, and GitHub merge. Actor text is
not authenticated identity. Native status/recovery overrides this example sequence.

The baseline keeps native SDD's default optional policy. Before the first full
scoped cycle, inspect `specsync change adopt --help` and deliberately adopt the
repository policy together with the human; do not silently substitute a disabled
policy for the requested full lifecycle. This proposal stays a document to keep
the tested bootstrap clean and avoid an unapproved active package on main.
