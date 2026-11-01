---
change: correct-archived-resume-guidance-and-record-the-kimi-cold-start
artifact: testing
---

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
