# Native lifecycle and evidence

Installed `specsync change status <id> --json`, `specsync change ship-status <id>` and
command help are authoritative for the current state and recovery path. This
reference follows SpecSync 6.1; do not hardcode it over another version's output.
Match the SpecSync version used by lifecycle writers and CI.

## Shape of a change

```text
new → interview → artifacts and delta → one human scope approval → implement →
check --commit → push, CI → human implementation review → review → ship/finalize →
push archive, CI → merge
```

| Native state | Chough stage | Gate |
|---|---|---|
| draft | plan | build refused |
| approved, implementing, verifying | build, test, accept | build allowed; leaving accept refused |
| archived | accept → done | done allowed |

Chough reads `summary.state` for these gates and refuses when status is missing or
cannot be decoded. It never runs `approve`, `review` or `finalize` for you.

## Approval

Record `specsync change approve <id> --actor "<human>"` only after a real human
decision about this concrete definition. A general instruction to keep working is
not approval of a new definition. Actor text is a portable claim, not identity.

## Verification

`specsync change check <id> --commit` materializes the approved delta and records
targeted structural verification. Stage new source files first. It does not run the
project's tests; product checks are separate observations. It refuses an edited
delta and incomplete tasks. Fix real failures; if evidence is stale, follow the exact
recovery action status names.

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

`chough cases run` and `chough evidence add` bind an observation to the Git revision,
working-tree fingerprint, environment, artifacts and, for cases, the definition
digest. They do not authenticate a witness or verify a remote artifact. A changed
input, artifact, case definition or environment makes the observation stale.
