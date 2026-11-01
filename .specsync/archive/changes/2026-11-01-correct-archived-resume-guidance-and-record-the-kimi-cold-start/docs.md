---
change: correct-archived-resume-guidance-and-record-the-kimi-cold-start
artifact: docs
---

# Docs

The documentation must report the original Kimi observation without turning it into evidence of a post-fix run. The confirmed merge metadata below comes from the merged pull requests and their merge commits. Dates are the observed GitHub merge dates. Do not infer publication, release, deployment, or human acceptance from a merge.

## Canonical skill and installed copies

Update `skills/chough-workflow/SKILL.md` in the Host use section. Explain that Kimi Code CLI auto-discovers the Project skill at `.agents/skills/chough-workflow/SKILL.md` and that `chough init --host openai` installs that directory. Keep `openai` as the installer value; do not document or add a `kimi` value. Retain the instruction to confirm discovery in the actual host and to provide the Markdown file directly when discovery is unavailable.

Make `.agents/skills/chough-workflow/SKILL.md` and `.claude/skills/chough-workflow/SKILL.md` byte-identical to the canonical file. The safe-update digest and automated bundle checks are implementation work listed in `tasks.md`, not separate documentation claims.

## README.md

- In the Start here installation guidance, add the Kimi mapping beside the existing host directory list: Kimi Code CLI uses the `openai` installer value and discovers `.agents/skills/chough-workflow/SKILL.md`. Do not present Kimi as a new CLI host enum value.
- Replace the statement that the first release candidate is still in progress in PR #9 with the observed fact that PR #9 merged as `2756dc2` on 2026-10-31. Preserve and emphasize the separate fact that no release is published, tagged, hosted, or deployed.
- Add a concise observed-host note for Kimi Code CLI 0.41.0 on macOS arm64: one 2026-11-01 run auto-discovered the `.agents` skill, built successfully, and exercised cold resume. It exposed BUG-0020's stale archived action and BUG-0021's missing mapping guidance. State that later fixes are covered by regression and documentation review, not by a fresh Kimi rerun.
- Replace the blanket limitation that hosts other than Claude Code and Codex are unexercised. Keep Cursor, Gemini, and Grok unexercised, describe Kimi as one limited run, and retain all other platform and delivery limits.

## docs/ROADMAP.md

Update the delivered-work rows using these confirmed facts:

- Bounded external commands: delivered in PR #8, merge commit `090460d`, merged 2026-10-31. Remove awaiting-merge wording.
- First release candidate: delivered in PR #9, merge commit `2756dc2`, merged 2026-10-31. Remove in-progress wording and continue to state that no release is published.
- Timing report: add the missing row for read-only stage, test, bug-fix, and human-gate timing, with elapsed wall-clock labels and history ranges that are estimates rather than targets. Delivered in PR #15, merge commit `3ddbcca`, merged 2026-11-01.
- Bundle hygiene and actionable errors: add the missing row for case-binding environment, colour-safe matching, dash-free bundle hygiene, and actionable requirement and bug errors. Delivered in PR #16, merge commit `8b30943`, merged 2026-11-01.
- Evidence freshness and bug verification: replace the old Observation reuse row with the delivered PR #17 behavior rather than leaving it unapproved and deferred. Use merge commit `c26bc26` and merge date 2026-11-01. Describe content-identical commits accurately: Chough-only records, empty commits, committing edits already present when observed, and SpecSync lifecycle bookkeeping keep observations current; any fingerprinted content change still stales them; records without a content identity remain revision-bound; and bug verification requires a pass after the latest failure. Keep ancestry checks and a separate reuse label outside the delivered behavior.

Update the More hosts row to record one Kimi Code CLI 0.41.0 discovery and cold-start run on macOS arm64, including the successful `.agents` discovery and the archived-resume defect it exposed. Keep Cursor, Gemini, and Grok as proposed exercises. State the one-run and no-fresh-post-fix-rerun limits. Do not turn the roadmap into an evidence ledger or claim that merging any of these rows constituted a release.

## docs/integrations.md

Add a Kimi Code CLI 0.41.0 row to the Hosts table. Its project skill directory is `.agents/skills/chough-workflow/` through the existing `openai` mapping. Its observed support is one macOS arm64 run on 2026-11-01 that auto-discovered and loaded the skill and exercised cold resume. Mention the stale archived-action defect found by that run and state that no fresh post-fix Kimi run was performed.

Leave Cursor, Gemini, and Grok in the unexercised mapping row. Keep the directory-convention warning and `chough skill status` distinction: an installed or discovered directory is not blanket proof of support. Do not add a new integration, provider API, runtime, service, or native tool boundary.

## docs/walkthrough.md

In the Install section, tell Kimi users to initialize with `--host openai` and identify `.agents/skills/chough-workflow/` as the directory Kimi Code CLI discovered in the observed run. Keep the existing Claude example and the plain-file fallback for other hosts.

In the Resume cold section, retain the independent Codex sample and add a separate reference to the one Kimi repository cold start. Say that Kimi discovered the skill and reached the archived native state but received stale review and ship guidance, which became BUG-0020. The automated regression demonstrates the correction; do not say Kimi returned to validate the corrected build. Link the cold-start exercise for the full observation and limits.

## docs/exercises/cold-start.md

Preserve the original Codex exercise as a historical record. Remove only its now-false blanket statement that a Kimi run was not performed, replacing it with a precise statement about hosts still not exercised on that sample.

Add a separate Kimi cold-start section dated 2026-11-01 with these details:

- Host and environment: Kimi Code CLI 0.41.0, macOS arm64, `local-macos-arm64`, worktree `.worktrees/kimi-host-cold-start`, branch `kimi/feat/kimi-host-cold-start`, base `5af3f78`.
- Input and discovery: the supplied task named the repository, objective, `AGENTS.md`, and canonical skill; Kimi independently listed and loaded the `.agents` Project skill. This is successful discovery, not proof that every Kimi configuration discovers it.
- Commands and results: build passed in 10 seconds; `next` emitted 11,567 JSON lines in 16 seconds with stage `done` and archived native state but stale review and ship guidance; native status confirmed archived; `status` emitted about 11,567 lines in 36 seconds; skill status was current; cases check was valid; and the baseline verify lane passed 9 steps in 114 seconds.
- Findings and scope: BUG-0020 and BUG-0021 are in this change, while BUG-0022 tool drift and BUG-0023 output size remain open and out of scope. The baseline used HI 0.5.0 and Fledge 1.8.0 with SpecSync 6.1.1, so it is not final CI-toolchain verification. Later release-verified HI 0.8.0 and Fledge 1.7.2 are under `target/tmp/ci-tools`, with SpecSync 6.1.1 required for final checks.
- Evidence and limits: link `.chough/artifacts/kimi-cold-start.log` and TC-cli-0106. State that the original guidance was absent, the later TC-cli-0106 pass is retrospective documentation review, and no fresh post-fix Kimi run, agent-recorded human decision, release, or deployment is claimed.

## specs/cli/testing.md

Revise the introductory case-definition summary so it no longer describes only the early automated and exercise IDs. Include TC-cli-0105 as the BUG-0020 archived-guidance regression and TC-cli-0106 as the BUG-0021 retrospective Kimi guidance review. Do not rewrite historical counts in older observation sections as though they were current counts.

Update the requirement matrix as follows:

- Add TC-cli-0105 to REQ-cli-003 for native-guided next and status behavior and to REQ-cli-007 for checkpoint resumption, paused and awaiting-human precedence, safe missing-native fallback, and unchanged session guidance.
- Add TC-cli-0106 to REQ-cli-016 alongside TC-cli-0037. Explain that TC-cli-0037 proves byte-identical installation and safe update mechanics, while TC-cli-0106 reviews the Kimi mapping and documentation without executing a new host session.

Add an observed Kimi cold-start subsection based on the concise log, including host and base, discovery, commands and timings, BUG-0020 through BUG-0023, initial tool drift, and later CI-matching tool requirements. Identify the original TC-cli-0106 observation as failed because guidance was absent. Describe any later TC-cli-0106 review as documentation-only and report its outcome only from the actual later observation. Keep BUG-0022 and BUG-0023 visible as remaining limitations. Update the not-covered host list so it no longer includes Kimi as wholly unexercised, while explicitly retaining the absence of a post-fix Kimi run and the unexercised Cursor, Gemini, and Grok hosts.

Do not prewrite final verification results. Final passes belong in actual observations on the final content; if a final result is later added to this file, rerun the affected cases and verify lane after that edit.

## specs/cli/tasks.md

Remove Kimi from the open follow-up that lists unexercised hosts. Leave Cursor, Gemini, and Grok in that follow-up.

Add a completed documentation and evidence task for the single Kimi Code CLI 0.41.0 macOS arm64 run, linking the cold-start exercise, concise log, and TC-cli-0106. Its wording must include both the discovery pass and the cold-resume and guidance defects, and must say that the later pass is a retrospective documentation review rather than another Kimi run. Do not mark BUG-0022 or BUG-0023 complete and do not add a release or deployment task as completed.

## Scope exclusions

Do not change `docs/FRD.md`, HI files, `specs/cli/requirements.md`, or `specs/cli/cli.spec.md`; this fix uses existing requirements and needs no canonical contract delta. Do not add host integrations or exercises beyond the one observed Kimi record. Do not document a fix for oversized `next` or `status` output, global tool installation drift, or the already-fixed BUG-0001 config comment as work performed by this change.
