---
change: correct-archived-resume-guidance-and-record-the-kimi-cold-start
artifact: tasks
---

# Tasks

Human scope approval is recorded. The checked items are completed implementation work. Chough-owned case definitions, artifacts, observations, and bug updates remain records owned by Chough; do not add them as native SpecSync affected paths.

## Reproduce and define the regressions

- [x] Add TC-cli-0105 to `.chough/cases/cli.cases.toml` with the automated `cargo-test` binding and traceability specified in `testing.md`. Add `done_archived_native_guidance_drives_next_and_status` to `tests/stages.rs`, covering both `next` and `status`, archived native guidance, paused and awaiting-human precedence, unavailable and missing native guidance, unchanged `session_next_action`, and no session writes. Run it against the unfixed `src/main.rs`, retain the failed observation, and link it to BUG-0020 before changing the implementation.

- [x] Add TC-cli-0106 as the functional agent-assisted retrospective case specified in `testing.md`, with `local-macos-arm64` and `repository-text` environments and explicit expected outcomes. Review the existing `.chough/artifacts/kimi-cold-start.log` and base guidance, record the original failed observation with the concise log and reviewer note, and link it to BUG-0020 and BUG-0021 before editing guidance. Append it as supporting evidence to BUG-0022 and BUG-0023 while keeping them open and out of scope.

## Correct archived resume guidance

- [x] In `src/main.rs`, change only the fallback action selection for an active schema 2 checkpoint at `done` bound to an archived native change. When `native_summary.next_action` is a nonempty string, derive `next_valid_action` from it. Keep paused and awaiting-human branches ahead of this logic, and preserve the existing blocking-bug and gate behavior. When native guidance is unavailable, undecodable, or lacks a nonempty next action, emit a safe native-status fallback instead of the stale checkpoint review or ship instruction. Leave `session_next_action`, the stored checkpoint, the response schema, and all other stages unchanged.

- [x] Run the focused stages test and confirm that the original archived-action assertion now passes without editing the test expectation. Do not add a lifecycle writer, a second native state, output reduction for BUG-0023, or a change for the BUG-0001 config comment that is already fixed on origin/main.

## Correct and protect the Kimi guidance

- [x] Update the Host use section of `skills/chough-workflow/SKILL.md` to state that Kimi Code CLI discovers `.agents/skills/chough-workflow/SKILL.md`, installed with the existing `chough init --host openai` mapping. Keep the instruction to confirm actual host discovery and the plain-Markdown fallback. Do not add `kimi` to `HOSTS`, the CLI value parser, or any host enum.

- [x] Copy the canonical `SKILL.md` bytes exactly to `.agents/skills/chough-workflow/SKILL.md` and `.claude/skills/chough-workflow/SKILL.md`. Add the pre-change canonical digest `d350cc91271e6d942bef516a2b2ea8a6009697a2e0d034b9ff48c8591be2ee5c` to `KNOWN_PREVIOUS` in `src/skill.rs`, identifying base `5af3f78` as the superseded version. Do not change unrelated known digests or update rules.

- [x] Extend `skills_install_per_host_and_update_only_unmodified_files` in `tests/optional.rs` with frozen pre-Kimi `SKILL.md` bytes kept inline in that test, so no new affected fixture path is needed. Assert that this exact prior copy is `known-outdated` and is updated to the canonical bytes, an independently edited copy remains `edited` and `preserved-edit`, both installed bundles match every canonical bundle file byte for byte, and subsequent skill status is `current`. Retain the existing unknown-host assertion and missing-file coverage.

## Update records and documentation

- [x] Apply every documentation edit enumerated in `docs.md` to `README.md`, `docs/ROADMAP.md`, `docs/integrations.md`, `docs/walkthrough.md`, `docs/exercises/cold-start.md`, `specs/cli/testing.md`, and `specs/cli/tasks.md`. Use the confirmed merge commits and dates listed there. Preserve the distinction between the original Kimi run, the later code regression, and the retrospective documentation pass. Do not claim a fresh post-fix Kimi run or any release or deployment.

- [x] Update the CLI testing companion's requirement matrix and case summary for TC-cli-0105 and TC-cli-0106. Keep historical observation sections as history, add only facts supported by the concise Kimi log and its evidence, and leave final verification results to actual evidence records rather than prewriting a pass.

- [x] Review the final diff for scope. Confirm no new HI criterion, requirement ID, canonical contract delta, host enum value, integration, runtime, service, or native `.chough` affected path was introduced. Confirm BUG-0022 and BUG-0023 remain open and that no BUG-0001 source edit was made.

## Final evidence and human closeout

After all implementation boxes and content edits are complete, run the focused tests, bundle comparisons, skill status, case checks, and final CI-matching sequence in `testing.md` with HI 0.8.0, Fledge 1.7.2, and SpecSync 6.1.1. Record the later TC-cli-0105 pass and link it to BUG-0020. Perform and record the later TC-cli-0106 documentation pass and link it to BUG-0021. Verify that each pass is strictly later than its retained failure before using the normal Chough bug procedure to mark either bug verified.

At the scope gate, checkpoint, commit the draft definition and cold-start records, push the branch, and open a draft PR titled `Draft: Kimi host cold start and roadmap refresh`. After approved implementation and testing, update the draft with changes, cold-start findings, an evidence table, deviations, and limits. Present the implementation, final observations, remaining open bugs, and the one-run and no-fresh-rerun limits for human implementation review. Never mark the PR ready without instruction. The human review remains a native gate; no agent may record it on the human's behalf. Native review and archival, merge, and any later release or deployment remain separate authorized steps, not side effects of this plan. If final content changes after the observations, rerun the affected cases and complete verify lane before presenting the result.
