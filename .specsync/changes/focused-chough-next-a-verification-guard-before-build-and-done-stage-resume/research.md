---
change: focused-chough-next-a-verification-guard-before-build-and-done-stage-resume
artifact: research
---

# Research

All measurements on macOS arm64, debug build of origin/main ee6e3d2
(`./target/debug/chough`), specsync 6.1.1, on 2026-11-01.

## Current next and status output

- `chough next`, `chough next --json` and `chough status` are byte-identical
  (`cmp`): 410,822 bytes each; with `--environment macos`, 410,825 bytes.
- Top-level fields as printed (pretty JSON): `observations` 358,088 bytes (204
  records, every one `stale-revision-or-inputs`), `native_specsync` 18,153 (its raw
  `content` 16,495 characters), `observation_references` 7,612 (61 referenced, all
  `historical`), `checkpoint` 1,187, `open_bugs` 977, everything else under 300
  each. Compact serialization: 296,142 bytes in all, 269,255 for observations.
- Wall time: 15.0 to 15.6 seconds per call. A standalone `specsync change status
  <archived id> --json` takes 12.7 seconds and prints 56,836 bytes, so the native read
  dominates. At stage plan or accept, `status()` reads native status twice: once in
  `native_status` and again in `stage::build_gate` or `accept_gate`.
- The checkpoint is at stage done with an archived change; `next_valid_action` is
  `Continue done: specsync change review bundle-hygiene-... --reviewer 0xLeif, then
  ship`, the stale action PR #20 reported, while native status says `merge the PR on
  GitHub if it is still open`.
- Existing callers: tests read full-report fields through `next --json`
  (tests/cli.rs, tests/bugs.rs, tests/stages.rs, tests/timing.rs,
  tests/evidence.rs); examples/feature-bug-loop.sh and examples/visual-design.sh pipe
  `chough next` into `jq -r .next_valid_action`.

## What SpecSync 6.1.1 exposes for the guard

- `change status <id> --json`: `summary` has approval_valid, artifacts_complete,
  correction_count, correction_valid, definition_digest, handoff, id, next_action,
  scoped_review_current, state, strict_validation_required, title and
  verification_commands; `change` has affected_specs, affected_paths and
  no_spec_change. Nothing about delta bodies or their digests.
- Rehearsal in a disposable copy of examples/shipping (synthetic approver, never this
  repository): after `change approve`, appending a REQUIREMENT block to the delta left
  status `approved`, `approval_valid: true` and next action `run specsync change check`
  (0.37 seconds per status call). BUG-0002 reproduces on 6.1.1.
- `change check <id>` without `--commit` is not read-only. With the edited delta it
  refused (exit 1, 0.96 seconds) and wrote nothing. With the delta restored it refused
  again, exit 1, because tasks.md had unchecked boxes. With tasks checked and no
  implementation it exited 1 on failed verification after writing
  specs/shipping/requirements.md, specs/shipping/shipping.spec.md, verification.json,
  verification-attempts.json, state.json and change.md, and moved the change to
  `verifying` (3.9 seconds). Every refusal is exit 1, told apart only by stderr text.
  So no gate can use it: it mutates canonical specs and native records, it always
  refuses at plan to build (tasks are unchecked then by design), and on this
  repository it runs the configured verification commands.
- The approval ledger `.specsync/changes/<id>/approvals.json` records, on each
  definition approval, `approved_delta_digests`: module to SHA-256. In spec-sync
  v6.1.1 `delta_body_digests` (src/change/collect_requirement_ids.rs) frames, with
  `FramedDigest` (src/change/sdd_version.rs), the domain
  `specsync.approved-delta.v1`, then `module` and `body`, each as an 8-byte
  big-endian tag length, tag, 8-byte big-endian value length, value; the body has
  CRLF replaced by LF. `ensure_approved_delta_bodies_unchanged` compares the latest
  definition approval's map (`effective_definition_approval` takes the last
  `gate == "definition"` entry) with the current digests of every module in
  affected_specs, and refuses a module that differs, is missing or was not approved.
  A Python reimplementation of this framing matched all 7 recorded digests in this
  repository's archive.
- Ledger sizes: this repository's largest approvals.json is 20,306 bytes; spec-sync's
  own archive has one of 273,639 bytes. Chough's RECORD_LIMIT (64 KiB) is too small;
  SpecSync bounds change artifacts at 4 MiB (MAX_CHANGE_ARTIFACT_BYTES).
- Upstream fix: spec-sync PR #823 (status reports approval_valid false and names
  changed_approved_deltas) merged to spec-sync main on 2026-11-01; the latest release is
  still v6.1.1 (2026-10-15).

## Closed PR #20 and host mapping

- `gh pr diff 20` and the archived change `gemini-host-cold-start-and-roadmap-refresh`
  on branch gemini/feat/gemini-host-cold-start (head 53b60aa). Its log,
  `.chough/artifacts/gemini-cold-start.log`, is written by the agent: it names the host
  "Google Antigravity (Gemini 3.8 Flash High)", while the PR's cold-start document says
  "Gemini 2.5 Pro via Google Antigravity CLI"; no event log or transcript is attached.
  It reports that Antigravity listed the skill from `.agents/skills`, that `chough
  next` from done echoed the archived change's review step, and it suggested `chough
  stage reset --to plan` without `--reason` (clap requires it). The change was
  recorded no_spec_change and the PR was closed unmerged.
- src/skill.rs `HOSTS`: openai `.agents/skills`, claude `.claude/skills`, codex
  `.codex/skills`, cursor `.cursor/skills`, gemini `.gemini/skills`, grok
  `.grok/skills`, "matching the CorvidLabs skills installer". `skill status` with no
  `--host` lists each host whose directory exists, so `.agents/skills` is reported once,
  as openai, with its directory in the report.
