---
change: correct-archived-resume-guidance-and-record-the-kimi-cold-start
artifact: plan
---

# Plan

## Approach and approval gate

Use two narrow product fixes and one factual documentation and evidence refresh. First make archived native guidance authoritative for a done checkpoint without weakening higher-priority gates. Then document Kimi's existing `openai` to `.agents` mapping and make unedited prior skill copies safely updatable. Preserve the original cold-start as a failed resume observation with explicit partial successes, and use only retrospective documentation review afterward.

The native change is still a draft. Present this definition for one coherent human scope approval before product edits, regression execution, or native approval. The task that requested the planning work is not itself that approval. After approval, keep the definition frozen; any scope change returns to the human.

## Out of scope

- BUG-0022: changing or repairing global HI and Fledge installations. Record the drift, but use `target/tmp/ci-tools` for final CI-matching verification.
- BUG-0023: changing `next` or `status` output size, pagination, evidence retention, or command performance beyond the single next-action correction.
- BUG-0001: its configuration-comment issue is already fixed on origin/main, so no source edit or regression belongs here.
- A fresh post-fix Kimi cold start, or any claim that one occurred. TC-cli-0106 is retrospective review only.
- A `kimi` host enum or installer mapping, a new integration, model client, agent runtime, hosted service, or duplicate lifecycle state.
- New HI criteria, requirement IDs, canonical requirement or contract edits, and `.chough` native affected paths.
- Release, deployment, live-health claims, or agent-recorded human scope approval, implementation review, finalization, or archive.

BUG-0022 and BUG-0023 remain open and out of scope. Independent authorized work may record their evidence, but must not expand this change to fix them.

## Ownership and relative sizing

Planning ownership is separate from later implementation ownership. This writer owns only `context.md`, `design.md`, `requirements.md`, and `plan.md` in this change directory. Companion planning writers own `testing.md`, `tasks.md`, and `docs.md`. The native coordinator owns `change.md` and lifecycle state and presents the complete definition to the human.

After approval, use these implementation streams. Do not let two streams write the same file concurrently.

| Increment | Owner and files | Relative size | Main risk |
|---|---|---|---|
| 1. Original evidence and red regressions | Records owner for `.chough/cases/cli.cases.toml`, the concise log, observations, and bug links; CLI test owner for `tests/stages.rs` | Small to medium | Retrospective evidence could overstate what the original run proved |
| 2. Archived next-action fix | CLI owner for `src/main.rs` and `tests/stages.rs` | Small | Branch placement could bypass pause or human-gate precedence |
| 3. Kimi guidance and safe update | Skill owner for `src/skill.rs`, `tests/optional.rs`, and the canonical, `.agents`, and `.claude` `SKILL.md` files | Small | An inexact prior digest would classify an unedited copy as edited |
| 4. Host, roadmap, and companion docs | Docs owner for `README.md`, `docs/ROADMAP.md`, `docs/integrations.md`, `docs/walkthrough.md`, `docs/exercises/cold-start.md`, `specs/cli/testing.md`, and `specs/cli/tasks.md` | Small to medium | Merge dates, one-run limits, and content-identical freshness must stay factual |
| 5. Passing evidence and bug disposition | Records owner for observations, BUG-0020 and BUG-0021 links and status, and checkpoint references | Small | A pass must be strictly later than the latest linked failure |
| 6. Final integrated verification | Integrator for all final content and verification artifacts | Medium | The initial global tools do not match CI, and host copies can drift after editing |

The integrator may sequence streams 2 through 4 in parallel only after increment 1 is complete, then serialize the shared evidence and checkpoint writes. One person or agent may hold several owner roles, but the file boundaries and observation order still apply.

The read-only `chough timing` report completed in about 233 seconds and emitted 51,507 bytes. Its estimates are elapsed wall-clock history, not effort or deadlines: plan, test, and human gates have too few samples; build has a minimum of 1 second, median of 1 minute 5 seconds, and maximum of 4 minutes 4 seconds from 3 samples; accept has a minimum of 7 minutes 38 seconds, median of 28 minutes 27 seconds, and maximum of 42 minutes 30 seconds from 4 samples; bug fixes have a minimum of 45 seconds, median of 14 minutes 26 seconds, and maximum of 1 hour 32 seconds from 15 samples. The implementation remains relatively small, with final integrated verification the main uncertainty. The timing output size and duration are additional evidence for out-of-scope BUG-0023, not a reason to redesign timing here.

## Steps

1. Complete the native definition and obtain human scope approval. Confirm the exact precedence, fallback string, two case definitions, one-run evidence limit, roadmap corrections, and out-of-scope bugs are all visible in the reviewed artifacts. No product work precedes this gate.

2. Preserve the original evidence and establish red coverage before changing product code.
   - Review the records owner's existing `.chough/artifacts/kimi-cold-start.log` against the original scratch log rather than recreating it. Confirm that it retains the supplied task guidance, base revision, commands, salient results, timings, tool versions, four findings, and absence of agent-recorded human decisions.
   - Define TC-cli-0105 and TC-cli-0106. Add the TC-cli-0105 regression test to `tests/stages.rs` without changing `src/main.rs`.
   - Run TC-cli-0105 against the unfixed routing and record `failed`. Link that observation to BUG-0020.
   - Review the original log and still-unfixed guidance for TC-cli-0106. Record a failed agent-assisted observation with its one-run, supplied-path, initial-tool, and no-post-fix-rerun limitations. Link it to BUG-0020 and BUG-0021, and append it to BUG-0022 and BUG-0023 as supporting evidence without changing their scope or status.

3. Implement the shared `next` and `status` fix in `src/main.rs` exactly as `design.md` specifies. Preserve schema 1 behavior and every existing higher-priority branch. At `done`, return `Continue done: <native next-action>` for nonblank archived native guidance; otherwise return the specified native-status fallback and never the saved review-and-ship action. Leave `session_next_action` and the checkpoint unchanged. Run the unchanged TC-cli-0105 test and confirm coverage of both commands, awaiting-human precedence, paused precedence, missing native guidance, failed native reads, and retained diagnostics.

4. Implement the skill update.
   - Add the Kimi mapping paragraph to the canonical `skills/chough-workflow/SKILL.md` without adding a host value.
   - Add the base `5af3f78` digest `d350cc91271e6d942bef516a2b2ea8a6009697a2e0d034b9ff48c8591be2ee5c` for `SKILL.md` to `KNOWN_PREVIOUS`.
   - Expand the test bound to TC-cli-0037 with the exact base bytes, a digest assertion, known-outdated status, update to current, and preservation of a separate edited copy. Retain the older 0.1.0 fixture assertions.
   - Update the `.agents` and `.claude` copies through the new safe update path, compare all three files byte-for-byte, and check that skill status reports every file current for both hosts.

5. Correct the documentation as itemized in `design.md`. Use the GitHub-observed UTC dates: PRs #8 and #9 on 2026-10-31, and PRs #15, #16, and #17 on 2026-11-01. The Observation reuse row must say content-identical commits keep observations current because freshness compares content identity rather than revision. It must not restore the old unapproved label for implemented behavior, and it must still exclude ancestry checks and a separate reuse label. Keep Cursor, Gemini, and Grok unexercised, describe only one Kimi run, and make no release or deployment claim.

6. Complete passing observations after all relevant source and docs edits.
   - Record the later TC-cli-0105 pass and link it to BUG-0020. Mark BUG-0020 verified only if the existing latest-failure timestamp rule is satisfied.
   - Perform TC-cli-0106 as a retrospective review of the original concise log and final guidance. Record and link its later pass to BUG-0021, explicitly reviewing docs rather than claiming another Kimi run. Mark BUG-0021 verified only under the same ordering rule.
   - Keep BUG-0022 and BUG-0023 open and out of scope. Update checkpoint references through the records owner, without adding `.chough` to native affected paths.

7. Verify the final integrated content with the CI-matching tools. Put `target/tmp/ci-tools` first on `PATH`, then run `hi --version`, `fledge --version`, and `specsync --version` and confirm 0.8.0, 1.7.2, and 6.1.1 before recording any final result. Run:
   - focused TC-cli-0105 and TC-cli-0037 case executions on the unchanged definitions used for their earlier observations;
   - `fledge run fmt`, `fledge run test`, and `fledge run lint`;
   - `chough cases check`, byte comparisons of the three `SKILL.md` files, and `chough skill status` for both installed hosts;
   - `fledge lanes run verify` on the final content, expecting all 9 steps to pass;
   - a final text review that finds no stale Kimi-unexercised statements, no em dash or en dash in the changed skill bundle, and no human approval, fresh Kimi rerun, hosted CI, release, or deployment claim not supported by an observation.

Record final observations with their actual environments and tool versions. The initial 114-second baseline verify remains historical evidence from HI 0.5.0 and Fledge 1.8.0, not a substitute for this final lane. Local verification does not establish hosted CI or human acceptance.

8. At the scope gate, checkpoint, commit the completed draft definition and cold-start records, push the branch, and open a draft PR titled `Draft: Kimi host cold start and roadmap refresh`. After approved implementation and testing, update that draft PR with the changes, cold-start findings, evidence table, deviations, and limits, then present it for genuine human implementation review. Never mark it ready without instruction. Keep further native lifecycle actions under their actual human and native gates. No release follows automatically.

## Risks and controls

- Native guidance could conceal a higher-priority human gate if inserted too early. The ordered branch design and TC-cli-0105 precedence assertions prevent this.
- A missing native response could fall back to the stale checkpoint action. The dedicated `done` fallback covers every unusable native response, and the test asserts the stale text is absent while native diagnostics remain.
- Safe updates could overwrite user edits if the previous digest were broad or approximate. The exact base SHA-256 is tied only to `SKILL.md`, and the test retains a separate edited-file control.
- A retrospective pass could be mistaken for a new host exercise. TC-cli-0106's setup, verdict summary, artifacts, and limitations all say that it reviews the original log and final docs only.
- Parallel writes could conflict in `.chough` or the host copies. The records owner and skill owner serialize those paths, and the integrator verifies final bytes.
- Roadmap dates can differ between local commit display and GitHub merge display. Use the confirmed GitHub UTC merge dates and the named merge commits, not an inferred release date.
