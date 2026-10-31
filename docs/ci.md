# Chough CI and native closure

The CI workflow preserves product tests and release builds on Ubuntu, macOS, and
Windows, plus formatting, Clippy, HI, and strict 100% SpecSync structural coverage.
`CI gate fixtures` exercises native archive inspection in disposable Git repos.
`SpecSync Archive` independently checks that every native scope is archived before
merge. It runs on draft PRs and can be expected red during work.

The gate uses published SpecSync **6.1.1**, its pinned action commit, and the
upstream action's release archive/checksum verification. The action version input
is explicit: the v6.1.1 wrapper's default binary is 6.1.0. Fledge remains v1.7.2
and HI remains 0.8.0. Workflow actions are pinned to verified commit SHAs.

## What Archive checks

The inspector runs only native readers: `change audit`, `change list`, and
`change status <ID>`, all using JSON. Audit checks live definitions, policy,
deltas, and current evidence. Any active roster entry blocks Archive, even a
scope-only draft or a change labelled accepted. Disabled bootstrap SDD does not
hide the roster. Missing tools, wrong versions, invalid output, missing/malformed
configuration, degraded inventories, and unavailable history fail the check.

Audit deliberately excludes archived history. The inspector inventories every
archive package and requires native status to report `archived` with
`terminal_evidence.validity: authenticated-history`. It carries lifecycle IDs
from the trusted merge base and every PR commit snapshot forward to the head, so
deleting a scope or archive cannot create a passing empty inventory. Temporary
Git snapshots preserve object history and never execute historical product code.
Source/lifecycle hashes and Git status must remain unchanged by inspection.

`authenticated-history` is a native structural provenance result. It does not
authenticate a human identity. Actor names, model text, files named review, and
synthetic fixture approvals do not establish genuine human acceptance. The gate
does not run approval, review, check/materialization, ship, finalize, or archive.
Follow actual native status, obtain genuine human review, and finalize before
merging; CI never performs those decisions.

## Local verification

Use Python 3.11 or later and the published SpecSync 6.1.1 executable:

```sh
fledge run ci-archive-fixtures
python3 .github/scripts/check-specsync-archive.py --base <trusted-base-sha>
fledge lanes run verify
```

The fixture task creates real native test lifecycles with clearly synthetic
identities in temporary repositories. It never approves actual Chough changes.
Positive archive results use native historical authentication; broken-tool mocks
are used only for rejection tests. A wrong local binary version is an error;
select the correct tool for that invocation without replacing global tools.

## Required-check recommendation

After these jobs exist on main and the effect on active PRs is reviewed, recommend
requiring these exact contexts:

- `Product / ubuntu-latest`
- `Product / macos-latest`
- `Product / windows-latest`
- `Fledge / HI / SpecSync`
- `CI gate fixtures`
- `SpecSync Archive`

A CI job and an enforced merge rule are separate. On 2026-10-31 the compact main
branch response reported `protected: true` with empty required-check context and
check arrays. The repository rulesets response was `[]`. The detailed protection
endpoint returned HTTP 403, `Resource not accessible by integration`, because the
connected GitHub App cannot read administration-protected settings. These reads
do not establish the complete effective protection configuration or prove Archive
is enforced. This PR changes no branch rules. A separately authorized repository
administrator must verify the effective configuration and activate required
checks after introduction.

## Runtime integration follow-up

The separate stage-workflow PR is not part of this branch. Once its actual
interfaces land, extend current-source CI with adversarial stage-gate tests,
nonempty case/profile/traceability checks, complete installed skill bundle checks,
cold-resume fixtures, current evidence checks, stop-during-interval regressions,
and bounded-process/descendant-output tests. Preserve its native audit task when
reconciling workflow changes. Those planned checks are not reported as green here.
