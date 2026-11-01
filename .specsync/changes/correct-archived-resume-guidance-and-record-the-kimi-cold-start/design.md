---
change: correct-archived-resume-guidance-and-record-the-kimi-cold-start
artifact: design
---

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
