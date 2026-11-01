---
change: correct-archived-resume-guidance-and-record-the-kimi-cold-start
artifact: context
---

# Context

## What happened

A single cold-start exercise used Kimi Code CLI 0.41.0 on macOS arm64 in the existing worktree `.worktrees/kimi-host-cold-start`, on branch `kimi/feat/kimi-host-cold-start`. The product base was origin/main at `5af3f78d66b40a5af42607eafd5649c9b957d2ad`. The task supplied the repository, objective, `AGENTS.md`, and canonical skill path. Kimi also auto-discovered and loaded `.agents/skills/chough-workflow/SKILL.md` as a Project skill. The explicit task guidance means this was not an unaided repository-discovery experiment, but it does establish that this Kimi version discovered the `.agents` copy.

The original scratch log records these observations:

- `cargo build --locked` passed in 10 seconds.
- `./target/debug/chough next --environment local-macos-arm64` exited zero in 16 seconds and emitted 11567 JSON lines. The checkpoint stage was `done`, and `native_summary.state` was `archived`. Nevertheless, `next_valid_action` was `Continue done: specsync change review ... then ship`, while native `summary.next_action` was `merge the PR on GitHub if it is still open`.
- A direct `specsync change status <archived-change> --json` confirmed native state `archived` in 28 seconds. The stale review and ship instruction therefore contradicted the native authority rather than merely reflecting an unread native response.
- `./target/debug/chough status --json` exited zero in 36 seconds and emitted about 11567 lines. `./target/debug/chough skill status` reported every file current for both installed hosts. `./target/debug/chough cases check` exited zero in 2 seconds with valid definitions.
- The baseline `fledge lanes run verify` passed all 9 steps in 114 seconds. It used the initial global HI 0.5.0, Fledge 1.8.0, and SpecSync 6.1.1. The first two do not match CI, so this is a useful baseline observation, not final CI-matching verification.

Release-verified local HI 0.8.0 and Fledge 1.7.2 tools, together with SpecSync 6.1.1, are available under `target/tmp/ci-tools` for later verification. The global tool mismatch remains an environment finding. No agent recorded human scope approval, implementation review, release, deployment, or any other human decision.

## Findings and chosen scope

- BUG-0020 is in scope. `status()` in `src/main.rs` serves both `next` and `status`. After the paused, awaiting-human, blocking-bug, and closed-gate branches, its general fallback always appends the saved checkpoint action. At `done`, that can repeat completed review and ship work even when the parsed native summary is archived and supplies different guidance.
- BUG-0021 is in scope. Kimi discovered the `.agents` skill, but the canonical Host use section, walkthrough, and integration notes did not explain that the existing `openai` installer mapping creates that directory. The fix documents the mapping and does not add a `kimi` host value. The base canonical `SKILL.md` and both installed copies have SHA-256 digest `d350cc91271e6d942bef516a2b2ea8a6009697a2e0d034b9ff48c8591be2ee5c`, which is the digest to add to `KNOWN_PREVIOUS` for safe updates.
- BUG-0022, the initial global HI and Fledge drift, stays open and out of scope. This change uses the CI-matching local tools for final evidence but does not alter global tool installation.
- BUG-0023, oversized and slow `next` and `status` output, stays open and out of scope. The large output made the contradiction harder to find, but this change does not redesign output or evidence retention.
- BUG-0001 needs no source edit. Its configuration-comment issue is already fixed on the selected origin/main base.

The exercise also exposed stale roadmap and host notes. GitHub readbacks confirm PRs #8 and #9 merged on 2026-10-31 UTC, and PRs #15, #16, and #17 merged on 2026-11-01 UTC. Their merge commits are respectively `090460d`, `2756dc2`, `3ddbcca`, `8b30943`, and `c26bc26`. The roadmap still calls some of this work pending, omits other delivered rows, and describes observation reuse as unapproved even though PR #17 implemented content-identical freshness. Documentation must distinguish those delivered facts from a release, which still has not occurred.

## Resume constraints

The native change is a draft and human scope approval is pending. The implementation request and the existence of this definition are not that approval. Product edits and pre-fix regression execution belong after the human approves this concrete scope and native status permits build.

The evidence will preserve the distinction between the original run and later review. There was one Kimi run, with successful skill discovery and several successful checks, but failed cold-resume guidance. There was no fresh post-fix Kimi run. TC-cli-0106 therefore uses a retrospective review of the original log and later documentation, not a fabricated rerun. TC-cli-0105 supplies the automated pre-fix and post-fix regression evidence for BUG-0020.

Existing requirements already cover the fix. No new HI criterion, requirement ID, canonical contract delta, host enum, integration, runtime, service, or duplicate lifecycle state is needed. `.chough` case, bug, artifact, evidence, and checkpoint records may be updated by their designated records owner, but `.chough` must not be added to the native affected paths because doing so would make routine record writes invalidate native verification.

## Interview note

The first pre-push audit rejected `no_spec_change` with the initial `public_contract: yes` answer. Following the archived interview convention in `parse-native-status-beyond-the-display-bound-and-apply-checkpoint-clears-first`, `public_contract` is answered `no`: the canonical contract and requirements do not change, while observable CLI behavior changes from a defect back to what REQ-cli-003 and REQ-cli-007 already specify. The refusal was resolved through the native interview command, not by editing native state.
