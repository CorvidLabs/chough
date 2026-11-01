# Lesson bundle — correct-archived-resume-guidance-and-record-the-kimi-cold-start

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Correct archived resume guidance and record the Kimi cold start
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: src/main.rs, src/skill.rs, tests/stages.rs, tests/optional.rs, skills/chough-workflow/SKILL.md, .agents/skills/chough-workflow/SKILL.md, .claude/skills/chough-workflow/SKILL.md, docs/exercises/cold-start.md, docs/ROADMAP.md, docs/integrations.md, docs/walkthrough.md, README.md, specs/cli/testing.md, specs/cli/tasks.md
- **Acceptance**: For a done checkpoint bound to an archived native change, chough next and status derive next_valid_action from native summary.next_action rather than stale checkpoint review or ship text; paused and awaiting-human handling keeps precedence, missing native guidance gets a safe status fallback, and session_next_action remains unchanged. TC-cli-0105 fails before the fix and passes afterward, with both observations linked to BUG-0020 (FLOW-13, LIFE-3, REQ-cli-003, REQ-cli-007).
- **Acceptance**: Kimi Code CLI guidance explains the existing openai to .agents skill mapping without adding a kimi host value. The canonical and two installed SKILL.md copies stay byte-identical, the previous digest is recognized for safe updates, edited copies remain protected, and chough skill status reports current. TC-cli-0106 records the original guidance failure and a later retrospective documentation pass linked to BUG-0021, without claiming a fresh post-fix Kimi run (FLOW-1, LIFE-4, REQ-cli-016).
- **Acceptance**: A concise .chough/artifacts/kimi-cold-start.log and its evidence record capture Kimi Code CLI 0.41.0 on macOS arm64, the supplied task guidance, base revision, commands, salient output, timings, successful discovery and checks, BUG-0020 through BUG-0023, initial tool drift, later CI-matching tools, and the absence of agent-recorded human decisions. The record states one-run and no-fresh-rerun limits.
- **Acceptance**: The roadmap marks PRs 8, 9, 15, 16 and 17 merged with their observed dates, adds the missing timing, bundle and errors, and evidence-freshness rows, corrects content-identical observation reuse, and describes one Kimi run with its discovery pass and cold-resume defect. README, integrations, walkthrough, cold-start exercise and CLI testing and tasks notes no longer say Kimi is unexercised; no release or deployment is claimed.
- **Acceptance**: Focused Rust tests, TC-cli-0105 and TC-cli-0106 evidence, bundle checks and fledge lanes run verify pass on the final content using HI 0.8.0, Fledge 1.7.2 and SpecSync 6.1.1. BUG-0022 global tool drift and BUG-0023 output size stay open and out of scope. No new integration, runtime, service, HI criterion, requirement ID or canonical contract delta is introduced.
- **Acceptance**: At the scope gate the branch contains the completed draft definition and cold-start records, is pushed, and has a draft PR titled Draft: Kimi host cold start and roadmap refresh. At implementation review its description names changes, cold-start findings, evidence, deviations, and limits. The agent never marks it ready and records no native review, archive, merge, release or deployment.

## Evidence

- Verification commit: `4faa7c7c4e9633b6f557307bf53630857c9ec60e`
- Base commit: `5af3f78d66b40a5af42607eafd5649c9b957d2ad`
- Verified by: `specsync check --spec cli`

## From the change's context.md

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

## From the change's design.md

# Design

## Shared next-action routing

Keep one implementation in `status()` in `src/main.rs`, because both `chough next` and `chough status` already call it. Reuse the existing bounded native read and parsed `native_summary`; do not add another SpecSync invocation, lifecycle field, or writable state. The saved checkpoint remains the source of `session_next_action`, and that JSON field is returned byte-for-byte as the same string before and after the fix.

For a schema 2 checkpoint, calculate `next_valid_action` in this exact order:

1. If the stage status is `paused`, keep the current paused message. This wins even when the stage is `done`, native state is archived, or a human gate is also present.
2. If `awaiting_human` is nonempty, keep the current waiting message and its instruction not to record the decision for the human.
3. If the current stage is `test` or `accept` and acceptance-blocking bugs are open, keep the current return-to-build message.
4. If the current stage is `plan` or `accept` and its computed native gate is closed, keep the current `Gate not open` message.
5. If the current stage is `done`, do not use the saved checkpoint action:
   - When `native_summary.state` is exactly the string `archived` and `native_summary.next_action` is a string whose trimmed value is nonempty, return `Continue done: <native next-action>`, preserving the native text exactly after the existing stage prefix.
   - Otherwise return exactly `Done checkpoint has no archived native next action; inspect native SpecSync status before further lifecycle action.` This fallback covers missing, null, blank, or non-string native guidance; missing, failed, unavailable, undecodable, oversized, timed-out, or unconfigured native status; and a native state other than archived.
6. For every other situation, keep the existing `Continue <stage>: <checkpoint.next_action>` fallback. Schema 1 migration guidance is unchanged.

The fallback does not infer that a merge, review, release, or deployment is valid. Existing `native_specsync`, `native_summary`, gate details, and unknown-state diagnostics remain in the report, so the safe fallback does not hide why native guidance could not be used. Neither command writes the checkpoint or executes the reported action.

TC-cli-0105 binds one Unix-gated integration test in `tests/stages.rs`. Its fixture drives a schema 2 checkpoint to `done` with archived native state and a deliberately stale review-and-ship checkpoint action. It asserts all of the following for both `next --json` and `status --json` where applicable:

- archived native guidance replaces the stale action exactly, while `session_next_action` still equals the saved stale string;
- awaiting-human text takes precedence over archived guidance, and paused text takes precedence when both paused and awaiting-human conditions exist;
- missing native next-action text produces the exact safe fallback, not the saved review-and-ship text;
- a failed native read also produces the safe fallback and retains the existing native failure diagnostic.

Run this test before changing `status()` and record its failed TC-cli-0105 observation, then link it to BUG-0020. After the fix, run the unchanged definition and test again, link the later pass to BUG-0020, and apply the existing strictly later pass rule before marking the bug verified.

## Kimi skill guidance and safe update

Edit only `SKILL.md` in the canonical bundle and its two installed copies, not the host table or other bundle files. Add Host use guidance with this meaning: `For Kimi Code CLI, use --host openai. The existing openai mapping installs .agents/skills/chough-workflow/, which Kimi Code CLI 0.41.0 was observed discovering; there is no separate kimi host value.` Format the command and paths as inline code in the Markdown.

Add the base digest `d350cc91271e6d942bef516a2b2ea8a6009697a2e0d034b9ff48c8591be2ee5c` for `SKILL.md` to `KNOWN_PREVIOUS` in `src/skill.rs`, identified with base `5af3f78` and BUG-0021. Existing `file_state` and `report` logic stays unchanged: current files are untouched, a byte-identical base copy is `known-outdated` and can be replaced, and every other nonmatching copy is `edited` and preserved.

Extend the existing test bound to TC-cli-0037 in `tests/optional.rs`; do not add another automated case ID for the same behavior. Keep its 0.1.0 fixture coverage, and add the exact LF bytes of the base `SKILL.md` as a test constant. Assert its SHA-256 digest is the value above, install it as an old copy, and verify this sequence:

1. `skill status` reports `known-outdated`, not `edited`.
2. `skill update` replaces it with bytes identical to the new canonical file, and subsequent status reports `current`.
3. A separately edited copy is still reported `edited`, has action `preserved-edit`, and retains its exact edited bytes.
4. Fresh `.agents` and `.claude` installations remain byte-identical to the canonical bundle.

After updating the repository's canonical file, use the new binary's safe update path for the `.agents` and `.claude` copies. Compare all three `SKILL.md` files byte-for-byte and require `chough skill status` to report every bundle file current for both hosts. Do not add `kimi` to `HOSTS`, command help, or installer metadata.

## Cases and honest retrospective evidence

Add TC-cli-0105 and TC-cli-0106 to `.chough/cases/cli.cases.toml` without adding `.chough` to native affected paths.

- TC-cli-0105 is automated, functional, and linked to FLOW-13, LIFE-3, REQ-cli-003, and REQ-cli-007. Its paths are `src/main.rs` and `tests/stages.rs`; its environments are `local-macos-arm64` and `ci-linux`, matching the Unix native stubs. Its expected result names the single regression test above.
- TC-cli-0106 is agent-assisted and functional, linked to FLOW-13, FLOW-1, LIFE-4, REQ-cli-007, and REQ-cli-016. Its paths cover the three `SKILL.md` copies and the affected host documentation. Its setup names the original concise Kimi log and the documentation under review. Its expected outcomes require the original guidance gap to be identified, the updated guidance to explain the existing `openai` to `.agents` mapping without a new host value, and the review to avoid claiming a fresh post-fix Kimi run.

Use the records owner's `.chough/artifacts/kimi-cold-start.log`, distilled from the original scratch capture now retained locally under `target/tmp/`, and review the durable artifact rather than recreating it. The concise artifact keeps the supplied task guidance and its discovery limitation, host and operating-system versions, base revision, commands, salient output, exit outcomes, and elapsed times. It includes the successful discovery, build, native archived-state confirmation, current skill status, valid cases check, and baseline 9-step verify; the stale next action; BUG-0020 through BUG-0023; initial tool drift; and later CI-matching local tools. It does not copy the thousands of native status or observation lines.

Before editing the guidance, record the original TC-cli-0106 observation as failed through `chough cases record`, using the concise log as an artifact. Its summary states that the retrospective review of the original run found the mapping guidance absent. Its limitations state that this was one run, the task supplied explicit paths, the first verify used non-CI HI and Fledge versions, and no fresh post-fix Kimi run occurred. Link this original observation to BUG-0020 and BUG-0021, and append it as supporting evidence to out-of-scope BUG-0022 and BUG-0023 without moving or closing them. The cold-resume objective failed even though the artifact separately records which checks passed.

After the source and documentation edits, perform TC-cli-0106 again as an agent-assisted retrospective documentation review. Record a later pass only after checking the original log against the final guidance and docs. Link that pass to BUG-0021. This is evidence about the original record and final text, not evidence of another Kimi discovery or cold start.

## Documentation corrections

- `README.md`: remove the statement that PR #9 is still in progress, retain the fact that no release is published, document Kimi's existing `openai` mapping, and replace the blanket unexercised-host limitation with the one-run Kimi observation and remaining unexercised hosts.
- `docs/integrations.md`: add a Kimi Code CLI 0.41.0 row for `.agents/skills/chough-workflow/` through the `openai` mapping. State that discovery passed and the single cold start exposed BUG-0020. Do not generalize this into support for every Kimi version or a fresh post-fix run.
- `docs/walkthrough.md`: show Kimi users `--host openai`, and cite both the existing Codex exercise and the new Kimi observation without conflating their repositories or outcomes.
- `docs/exercises/cold-start.md`: preserve the Codex record and add a separate Kimi section with the supplied-guidance limitation, archived-done contradiction, partial successes, tool drift, and links to BUG-0020 through BUG-0023.
- `docs/ROADMAP.md`: mark PR #8 merged at `090460d` on 2026-10-31 UTC and PR #9 merged at `2756dc2` on 2026-10-31 UTC. Add delivered rows for the timing report in PR #15 at `3ddbcca`, bundle hygiene and case and error improvements in PR #16 at `8b30943`, and evidence freshness and ordered bug verification in PR #17 at `c26bc26`, all on 2026-11-01 UTC. Update the More hosts row with the one Kimi run and its defect. Change Observation reuse to the implemented content-identical behavior: freshness compares working content identity, the revision is recorded but not compared, and content-identical commits keep evidence current. Legacy records without content identity stay revision-bound; no ancestry check or separate reuse label was added, and those broader ideas remain unapproved history.
- `specs/cli/testing.md`: map TC-cli-0105 to REQ-cli-003 and REQ-cli-007, map TC-cli-0106 and the expanded TC-cli-0037 to REQ-cli-016, and record observed evidence with its environments and limitations. Remove Kimi from the unexercised-host list without removing Cursor, Gemini, and Grok.
- `specs/cli/tasks.md`: record that Kimi discovery and one cold start were exercised and found follow-up defects. Leave the other host exercises open and do not equate the exercise with release or human acceptance.

No edit is needed to `INTENT.md`, `hi/`, `docs/FRD.md`, `docs/DECISIONS.md`, `docs/NEXT_CHANGE.md`, `specs/cli/requirements.md`, or `specs/cli/cli.spec.md`. The canonical contract and durable requirements already describe the required behavior.

## From the change's testing.md

# Testing

## Evidence rules

Keep the original failures and the later passes as separate observations. Record the BUG-0020 failure before changing `src/main.rs`, and the BUG-0021 failure before changing the skill guidance or its documentation. Link each failure and its later pass to the relevant bug. A bug can be marked verified only after a present pass observed strictly later than its latest present failure. Do not alter timestamps, discard failures, or weaken an expected outcome to obtain a pass.

TC-cli-0106 is retrospective. Its original observation reviews the one Kimi run and the guidance available at base `5af3f78`; its later observation reviews the corrected text and the same original log. Neither observation is evidence of a fresh post-fix Kimi session. Structural checks and review notes are not human acceptance, release, or deployment evidence.

## Acceptance criterion coverage

1. Archived resume guidance is covered by the new automated TC-cli-0105. It must exercise both `chough next` and `chough status` against a schema 2 checkpoint at stage `done` whose native change is archived, while the checkpoint next action contains stale review and ship instructions. Both commands must derive `next_valid_action` from `native_summary.next_action`, leave `session_next_action` and the checkpoint bytes unchanged, and never recommend the stale lifecycle steps. The same test must prove that paused handling wins over native guidance, awaiting-human handling wins when the checkpoint is active, and unavailable or missing native next-action guidance produces a safe instruction to inspect native status rather than reusing stale review or ship text. This covers FLOW-13, LIFE-3, REQ-cli-003, and REQ-cli-007.

2. Kimi installation guidance is covered by TC-cli-0106 together with the existing automated TC-cli-0037. The retrospective case checks that the guidance tells a Kimi user to use the existing `openai` mapping for `.agents/skills/chough-workflow/`, without adding a `kimi` host value. TC-cli-0037 must also prove that canonical, `.agents`, and `.claude` bundles are byte-identical, the digest of the pre-change `SKILL.md` is `known-outdated` and safely updated, an edited copy remains `edited` and is preserved, and a synchronized installation reports `current`. This covers FLOW-1, LIFE-4, and REQ-cli-016.

3. The original cold-start record is covered by the failed TC-cli-0106 observation and its attached `.chough/artifacts/kimi-cold-start.log`. Review the record for Kimi Code CLI 0.41.0 on macOS arm64, the supplied repository and task guidance, worktree and branch, base `5af3f78`, commands, salient output, and elapsed times. It must preserve the successful `.agents` discovery, the archived native state and contradictory stale action, all four bugs, the initial tool drift, the later CI-matching tool location and versions, and the absence of agent-recorded human decisions. Its limitations must state that this was one run and that no fresh post-fix Kimi rerun occurred. The record may describe base `5af3f78` in its summary and artifact, but must not falsify the snapshot of the later retrospective recording.

4. Documentation accuracy is covered by the later TC-cli-0106 review plus direct inspection of the roadmap against confirmed merge metadata. The reviewer must check every document listed in `docs.md`, including the narrow PR #17 content-identical observation behavior and the distinction between the one Kimi discovery pass and its cold-resume defect. The reviewer must also check that Cursor, Gemini, and Grok remain unexercised and that no text claims a new post-fix Kimi run, publication, release, deployment, or human acceptance. Native structural checks validate file and contract shape, not these factual claims.

5. Final verification is covered by the focused Rust tests, a later passing TC-cli-0105, the later passing TC-cli-0106, bundle checks, and the complete Fledge verify lane on the final content. The final environment must report HI 0.8.0, Fledge 1.7.2, and SpecSync 6.1.1. BUG-0022 and BUG-0023 must remain open and explicitly out of scope. A scope review must confirm no new HI criterion, requirement ID, host enum value, integration, runtime, service, canonical contract delta, or native `.chough` affected path.

## TC-cli-0105 definition and procedure

Define TC-cli-0105 as an active functional automated case linked to FLOW-13 and LIFE-3, REQ-cli-003 and REQ-cli-007, and paths `src/main.rs` and `tests/stages.rs`. Its environments are `local-macos-arm64` and `ci-linux`, its artifact is `test-log`, and its note identifies BUG-0020. Bind it with `cargo-test` to target `stages` and exactly one test named `done_archived_native_guidance_drives_next_and_status`. The Unix test can use the existing native stub and isolated fixture without changing the shared fixture module.

Before the fix, run the case and retain its failed observation. The original archived-guidance assertion must fail because `next_valid_action` repeats the checkpoint's stale review and ship text. Link that observation to BUG-0020.

After the fix, run the focused Rust test and confirm every assertion without changing the case definition. After all content edits, rerun the same unchanged case definition for the final observation. For each of `next` and `status`, assert all of the following:

- With native state `archived` and native next action `stub next action for archived`, the valid action is `Continue done: stub next action for archived`, not the stale checkpoint action.
- `session_next_action` still equals the original checkpoint value, and reading either command does not rewrite the session.
- A paused checkpoint reports the pause and resume guidance even when native guidance is present. An active checkpoint with an awaiting-human entry reports that human gate instead. Neither path falls through to the native archived action.
- With native guidance unavailable, and with an archived response lacking a nonempty string next action, the valid action is exactly `Done checkpoint has no archived native next action; inspect native SpecSync status before further lifecycle action.` It does not contain the stale review or ship instruction.

Retain the passing observation only after all content edits are complete, link it to BUG-0020, and confirm that its observed time is strictly later than the retained failure.

## TC-cli-0106 definition and procedure

Define TC-cli-0106 as an active functional agent-assisted case linked to FLOW-13, FLOW-1, and LIFE-4, plus REQ-cli-007 and REQ-cli-016, and the canonical skill, affected documentation, and cold-start artifact paths. Its environments are `local-macos-arm64` for the original observation and `repository-text` for the retrospective documentation review. Its artifacts are `test-log` and `reviewer-note`.

Its setup names the durable concise log and base `5af3f78`. Its steps require the reviewer to extract the original host facts, inspect the guidance available to the original run, compare the corrected canonical and installed guidance and affected documents, and state the review limits. Its expected outcomes are:

- The original log supports the one run's discovery, cold-resume, command, timing, bug, and toolchain facts without enlarging them.
- Current guidance gives the `openai` to `.agents` installation mapping for Kimi and does not invent a `kimi` host.
- The canonical and installed guidance and the affected documents are consistent, including their roadmap and observation-reuse statements.
- The evidence distinguishes the original run from a later documentation review and records no human decision on anyone's behalf.

For the original observation, attach the concise cold-start log and a reviewer note. Give the original-run fact and evidence-limit outcomes a pass, but fail the installation-guidance and cross-document consistency outcomes because that guidance was absent at base. Link the resulting failed observation to BUG-0020 and BUG-0021, and append it to BUG-0022 and BUG-0023 as supporting evidence without changing their scope or status. It is not rewritten as a pass merely because Kimi discovery itself succeeded.

After the guidance and all listed documents are corrected, perform a second agent-assisted review against the same expected outcomes. Attach a new reviewer note and the original concise log, record all outcomes as pass only if the text supports them, and state explicitly that the pass is a documentation review with no new Kimi execution. Link this later pass to BUG-0021 and confirm strict failure-before-pass ordering.

## Cold-start artifact review

Condense rather than copy the 28,454-line scratch log. The concise log must retain the following observed facts:

- Kimi Code CLI 0.41.0, Darwin arm64, environment `local-macos-arm64`, worktree `.worktrees/kimi-host-cold-start`, branch `kimi/feat/kimi-host-cold-start`, and base `5af3f78`.
- The supplied task named the repository, objective, `AGENTS.md`, and canonical skill. Kimi also auto-discovered and loaded `.agents/skills/chough-workflow/SKILL.md` as a Project skill.
- `cargo build --locked` passed in 10 seconds. `chough next` emitted 11,567 JSON lines in 16 seconds with stage `done`, native state `archived`, and stale review and ship guidance. Native status confirmed `archived` and advised merging the PR if it was still open. `chough status` emitted about 11,567 lines in 36 seconds.
- Skill status was current, case definitions were valid, and the baseline verify lane completed 9 steps in 114 seconds. That baseline used global HI 0.5.0 and Fledge 1.8.0 with SpecSync 6.1.1, so it is not final CI-toolchain evidence.
- BUG-0020 and BUG-0021 are in scope. BUG-0022 records the initial tool drift and BUG-0023 records the oversized output; both remain out of scope. Release-verified HI 0.8.0 and Fledge 1.7.2 were later placed under `target/tmp/ci-tools`, with SpecSync 6.1.1 available.
- No approval, review, release, deployment, or other human decision was recorded by the agent. The artifact states the one-run and no-fresh-rerun limits.

## Final CI-matching verification

Make every source, test, case, skill, and documentation edit before this final sequence. Put the release-verified tools first on PATH and confirm all three versions before running checks:

```sh
export PATH="$PWD/target/tmp/ci-tools:$PATH"
hi --version
fledge --version
specsync --version
cargo test --locked --test stages done_archived_native_guidance_drives_next_and_status -- --exact
cargo test --locked --test optional skills_install_per_host_and_update_only_unmodified_files -- --exact
fledge run build
./target/debug/chough cases run TC-cli-0105 --environment local-macos-arm64
fledge run cases
./target/debug/chough skill status
cmp skills/chough-workflow/SKILL.md .agents/skills/chough-workflow/SKILL.md
cmp skills/chough-workflow/SKILL.md .claude/skills/chough-workflow/SKILL.md
fledge run fmt
fledge run test
fledge run lint
fledge lanes run verify
```

Record the TC-cli-0106 retrospective pass with `chough cases record` after reviewing the final documents. Ensure any final case observation is followed only by Chough-owned evidence or bug bookkeeping, not by another fingerprinted content edit. If any source, test, case, skill, native artifact, or documentation file changes afterward, rerun the affected cases and the full verify lane on that new final content.

The final report must name actual observed results and limitations. It must not convert the earlier mismatched-tool baseline into CI-matching evidence, claim hosted CI without a hosted run, close BUG-0022 or BUG-0023 without their own observed fixes, or claim a fresh Kimi run, human review, release, or deployment.

## Where these lessons go

- `specs/cli/context.md`
