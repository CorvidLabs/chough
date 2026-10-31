---
change: require-native-lifecycle-closure-before-chough-pull-requests-merge
artifact: research
---

# Research

First inspected 2026-10-31 against Chough main `57aa169588522ef44438236b9f03f409dc1254c2`, then re-inspected after the rebase against main `f19823d`, and published SpecSync v6.1.1 commit `e8fd149d4d9e7b8df9ea1d40d446df67da13f2fa`. Installed executable reports `specsync 6.1.1`; the published release is neither draft nor prerelease and includes Linux/macOS archives and SHA-256 assets. The action verifies downloaded checksums. No upstream source was changed.

## Verified native semantics

- `specsync change audit --json` is read-only; `src/change/historical_definition_digest_matches.rs::audit_project` calls the active-only implementation. Its documentation explicitly excludes archived terminal evidence. It validates active definitions/approval/deltas and policy/effective-contract coherence. Its CLI exit code follows native errors; `--strict` does not independently turn its report warnings into errors.
- `specsync change list --json` uses the native roster. `src/change/sdd_version.rs::list_changes_uncached` reports unreadable workspaces separately, distinguishes untrackable empty husks, and does not turn a malformed record into zero changes. The JSON presenter exits nonzero for degraded rosters. The no-active success output is exactly `[]`.
- `specsync change status <ID> --json` resolves active or archive locations natively. `summary.terminal_evidence` for archived records invokes `validate_archived_integrity_with_cache` and returns `authenticated-history` or `corrupt-history`. Inspection can exit zero even when the summary is unhealthy; consumers must inspect the value.
- Native state and readiness are separate. An isolated copied approved fixture with an empty approval ledger returned state `approved`, `approval_valid: false`, and a next action to approve. A state string alone is insufficient.
- `change check` is a writer/materializer that runs verification. It is unsuitable as this CI inspection command. `ship`, `finalize`, `approve`, and `review` are likewise excluded.
- `policy_at_comparison_base` uses `GITHUB_BASE_REF`/origin history in CI and fails when that trusted base cannot be inspected. Full history and a real PR base must remain available.
- The upstream `.github/scripts/archive-readiness.sh` at v6.1.1 rejects open workspaces but does not authenticate archive history. It also implements filesystem inventory heuristics, so copying it alone would not meet this stronger request. The proposed gate uses native roster and status for lifecycle truth.

Primary sources: [published release](https://github.com/CorvidLabs/spec-sync/releases/tag/v6.1.1), [native source at the published commit](https://github.com/CorvidLabs/spec-sync/tree/e8fd149d4d9e7b8df9ea1d40d446df67da13f2fa/src/change), [action wrapper](https://github.com/CorvidLabs/spec-sync/blob/e8fd149d4d9e7b8df9ea1d40d446df67da13f2fa/action.yml), [upstream archive check](https://github.com/CorvidLabs/spec-sync/blob/e8fd149d4d9e7b8df9ea1d40d446df67da13f2fa/.github/scripts/archive-readiness.sh).

## Current Chough gates

Three `Product / <os>` jobs run `cargo test --locked` and release builds. `Fledge / HI / SpecSync` installs Fledge v1.7.2, SpecSync 6.1.0 and HI 0.8.0, runs strict 100% structural coverage, then the verify lane (fmt, Clippy, tests, HI, SpecSync, build). These current jobs do not establish native archive closure. New runtime work is unpublished and is not a basis for claiming a current flow check.

## Protection findings and limits

Read-only `GET /repos/CorvidLabs/chough/branches/main` reported `protected: true` but `required_status_checks.contexts: []` and `checks: []`. `GET /repos/CorvidLabs/chough/rulesets` returned `[]`. Detailed `GET /repos/CorvidLabs/chough/branches/main/protection` returned HTTP 403, `Resource not accessible by integration`; the connected GitHub App lacks the administration access needed for that endpoint. Do not infer all enforcement settings from the compact branch response, and do not claim these proposed jobs currently block merges. No rules were changed.

Re-read later on 2026-10-31 with the owner's credentials: detailed protection was readable and reported required status checks enabled with `strict: false`, `contexts: []` and `checks: []`; `enforce_admins` true; required linear history true; force pushes disallowed; 0 required approving reviews. `GET /repos/CorvidLabs/chough/rules/branches/main` (effective rules, including organization rulesets) returned `[]`. No check is therefore required to merge today.

Once introduced on main, recommend requiring `Product / ubuntu-latest`, `Product / macos-latest`, `Product / windows-latest`, `Fledge / HI / SpecSync`, `CI gate fixtures`, and `SpecSync Archive`. A separately authorized administrator must verify and apply effective rules with the active PR impact understood.
