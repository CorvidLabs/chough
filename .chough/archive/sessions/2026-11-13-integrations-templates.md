---
kind: session
id: 2026-11-13-integrations-templates
aliases: [integrations-templates]
objective: "#219: integrations/ ships versioned example hc templates for Claude Code, Codex, Kimi, Grok and a local agent, with a README saying Chough has no runtime dependency on hc, Herdr or Rune (REQ-cli-065)"
intent_scope: LIFE
modules: [cli]
stage: done
stage_status: archived
iteration: 1
native_change: 219-life-24-integrations-ships-versioned-example-hc-templates-for-claude-code-codex-kimi-grok-and-a-local-agent-a
criteria: [LIFE-24, LIFE-22]
requirements: [REQ-cli-065]
cases:
  - TC-cli-integrations-templates
  - TC-cli-integrations-manifest
  - TC-cli-integrations-digest-detects-change
  - TC-cli-integrations-no-runtime-dependency
  - TC-cli-integrations-readme
  - TC-cli-integrations-skill-file-exists-after-init
files:
  - tests/integrations.rs
  - specs/cli/cli.spec.md
  - integrations/README.md
  - integrations/templates.toml
  - integrations/claude-code/hc-template.md
  - integrations/codex/hc-template.md
  - integrations/kimi/hc-template.md
  - integrations/grok/hc-template.md
  - integrations/local/hc-template.md
  - docs/integrations.md
  - CHANGELOG.md
observations:
  - 2026-11-13-tc-cli-integrations-templates
  - 2026-11-13-tc-cli-integrations-manifest
  - 2026-11-13-tc-cli-integrations-digest-detects-change
  - 2026-11-13-tc-cli-integrations-no-runtime-dependency
  - 2026-11-13-tc-cli-integrations-readme
  - 2026-11-13-tc-cli-integrations-templates-853b5f
  - 2026-11-13-tc-cli-integrations-manifest-8480d6
  - 2026-11-13-tc-cli-integrations-digest-detects-change-75ba7d
  - 2026-11-13-tc-cli-integrations-no-runtime-dependency-819e19
  - 2026-11-13-tc-cli-integrations-readme-6e82a9
  - 2026-11-13-tc-cli-integrations-templates-11a405
  - 2026-11-13-tc-cli-integrations-manifest-62440e
  - 2026-11-13-tc-cli-integrations-digest-detects-change-3f83e7
  - 2026-11-13-tc-cli-integrations-no-runtime-dependency-30a5bd
  - 2026-11-13-tc-cli-integrations-readme-3d43b8
  - 2026-11-13-the-full-test-suite-passed-740-of-740-across-49-targets-cargo
  - 2026-11-13-tc-cli-integrations-manifest-1e9fe6
  - 2026-11-13-tc-cli-integrations-digest-detects-change-401e9b
  - 2026-11-13-tc-cli-integrations-skill-file-exists-after-init
  - 2026-11-13-tc-cli-integrations-templates-ed769d
  - 2026-11-13-tc-cli-integrations-manifest-51d9b1
  - 2026-11-13-tc-cli-integrations-digest-detects-change-a1771f
  - 2026-11-13-tc-cli-integrations-no-runtime-dependency-f79cdd
  - 2026-11-13-tc-cli-integrations-readme-0e6fc0
  - 2026-11-13-tc-cli-integrations-skill-file-exists-after-init-bda2c8
  - 2026-11-13-the-full-test-suite-passed-741-of-741-across-49-targets-cargo
  - 2026-11-13-tc-cli-integrations-templates-a34ade
  - 2026-11-13-tc-cli-integrations-manifest-1fbb4b
  - 2026-11-13-tc-cli-integrations-digest-detects-change-0d3a56
  - 2026-11-13-tc-cli-integrations-no-runtime-dependency-c74660
  - 2026-11-13-tc-cli-integrations-readme-c629fa
  - 2026-11-13-tc-cli-integrations-skill-file-exists-after-init-fd8cfb
  - 2026-11-13-the-full-test-suite-passed-743-of-743-across-49-targets-cargo
bugs:
  - 2026-11-13-the-grok-template-opens-a-skill-file-its-own-chough-init-does
  - 2026-11-13-a-changed-integrations-template-passes-with-its-version
closed: 2026-11-14
outcome: done
---
# #219: integrations/ ships versioned example hc templates for Claude Code, Codex, Kimi, Grok and a local agent, with a README saying Chough has no runtime dependency on hc, Herdr or Rune (REQ-cli-065)

## Goal

#219: integrations/ ships versioned example hc templates for Claude Code, Codex, Kimi, Grok and a local agent, with a README saying Chough has no runtime dependency on hc, Herdr or Rune (REQ-cli-065)

## Where it stands

- REQ-cli-065: integrations/ with five templates, templates.toml and the README; 5 of 5 integrations tests and 19 of 19 docs tests pass locally

## Decisions and why

- Templates for Claude Code, Codex, Kimi, Grok and a local agent, the local one marked unqualified until #220 (Leif, Q4, 2026-11-13T03:30Z, relayed by t370; rejected: adding Antigravity, and shipping only Claude Code, Codex and Kimi for now)
- Layout integrations/<host>/hc-template.md for claude-code, codex, kimi, grok and local, agreed with ch10-harness (t38q, 2026-11-13), whose #218 adapter.sh sits beside each; every template keeps exactly hc's name, description and command front matter because the harness reads it as is
- Versions live in integrations/templates.toml with each template's SHA-256, not in template front matter: whether hc accepts extra front-matter keys could not be probed without writing to the machine's shared hc template folder, and the harness needs only the three keys
- Requirement ID REQ-cli-065 (t382, records owner, 2026-11-13)
- Leif decided "Approve the intent: integrations/ ships versioned example hc templates for five hosts (#219, LIFE-24)": "Claude, Codex, Kimi, Grok + local (Recommended)" (coordinator chat (Claude Code, hc t370), 2026-11-13T03:30Z, answering ch10-flake's design question Q4 'Which hosts get an example template?' for #219 (local marked unqualified until #220); relayed by claude-code-t370); recorded by claude-code-t38r; a relayed claim, not authenticated
- Local template command stays the placeholder chough-local-agent, status unqualified; t38q sets the real command and bumps its version and digest in #220 (t38q, 2026-11-13; Leif chose a generic local setup, LIFE-29). The harness reads only adapter.sh and hc-template.md, so templates.toml is this lane's
- Leif decided "Approve the scope of 219-life-24-integrations-ships-versioned-example-hc-templates-for-claude-code-codex-kimi-grok-and-a-local-agent-a": "Claude, Codex, Kimi, Grok + local (Recommended)" (coordinator chat (Claude Code, hc t370), 2026-11-13T03:30Z, answering ch10-flake's design question Q4 'Which hosts get an example template?' for #219 (local marked unqualified until #220); this change's scope is that design's first #219 PR: the folder, five templates, the manifest and the README, REQ-cli-065; relayed by claude-code-t370); recorded by claude-code-t38r; a relayed claim, not authenticated; ran specsync change approve 219-life-24-integrations-ships-versioned-example-hc-templates-for-claude-code-codex-kimi-grok-and-a-local-agent-a as 0xLeif
- TC-cli-integrations-no-runtime-dependency's red run at ced59395 failed on a defect in the test itself: it banned the word hc in skills/, stricter than REQ-cli-065, while the skill rightly names an actor's hc terminal (REQ-history-002). The fix narrows the skill check to Herdr and Rune. That case is a guard on existing code, so it has no red-then-green; the other four went red then green
- TC-cli-integrations-digest-detects-change now unit-tests the manifest checker (version_problems) on made-up data, so it passed at the red commit 2cf5facb by construction; the red-first proof for the version rule is TC-cli-integrations-manifest, red at 2cf5facb
- Grok's template uses --host openai, the shared .agents copy, not --host grok: --host grok installs only .grok/skills, while a project set up for any other host has .agents/skills; one row of the approved design table said grok (reported to t370)
- Leif decided "Approve the narrowed scope of 219-life-24-integrations-ships-versioned-example-hc-templates-for-claude-code-codex-kimi-grok-and-a-local-agent-a": "Claude, Codex, Kimi, Grok + local (Recommended)" (coordinator chat (Claude Code, hc t370), 2026-11-13T03:30Z, answering ch10-flake's design question Q4 'Which hosts get an example template?' for #219; re-recorded after the send-back for REQ-cli-065's narrowed version wording (t384's re-review of #253): the coordinator claude-code-t370 checked the narrowed wording against this pick and judged it aligned on 2026-11-13 after Leif's 14:14Z approvals, under Leif's standing rule that the coordinator approves aligned requirement wording (coordinator chat, 2026-11-11); relayed by claude-code-t370); recorded by claude-code-t38r; a relayed claim, not authenticated; ran specsync change approve 219-life-24-integrations-ships-versioned-example-hc-templates-for-claude-code-codex-kimi-grok-and-a-local-agent-a as 0xLeif
- 0xLeif decided "Implementation review of 219-life-24-integrations-ships-versioned-example-hc-templates-for-claude-code-codex-kimi-grok-and-a-local-agent-a": "0xLeif ran: specsync change review 219-life-24-integrations-ships-versioned-example-hc-templates-for-claude-code-codex-kimi-grok-and-a-local-agent-a --reviewer 0xLeif --verdict pass" (0xLeif's own terminal in the Claude Code desktop app, 2026-11-14; the native review is already recorded at 8306f4c); recorded by claude-code; a relayed claim, not authenticated; no native review ran (--no-native)

## What was verified

- 24 earlier runs of the cases below are kept in this story's receipts, not listed here.
- The full test suite passed 740 of 740 across 49 targets (cargo test --locked --no-fail-fast, under the heavy-tests lock) at a442a2ac, #219's head with main (#248) merged and REQ-cli-065 materialized; clippy on the integrations test with -D warnings and rustfmt --check were clean ^2026-11-13-the-full-test-suite-passed-740-of-740-across-49-targets-cargo
- The full test suite passed 741 of 741 across 49 targets (cargo test --locked --no-fail-fast, under the heavy-tests lock) at dbe09d42, #253's head after t384's two must-fixes; clippy -D warnings on the integrations test and rustfmt --check were clean ^2026-11-13-the-full-test-suite-passed-741-of-741-across-49-targets-cargo
- Ran cargo test --locked --test integrations -- --exact each_template_has_hcs_front_matter_resumes_with_chough_next_and_holds_no_local_path for TC-cli-integrations-templates: passed (exit 0, tests run 1). — 9f8f2fe7f034 ^2026-11-13-tc-cli-integrations-templates-a34ade
- Ran cargo test --locked --test integrations -- --exact the_manifest_versions_every_template_with_its_digest for TC-cli-integrations-manifest: passed (exit 0, tests run 1). — 3a032147d366 ^2026-11-13-tc-cli-integrations-manifest-1fbb4b
- Ran cargo test --locked --test integrations -- --exact a_changed_template_fails_until_a_new_version_lists_its_digest for TC-cli-integrations-digest-detects-change: passed (exit 0, tests run 1). — b8a762f950c1 ^2026-11-13-tc-cli-integrations-digest-detects-change-0d3a56
- Ran cargo test --locked --test integrations -- --exact chough_never_starts_hc_herdr_or_rune_and_its_skill_never_names_them for TC-cli-integrations-no-runtime-dependency: passed (exit 0, tests run 1). — 7c5c8b8e6b9c ^2026-11-13-tc-cli-integrations-no-runtime-dependency-c74660
- Ran cargo test --locked --test integrations -- --exact the_readme_names_every_template_and_says_chough_has_no_runtime_dependency for TC-cli-integrations-readme: passed (exit 0, tests run 1). — 8bd06d0ec36e ^2026-11-13-tc-cli-integrations-readme-c629fa
- Ran cargo test --locked --test integrations -- --exact each_templates_skill_file_exists_after_its_own_chough_init for TC-cli-integrations-skill-file-exists-after-init: passed (exit 0, tests run 1). — 45d91e55a199 ^2026-11-13-tc-cli-integrations-skill-file-exists-after-init-fd8cfb
- The full test suite passed 743 of 743 across 49 targets (cargo test --locked --no-fail-fast, under the heavy-tests lock) at 5219fe8d, #253's head with main (#242) merged and the narrowed REQ-cli-065 materialized as cli version 90; the six integrations cases passed at the same content ^2026-11-13-the-full-test-suite-passed-743-of-743-across-49-targets-cargo

## Bugs

- [the Grok template opens a skill file its own chough init does not create](../../bugs/2026-11-13-the-grok-template-opens-a-skill-file-its-own-chough-init-does.md): verified
- [a changed integrations template passes with its version unchanged](../../bugs/2026-11-13-a-changed-integrations-template-passes-with-its-version.md): verified

## Open threads

- REQ-cli-065 rewording (t384 re-review of #253, wording only): is the delta in the coordinator's queue judged aligned? Pending t370; the README already says what the tests enforce (resolved: Aligned (t370, 2026-11-13, after Leif's 14:14Z approvals): same meaning, the overclaim removed and review covering in-place edits; send back and re-record the approval with Leif's Q4 words plus a note that claude-code-t370 checked the narrowed wording)

## Timeline

- 2026-11-13T03:53:10Z · start · intent → intent · #219: integrations/ ships versioned example hc templates for Claude Code, Codex, Kimi, Grok and a local agent, with a README saying Chough has no runtime dependency on hc, Herdr or Rune (REQ-cli-065) ^t-07efc9
  - revision: 63200149608d01550722a2033cadc83103c69c9a
  - by: claude-code (hc t38r)
- 2026-11-13T03:53:10Z · advance · intent → design ·  ^t-da0183
  - revision: 63200149608d01550722a2033cadc83103c69c9a
  - by: claude-code (hc t38r)
- 2026-11-13T03:53:11Z · skip · design → plan · No visuals or interactions change: Markdown templates, a manifest, a README and a test ^t-764aed
  - revision: 63200149608d01550722a2033cadc83103c69c9a
  - by: claude-code (hc t38r)
- 2026-11-13T03:53:18Z · back · plan → intent · Advanced by mistake: the intent gate's checkpoint was refused (home-path leak in a decision) and a grep in the command chain hid the refusal; redo the intent gate before leaving intent ^t-0e156d
  - revision: 63200149608d01550722a2033cadc83103c69c9a
  - refresh: intent: HI criteria and FRD; append new criteria, never rewrite approved ones, design: canonical prototype and companion; their human approval covers the old version, plan: native definition; scope or contract changes need native reapproval
  - by: claude-code (hc t38r)
- 2026-11-13T03:53:27Z · gate-opened · intent → intent · Approve the intent: integrations/ ships versioned example hc templates for five hosts (#219, LIFE-24) ^t-9b5dd3
  - revision: 63200149608d01550722a2033cadc83103c69c9a
  - holds: intent
  - drives: none
  - by: claude-code (hc t38r)
- 2026-11-13T03:53:37Z · gate-closed · intent → intent · Approve the intent: integrations/ ships versioned example hc templates for five hosts (#219, LIFE-24) ^t-c345dc
  - revision: 63200149608d01550722a2033cadc83103c69c9a
  - relayed_by:
    - decided_by: Leif
    - words: Claude, Codex, Kimi, Grok + local (Recommended)
    - where: coordinator chat (Claude Code, hc t370), 2026-11-13T03:30Z, answering ch10-flake's design question Q4 'Which hosts get an example template?' for #219 (local marked unqualified until #220); relayed by claude-code-t370
    - recorded_by: claude-code-t38r
  - by: claude-code (hc t38r)
- 2026-11-13T03:53:37Z · advance · intent → design ·  ^t-3517c8
  - revision: 63200149608d01550722a2033cadc83103c69c9a
  - by: claude-code (hc t38r)
- 2026-11-13T03:53:37Z · skip · design → plan · No visuals or interactions change: Markdown templates, a manifest, a README and a test ^t-e6431c
  - revision: 63200149608d01550722a2033cadc83103c69c9a
  - by: claude-code (hc t38r)
- 2026-11-13T03:55:24Z · gate-opened · plan → plan · Approve the scope of 219-life-24-integrations-ships-versioned-example-hc-templates-for-claude-code-codex-kimi-grok-and-a-local-agent-a ^t-9befd6
  - native_change: 219-life-24-integrations-ships-versioned-example-hc-templates-for-claude-code-codex-kimi-grok-and-a-local-agent-a
  - revision: 63200149608d01550722a2033cadc83103c69c9a
  - holds: plan
  - drives: approve
  - by: claude-code (hc t38r)
- 2026-11-13T03:55:24Z · gate-closed · plan → plan · Approve the scope of 219-life-24-integrations-ships-versioned-example-hc-templates-for-claude-code-codex-kimi-grok-and-a-local-agent-a ^t-86caf9
  - native_change: 219-life-24-integrations-ships-versioned-example-hc-templates-for-claude-code-codex-kimi-grok-and-a-local-agent-a
  - revision: 63200149608d01550722a2033cadc83103c69c9a
  - relayed_by:
    - decided_by: Leif
    - words: Claude, Codex, Kimi, Grok + local (Recommended)
    - where: coordinator chat (Claude Code, hc t370), 2026-11-13T03:30Z, answering ch10-flake's design question Q4 'Which hosts get an example template?' for #219 (local marked unqualified until #220); this change's scope is that design's first #219 PR: the folder, five templates, the manifest and the README, REQ-cli-065; relayed by claude-code-t370
    - recorded_by: claude-code-t38r
    - native: specsync change approve 219-life-24-integrations-ships-versioned-example-hc-templates-for-claude-code-codex-kimi-grok-and-a-local-agent-a
    - actor: 0xLeif
  - by: claude-code (hc t38r)
- 2026-11-13T03:55:31Z · advance · plan → build · native:approved ^t-d0600f
  - native_change: 219-life-24-integrations-ships-versioned-example-hc-templates-for-claude-code-codex-kimi-grok-and-a-local-agent-a
  - gate: native:approved
  - revision: 63200149608d01550722a2033cadc83103c69c9a
  - by: claude-code (hc t38r)
- 2026-11-13T04:14:28Z · advance · build → test · native:approved ^t-a4ce06
  - native_change: 219-life-24-integrations-ships-versioned-example-hc-templates-for-claude-code-codex-kimi-grok-and-a-local-agent-a
  - gate: native:approved
  - revision: f50da0d5b50721e81bcce0bbfd19a046f469027c
  - by: claude-code (hc t38r)
- 2026-11-13T04:14:29Z · advance · test → accept · native:approved ^t-9c3178
  - native_change: 219-life-24-integrations-ships-versioned-example-hc-templates-for-claude-code-codex-kimi-grok-and-a-local-agent-a
  - gate: native:approved
  - revision: f50da0d5b50721e81bcce0bbfd19a046f469027c
  - by: claude-code (hc t38r)
- 2026-11-13T05:03:50Z · gate-opened · accept → accept · Implementation review of 219-life-24-integrations-ships-versioned-example-hc-templates-for-claude-code-codex-kimi-grok-and-a-local-agent-a ^t-c0d3da
  - native_change: 219-life-24-integrations-ships-versioned-example-hc-templates-for-claude-code-codex-kimi-grok-and-a-local-agent-a
  - revision: c56dd78a35fbd42d94da891f30b0f016a890c6f8
  - holds: accept
  - drives: review
  - by: claude-code (hc t38r)
- 2026-11-13T05:34:34Z · gate-closed · accept → accept · Implementation review of 219-life-24-integrations-ships-versioned-example-hc-templates-for-claude-code-codex-kimi-grok-and-a-local-agent-a ^t-3896da
  - native_change: 219-life-24-integrations-ships-versioned-example-hc-templates-for-claude-code-codex-kimi-grok-and-a-local-agent-a
  - revision: 81eccae15ccf4aac8ba89692cabb5aa59740b782
  - by: claude-code (hc t38r)
- 2026-11-13T05:34:36Z · back · accept → build · t384's independent review of #253 at 81eccae1: the Grok template names a skill file its own chough init does not create, and the tests do not make a changed template raise its version ^t-0a67a5
  - native_change: 219-life-24-integrations-ships-versioned-example-hc-templates-for-claude-code-codex-kimi-grok-and-a-local-agent-a
  - gate: native:verifying
  - revision: 81eccae15ccf4aac8ba89692cabb5aa59740b782
  - refresh: build: implementation against the current definition, test: rerun affected cases; earlier observations may be stale, accept: native verification and the human implementation review, checkpoint references: 5 case(s) and 16 observation(s) to re-check with `chough status`
  - by: claude-code (hc t38r)
- 2026-11-13T06:20:14Z · advance · build → test · native:verifying ^t-8b5524
  - native_change: 219-life-24-integrations-ships-versioned-example-hc-templates-for-claude-code-codex-kimi-grok-and-a-local-agent-a
  - gate: native:verifying
  - revision: dbe09d422be76fe12904652e04ba10d8e0d3fdeb
  - by: claude-code (hc t38r)
- 2026-11-13T06:20:17Z · advance · test → accept · native:verifying ^t-d2e66b
  - native_change: 219-life-24-integrations-ships-versioned-example-hc-templates-for-claude-code-codex-kimi-grok-and-a-local-agent-a
  - gate: native:verifying
  - revision: dbe09d422be76fe12904652e04ba10d8e0d3fdeb
  - by: claude-code (hc t38r)
- 2026-11-13T06:47:55Z · gate-opened · accept → accept · Implementation review of 219-life-24-integrations-ships-versioned-example-hc-templates-for-claude-code-codex-kimi-grok-and-a-local-agent-a ^t-1448a9
  - native_change: 219-life-24-integrations-ships-versioned-example-hc-templates-for-claude-code-codex-kimi-grok-and-a-local-agent-a
  - revision: 2450471835f4326725e0babe19cedb032ea0197d
  - holds: accept
  - drives: review
  - by: claude-code (hc t38r)
- 2026-11-13T14:19:53Z · gate-closed · accept → accept · Implementation review of 219-life-24-integrations-ships-versioned-example-hc-templates-for-claude-code-codex-kimi-grok-and-a-local-agent-a ^t-27a155
  - native_change: 219-life-24-integrations-ships-versioned-example-hc-templates-for-claude-code-codex-kimi-grok-and-a-local-agent-a
  - revision: 0cf360ba26c14864a749d4558c01004a3b44102d
  - by: claude-code (hc t38r)
- 2026-11-13T14:19:55Z · back · accept → plan · REQ-cli-065's wording overclaims what the version tests enforce (t384's re-review of #253, mutation B); t370 judged the narrowed wording aligned, so the definition goes back for that rewording ^t-0ee6cd
  - native_change: 219-life-24-integrations-ships-versioned-example-hc-templates-for-claude-code-codex-kimi-grok-and-a-local-agent-a
  - revision: 0cf360ba26c14864a749d4558c01004a3b44102d
  - refresh: plan: native definition; scope or contract changes need native reapproval, build: implementation against the current definition, test: rerun affected cases; earlier observations may be stale, accept: native verification and the human implementation review, checkpoint references: 6 case(s) and 26 observation(s) to re-check with `chough status`, native: the definition is verifying; changing intent, contract, acceptance criteria or scope needs native reapproval. Follow `specsync change status 219-life-24-integrations-ships-versioned-example-hc-templates-for-claude-code-codex-kimi-grok-and-a-local-agent-a`; Chough does not change native state.
  - by: claude-code (hc t38r)
- 2026-11-13T14:20:45Z · gate-opened · plan → plan · Approve the narrowed scope of 219-life-24-integrations-ships-versioned-example-hc-templates-for-claude-code-codex-kimi-grok-and-a-local-agent-a ^t-065ec8
  - native_change: 219-life-24-integrations-ships-versioned-example-hc-templates-for-claude-code-codex-kimi-grok-and-a-local-agent-a
  - revision: 0cf360ba26c14864a749d4558c01004a3b44102d
  - holds: plan
  - drives: approve
  - by: claude-code (hc t38r)
- 2026-11-13T14:20:45Z · direct-native-write · plan → plan · 219-life-24-integrations-ships-versioned-example-hc-templates-for-claude-code-codex-kimi-grok-and-a-local-agent-a's lifecycle records changed outside Chough: state.json changed, approvals.json changed; state verifying → draft ^t-1adfad
  - native_change: 219-life-24-integrations-ships-versioned-example-hc-templates-for-claude-code-codex-kimi-grok-and-a-local-agent-a
  - revision: 0cf360ba26c14864a749d4558c01004a3b44102d
  - direct_write:
    - change: 219-life-24-integrations-ships-versioned-example-hc-templates-for-claude-code-codex-kimi-grok-and-a-local-agent-a
    - files:
      - state.json: changed
      - approvals.json: changed
    - before_state: verifying
    - after_state: draft
  - by: claude-code (hc t38r)
- 2026-11-13T14:20:45Z · gate-closed · plan → plan · Approve the narrowed scope of 219-life-24-integrations-ships-versioned-example-hc-templates-for-claude-code-codex-kimi-grok-and-a-local-agent-a ^t-d53899
  - native_change: 219-life-24-integrations-ships-versioned-example-hc-templates-for-claude-code-codex-kimi-grok-and-a-local-agent-a
  - revision: 0cf360ba26c14864a749d4558c01004a3b44102d
  - relayed_by:
    - decided_by: Leif
    - words: Claude, Codex, Kimi, Grok + local (Recommended)
    - where: coordinator chat (Claude Code, hc t370), 2026-11-13T03:30Z, answering ch10-flake's design question Q4 'Which hosts get an example template?' for #219; re-recorded after the send-back for REQ-cli-065's narrowed version wording (t384's re-review of #253): the coordinator claude-code-t370 checked the narrowed wording against this pick and judged it aligned on 2026-11-13 after Leif's 14:14Z approvals, under Leif's standing rule that the coordinator approves aligned requirement wording (coordinator chat, 2026-11-11); relayed by claude-code-t370
    - recorded_by: claude-code-t38r
    - native: specsync change approve 219-life-24-integrations-ships-versioned-example-hc-templates-for-claude-code-codex-kimi-grok-and-a-local-agent-a
    - actor: 0xLeif
  - by: claude-code (hc t38r)
- 2026-11-13T14:21:19Z · advance · plan → build · native:approved ^t-99a06a
  - native_change: 219-life-24-integrations-ships-versioned-example-hc-templates-for-claude-code-codex-kimi-grok-and-a-local-agent-a
  - gate: native:approved
  - revision: 5219fe8daacaa2baeb870782955b1f06090c072f
  - by: claude-code (hc t38r)
- 2026-11-13T14:21:19Z · advance · build → test · native:approved ^t-2b5040
  - native_change: 219-life-24-integrations-ships-versioned-example-hc-templates-for-claude-code-codex-kimi-grok-and-a-local-agent-a
  - gate: native:approved
  - revision: 5219fe8daacaa2baeb870782955b1f06090c072f
  - by: claude-code (hc t38r)
- 2026-11-13T14:21:20Z · advance · test → accept · native:approved ^t-73a726
  - native_change: 219-life-24-integrations-ships-versioned-example-hc-templates-for-claude-code-codex-kimi-grok-and-a-local-agent-a
  - gate: native:approved
  - revision: 5219fe8daacaa2baeb870782955b1f06090c072f
  - by: claude-code (hc t38r)
- 2026-11-13T14:56:13Z · gate-opened · accept → accept · Implementation review of 219-life-24-integrations-ships-versioned-example-hc-templates-for-claude-code-codex-kimi-grok-and-a-local-agent-a ^t-92ca46
  - native_change: 219-life-24-integrations-ships-versioned-example-hc-templates-for-claude-code-codex-kimi-grok-and-a-local-agent-a
  - revision: d1569a4467e95f264d077b9507993811c3b54f61
  - holds: accept
  - drives: review
  - by: claude-code (hc t38r)
- 2026-11-14T17:34:00Z · gate-closed · accept → accept · Implementation review of 219-life-24-integrations-ships-versioned-example-hc-templates-for-claude-code-codex-kimi-grok-and-a-local-agent-a ^t-4431bd
  - native_change: 219-life-24-integrations-ships-versioned-example-hc-templates-for-claude-code-codex-kimi-grok-and-a-local-agent-a
  - revision: 31996ff0a43d0674c8df80f9a9631c6d3ecbffa3
  - relayed_by:
    - decided_by: 0xLeif
    - words: 0xLeif ran: specsync change review 219-life-24-integrations-ships-versioned-example-hc-templates-for-claude-code-codex-kimi-grok-and-a-local-agent-a --reviewer 0xLeif --verdict pass
    - where: 0xLeif's own terminal in the Claude Code desktop app, 2026-11-14; the native review is already recorded at 8306f4c
    - recorded_by: claude-code
  - by: claude-code
- 2026-11-14T17:34:01Z · advance · accept → done · native:archived ^t-571007
  - native_change: 219-life-24-integrations-ships-versioned-example-hc-templates-for-claude-code-codex-kimi-grok-and-a-local-agent-a
  - gate: native:archived
  - revision: 31996ff0a43d0674c8df80f9a9631c6d3ecbffa3
  - by: claude-code

Closed 2026-11-14: done.

## Receipts

```yaml
receipts:
  - id: 2026-11-13-tc-cli-integrations-templates
    claim: "Ran cargo test --locked --test integrations -- --exact each_template_has_hcs_front_matter_resumes_with_chough_next_and_holds_no_local_path for TC-cli-integrations-templates: failed (exit 101, tests run 1)."
    result: failed
    command: cargo test --locked --test integrations -- --exact each_template_has_hcs_front_matter_resumes_with_chough_next_and_holds_no_local_path
    revision: ced59395ac51df57dfd36bdfd15c2258859e98b1
    environment: local-macos-arm64
    content: ddb19099f22805755afc1bc77de18bf519dd8c72639381e6de65e4167a01045b
    fingerprint: sha256:291f54571fa630717caa67140a54f92685f2b75d1b0fb777a60decbf3b578025
    case: TC-cli-integrations-templates
    case_digest: 3666c27de6e873eab3f46399adac32565efbb8d0d2b3a7c6cb12645fd38ec4ff
    observed: 2026-11-13T04:12:36Z
    duration_seconds: 1
    actor: claude-code (hc t38r)
  - id: 2026-11-13-tc-cli-integrations-manifest
    claim: "Ran cargo test --locked --test integrations -- --exact the_manifest_versions_every_template_with_its_digest for TC-cli-integrations-manifest: failed (exit 101, tests run 1)."
    result: failed
    command: cargo test --locked --test integrations -- --exact the_manifest_versions_every_template_with_its_digest
    revision: ced59395ac51df57dfd36bdfd15c2258859e98b1
    environment: local-macos-arm64
    content: ddb19099f22805755afc1bc77de18bf519dd8c72639381e6de65e4167a01045b
    fingerprint: sha256:5c8fb3c0f8fb509a6d7fee2bfb76282769c4870a34648a99fc0978e27ad29de0
    case: TC-cli-integrations-manifest
    case_digest: f2dc5b12c78e1adc195ab83332e1a2f823e4fd34da08456cd7f51f72af9bdf29
    observed: 2026-11-13T04:12:39Z
    duration_seconds: 0
    actor: claude-code (hc t38r)
  - id: 2026-11-13-tc-cli-integrations-digest-detects-change
    claim: "Ran cargo test --locked --test integrations -- --exact a_changed_template_byte_no_longer_matches_its_listed_digest for TC-cli-integrations-digest-detects-change: failed (exit 101, tests run 1)."
    result: failed
    command: cargo test --locked --test integrations -- --exact a_changed_template_byte_no_longer_matches_its_listed_digest
    revision: ced59395ac51df57dfd36bdfd15c2258859e98b1
    environment: local-macos-arm64
    content: ddb19099f22805755afc1bc77de18bf519dd8c72639381e6de65e4167a01045b
    fingerprint: sha256:bd713a180810297ea0ce6d871e8b3efb75f0542ef0016bf49927f5973f5f76ce
    case: TC-cli-integrations-digest-detects-change
    case_digest: adaaf77380423e15ce74e2525868d5f59885efefd6aff95edb4a5c54c1fc2fbd
    observed: 2026-11-13T04:12:41Z
    duration_seconds: 0
    actor: claude-code (hc t38r)
  - id: 2026-11-13-tc-cli-integrations-no-runtime-dependency
    claim: "Ran cargo test --locked --test integrations -- --exact chough_never_starts_hc_herdr_or_rune_and_its_skill_never_names_them for TC-cli-integrations-no-runtime-dependency: failed (exit 101, tests run 1)."
    result: failed
    command: cargo test --locked --test integrations -- --exact chough_never_starts_hc_herdr_or_rune_and_its_skill_never_names_them
    revision: ced59395ac51df57dfd36bdfd15c2258859e98b1
    environment: local-macos-arm64
    content: ddb19099f22805755afc1bc77de18bf519dd8c72639381e6de65e4167a01045b
    fingerprint: sha256:b0ac88fe3b767c4fd9523065f4bd2dea3ca07f539915c5f9fc26606b640f8d31
    case: TC-cli-integrations-no-runtime-dependency
    case_digest: f7f634f29e7ef6671beb8a7fec69289f22983cf4ba5d71e54bf2be64e7d9daf5
    observed: 2026-11-13T04:12:43Z
    duration_seconds: 0
    actor: claude-code (hc t38r)
  - id: 2026-11-13-tc-cli-integrations-readme
    claim: "Ran cargo test --locked --test integrations -- --exact the_readme_names_every_template_and_says_chough_has_no_runtime_dependency for TC-cli-integrations-readme: failed (exit 101, tests run 1)."
    result: failed
    command: cargo test --locked --test integrations -- --exact the_readme_names_every_template_and_says_chough_has_no_runtime_dependency
    revision: ced59395ac51df57dfd36bdfd15c2258859e98b1
    environment: local-macos-arm64
    content: ddb19099f22805755afc1bc77de18bf519dd8c72639381e6de65e4167a01045b
    fingerprint: sha256:88b8c12affee411b6fa561ba157f23d0f8162a90ec8224b5a2b6da981cd4586a
    case: TC-cli-integrations-readme
    case_digest: 8789bd2fb23f68bf4f254a914934273ebf75ccf35a2b5a3b0bb3cd0c336c0440
    observed: 2026-11-13T04:12:45Z
    duration_seconds: 0
    actor: claude-code (hc t38r)
  - id: 2026-11-13-tc-cli-integrations-templates-853b5f
    claim: "Ran cargo test --locked --test integrations -- --exact each_template_has_hcs_front_matter_resumes_with_chough_next_and_holds_no_local_path for TC-cli-integrations-templates: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test integrations -- --exact each_template_has_hcs_front_matter_resumes_with_chough_next_and_holds_no_local_path
    revision: 12f2cafe777a48ef6e6cb92c9435997ab5b4e0d2
    environment: local-macos-arm64
    content: dd44a2f14e6358d057ef934649d20ff96745157b0280619cc43ef2f0e973b2c4
    fingerprint: sha256:6808d5d41d08f689f36dac300d50be2d13bd401f2d8a45b9f969556c83f2106c
    case: TC-cli-integrations-templates
    case_digest: 3666c27de6e873eab3f46399adac32565efbb8d0d2b3a7c6cb12645fd38ec4ff
    observed: 2026-11-13T04:14:00Z
    duration_seconds: 0
    actor: claude-code (hc t38r)
  - id: 2026-11-13-tc-cli-integrations-manifest-8480d6
    claim: "Ran cargo test --locked --test integrations -- --exact the_manifest_versions_every_template_with_its_digest for TC-cli-integrations-manifest: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test integrations -- --exact the_manifest_versions_every_template_with_its_digest
    revision: 12f2cafe777a48ef6e6cb92c9435997ab5b4e0d2
    environment: local-macos-arm64
    content: dd44a2f14e6358d057ef934649d20ff96745157b0280619cc43ef2f0e973b2c4
    fingerprint: sha256:d7b652aded16e962b2e1342ee23b840cbae2278d262b4a6ed2d6ff1372514fc4
    case: TC-cli-integrations-manifest
    case_digest: f2dc5b12c78e1adc195ab83332e1a2f823e4fd34da08456cd7f51f72af9bdf29
    observed: 2026-11-13T04:14:04Z
    duration_seconds: 0
    actor: claude-code (hc t38r)
  - id: 2026-11-13-tc-cli-integrations-digest-detects-change-75ba7d
    claim: "Ran cargo test --locked --test integrations -- --exact a_changed_template_byte_no_longer_matches_its_listed_digest for TC-cli-integrations-digest-detects-change: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test integrations -- --exact a_changed_template_byte_no_longer_matches_its_listed_digest
    revision: 12f2cafe777a48ef6e6cb92c9435997ab5b4e0d2
    environment: local-macos-arm64
    content: dd44a2f14e6358d057ef934649d20ff96745157b0280619cc43ef2f0e973b2c4
    fingerprint: sha256:106eafea73112b45d19769a577c5c5314a64c34612a3399e58cbdbb2b0cafecc
    case: TC-cli-integrations-digest-detects-change
    case_digest: adaaf77380423e15ce74e2525868d5f59885efefd6aff95edb4a5c54c1fc2fbd
    observed: 2026-11-13T04:14:08Z
    duration_seconds: 0
    actor: claude-code (hc t38r)
  - id: 2026-11-13-tc-cli-integrations-no-runtime-dependency-819e19
    claim: "Ran cargo test --locked --test integrations -- --exact chough_never_starts_hc_herdr_or_rune_and_its_skill_never_names_them for TC-cli-integrations-no-runtime-dependency: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test integrations -- --exact chough_never_starts_hc_herdr_or_rune_and_its_skill_never_names_them
    revision: 12f2cafe777a48ef6e6cb92c9435997ab5b4e0d2
    environment: local-macos-arm64
    content: dd44a2f14e6358d057ef934649d20ff96745157b0280619cc43ef2f0e973b2c4
    fingerprint: sha256:aedf96ca8855423ef03e533464becdbe919981be1a84584cfaaef9c3c8d99e0b
    case: TC-cli-integrations-no-runtime-dependency
    case_digest: f7f634f29e7ef6671beb8a7fec69289f22983cf4ba5d71e54bf2be64e7d9daf5
    observed: 2026-11-13T04:14:12Z
    duration_seconds: 0
    actor: claude-code (hc t38r)
  - id: 2026-11-13-tc-cli-integrations-readme-6e82a9
    claim: "Ran cargo test --locked --test integrations -- --exact the_readme_names_every_template_and_says_chough_has_no_runtime_dependency for TC-cli-integrations-readme: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test integrations -- --exact the_readme_names_every_template_and_says_chough_has_no_runtime_dependency
    revision: 12f2cafe777a48ef6e6cb92c9435997ab5b4e0d2
    environment: local-macos-arm64
    content: dd44a2f14e6358d057ef934649d20ff96745157b0280619cc43ef2f0e973b2c4
    fingerprint: sha256:b909d193093c51fb3856f7acebf5a3fbdc0c363b0a06b8c207515c8349460f6f
    case: TC-cli-integrations-readme
    case_digest: 8789bd2fb23f68bf4f254a914934273ebf75ccf35a2b5a3b0bb3cd0c336c0440
    observed: 2026-11-13T04:14:15Z
    duration_seconds: 0
    actor: claude-code (hc t38r)
  - id: 2026-11-13-tc-cli-integrations-templates-11a405
    claim: "Ran cargo test --locked --test integrations -- --exact each_template_has_hcs_front_matter_resumes_with_chough_next_and_holds_no_local_path for TC-cli-integrations-templates: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test integrations -- --exact each_template_has_hcs_front_matter_resumes_with_chough_next_and_holds_no_local_path
    revision: a442a2acc4a23948c031c36675861ba57f04c980
    environment: local-macos-arm64
    content: 2fb922c6ab66ac421f2cbd88feaa0ec2fd87004f296b827945dfb087f7231437
    fingerprint: sha256:f5ac40617a76a92475c13f4a74e473619b0804c20cbcf001b60a4027619377c8
    case: TC-cli-integrations-templates
    case_digest: 3666c27de6e873eab3f46399adac32565efbb8d0d2b3a7c6cb12645fd38ec4ff
    observed: 2026-11-13T04:54:34Z
    duration_seconds: 3
    actor: claude-code (hc t38r)
  - id: 2026-11-13-tc-cli-integrations-manifest-62440e
    claim: "Ran cargo test --locked --test integrations -- --exact the_manifest_versions_every_template_with_its_digest for TC-cli-integrations-manifest: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test integrations -- --exact the_manifest_versions_every_template_with_its_digest
    revision: a442a2acc4a23948c031c36675861ba57f04c980
    environment: local-macos-arm64
    content: 2fb922c6ab66ac421f2cbd88feaa0ec2fd87004f296b827945dfb087f7231437
    fingerprint: sha256:f831f20d1c54af1064ed6aa9a10531c17819946697f70ee4015e489812d8f32e
    case: TC-cli-integrations-manifest
    case_digest: f2dc5b12c78e1adc195ab83332e1a2f823e4fd34da08456cd7f51f72af9bdf29
    observed: 2026-11-13T04:54:36Z
    duration_seconds: 0
    actor: claude-code (hc t38r)
  - id: 2026-11-13-tc-cli-integrations-digest-detects-change-3f83e7
    claim: "Ran cargo test --locked --test integrations -- --exact a_changed_template_byte_no_longer_matches_its_listed_digest for TC-cli-integrations-digest-detects-change: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test integrations -- --exact a_changed_template_byte_no_longer_matches_its_listed_digest
    revision: a442a2acc4a23948c031c36675861ba57f04c980
    environment: local-macos-arm64
    content: 2fb922c6ab66ac421f2cbd88feaa0ec2fd87004f296b827945dfb087f7231437
    fingerprint: sha256:d2221a8234592f3cba2a3c4615842519f1e0d1086507ad3e4b82539c0deef22c
    case: TC-cli-integrations-digest-detects-change
    case_digest: adaaf77380423e15ce74e2525868d5f59885efefd6aff95edb4a5c54c1fc2fbd
    observed: 2026-11-13T04:54:38Z
    duration_seconds: 0
    actor: claude-code (hc t38r)
  - id: 2026-11-13-tc-cli-integrations-no-runtime-dependency-30a5bd
    claim: "Ran cargo test --locked --test integrations -- --exact chough_never_starts_hc_herdr_or_rune_and_its_skill_never_names_them for TC-cli-integrations-no-runtime-dependency: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test integrations -- --exact chough_never_starts_hc_herdr_or_rune_and_its_skill_never_names_them
    revision: a442a2acc4a23948c031c36675861ba57f04c980
    environment: local-macos-arm64
    content: 2fb922c6ab66ac421f2cbd88feaa0ec2fd87004f296b827945dfb087f7231437
    fingerprint: sha256:066f0024547be19cac62553e56cfe92c70a6161aa7104ef41280610f22d2f4a8
    case: TC-cli-integrations-no-runtime-dependency
    case_digest: f7f634f29e7ef6671beb8a7fec69289f22983cf4ba5d71e54bf2be64e7d9daf5
    observed: 2026-11-13T04:54:41Z
    duration_seconds: 0
    actor: claude-code (hc t38r)
  - id: 2026-11-13-tc-cli-integrations-readme-3d43b8
    claim: "Ran cargo test --locked --test integrations -- --exact the_readme_names_every_template_and_says_chough_has_no_runtime_dependency for TC-cli-integrations-readme: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test integrations -- --exact the_readme_names_every_template_and_says_chough_has_no_runtime_dependency
    revision: a442a2acc4a23948c031c36675861ba57f04c980
    environment: local-macos-arm64
    content: 2fb922c6ab66ac421f2cbd88feaa0ec2fd87004f296b827945dfb087f7231437
    fingerprint: sha256:bd0969136be94666e1e375e2d891e5ee068864d415d197b517dfdf7e6ec73d6a
    case: TC-cli-integrations-readme
    case_digest: 8789bd2fb23f68bf4f254a914934273ebf75ccf35a2b5a3b0bb3cd0c336c0440
    observed: 2026-11-13T04:54:43Z
    duration_seconds: 0
    actor: claude-code (hc t38r)
  - id: 2026-11-13-the-full-test-suite-passed-740-of-740-across-49-targets-cargo
    claim: "The full test suite passed 740 of 740 across 49 targets (cargo test --locked --no-fail-fast, under the heavy-tests lock) at a442a2ac, #219's head with main (#248) merged and REQ-cli-065 materialized; clippy on the integrations test with -D warnings and rustfmt --check were clean"
    result: passed
    revision: a442a2acc4a23948c031c36675861ba57f04c980
    environment: local-macos-arm64
    content: e4635ebf3f79f4f68a66e0b586fa29e2568f544b8874addb64602529ea1095c9
    observed: 2026-11-13T05:03:10Z
    actor: claude-code (hc t38r)
  - id: 2026-11-13-tc-cli-integrations-manifest-1e9fe6
    claim: "Ran cargo test --locked --test integrations -- --exact the_manifest_versions_every_template_with_its_digest for TC-cli-integrations-manifest: failed (exit 101, tests run 1)."
    result: failed
    command: cargo test --locked --test integrations -- --exact the_manifest_versions_every_template_with_its_digest
    revision: 2cf5facb7c792081f8f73104eb458f4cda4408b9
    environment: local-macos-arm64
    content: e87d689143e165f16ea94afa6e1b6bb1edd9f5a40749321661f272dfb48435ef
    fingerprint: sha256:45e6e01e8b441fdd996febdf3a99aa2898b3864da3d6a41420400772a0af9aa5
    case: TC-cli-integrations-manifest
    case_digest: f2dc5b12c78e1adc195ab83332e1a2f823e4fd34da08456cd7f51f72af9bdf29
    observed: 2026-11-13T06:07:11Z
    duration_seconds: 0
    actor: claude-code (hc t38r)
  - id: 2026-11-13-tc-cli-integrations-digest-detects-change-401e9b
    claim: "Ran cargo test --locked --test integrations -- --exact a_changed_template_fails_until_a_new_version_lists_its_digest for TC-cli-integrations-digest-detects-change: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test integrations -- --exact a_changed_template_fails_until_a_new_version_lists_its_digest
    revision: 2cf5facb7c792081f8f73104eb458f4cda4408b9
    environment: local-macos-arm64
    content: e87d689143e165f16ea94afa6e1b6bb1edd9f5a40749321661f272dfb48435ef
    fingerprint: sha256:628af92c583d8de7465d575ed820d58bb2b9dcf92fe57adea6bbc42ada44870a
    case: TC-cli-integrations-digest-detects-change
    case_digest: aadc547832dd06247ad6e92c9b62442095049f56faf523a6ecc0c319c57cb466
    observed: 2026-11-13T06:07:13Z
    duration_seconds: 0
    actor: claude-code (hc t38r)
  - id: 2026-11-13-tc-cli-integrations-skill-file-exists-after-init
    claim: "Ran cargo test --locked --test integrations -- --exact each_templates_skill_file_exists_after_its_own_chough_init for TC-cli-integrations-skill-file-exists-after-init: failed (exit 101, tests run 1)."
    result: failed
    command: cargo test --locked --test integrations -- --exact each_templates_skill_file_exists_after_its_own_chough_init
    revision: 2cf5facb7c792081f8f73104eb458f4cda4408b9
    environment: local-macos-arm64
    content: e87d689143e165f16ea94afa6e1b6bb1edd9f5a40749321661f272dfb48435ef
    fingerprint: sha256:62b609932de40b316cf52cb653524b467359093b602061934e21ced09153877b
    case: TC-cli-integrations-skill-file-exists-after-init
    case_digest: 23f650f2098c747e712edc8684d3bda735c787475ceb29c5d44401c9fe1ad543
    observed: 2026-11-13T06:07:17Z
    duration_seconds: 2
    actor: claude-code (hc t38r)
  - id: 2026-11-13-tc-cli-integrations-templates-ed769d
    claim: "Ran cargo test --locked --test integrations -- --exact each_template_has_hcs_front_matter_resumes_with_chough_next_and_holds_no_local_path for TC-cli-integrations-templates: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test integrations -- --exact each_template_has_hcs_front_matter_resumes_with_chough_next_and_holds_no_local_path
    revision: c0e3c340ae4fce048f37442831d242d42eeeef28
    environment: local-macos-arm64
    content: 1012c8e4277ff01558fc9805d340ea45db95300ec2124f149e6c4af0ae6b866a
    fingerprint: sha256:a3f68efe12aab162e390a0360524da4c497fb26cc3f1293d98ae93a5c8cf7764
    case: TC-cli-integrations-templates
    case_digest: 3666c27de6e873eab3f46399adac32565efbb8d0d2b3a7c6cb12645fd38ec4ff
    observed: 2026-11-13T06:19:30Z
    duration_seconds: 1
    actor: claude-code (hc t38r)
  - id: 2026-11-13-tc-cli-integrations-manifest-51d9b1
    claim: "Ran cargo test --locked --test integrations -- --exact the_manifest_versions_every_template_with_its_digest for TC-cli-integrations-manifest: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test integrations -- --exact the_manifest_versions_every_template_with_its_digest
    revision: c0e3c340ae4fce048f37442831d242d42eeeef28
    environment: local-macos-arm64
    content: 1012c8e4277ff01558fc9805d340ea45db95300ec2124f149e6c4af0ae6b866a
    fingerprint: sha256:ac55340fc51d4758ffa33af57bcd1c7fc993c4a2c07ba9642543d3a06d2a033b
    case: TC-cli-integrations-manifest
    case_digest: f2dc5b12c78e1adc195ab83332e1a2f823e4fd34da08456cd7f51f72af9bdf29
    observed: 2026-11-13T06:19:32Z
    duration_seconds: 0
    actor: claude-code (hc t38r)
  - id: 2026-11-13-tc-cli-integrations-digest-detects-change-a1771f
    claim: "Ran cargo test --locked --test integrations -- --exact a_changed_template_fails_until_a_new_version_lists_its_digest for TC-cli-integrations-digest-detects-change: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test integrations -- --exact a_changed_template_fails_until_a_new_version_lists_its_digest
    revision: c0e3c340ae4fce048f37442831d242d42eeeef28
    environment: local-macos-arm64
    content: 1012c8e4277ff01558fc9805d340ea45db95300ec2124f149e6c4af0ae6b866a
    fingerprint: sha256:20c40b996156011ba721e05a91f8624b0e2acb9f93364802ec75c06d8a53bcb5
    case: TC-cli-integrations-digest-detects-change
    case_digest: aadc547832dd06247ad6e92c9b62442095049f56faf523a6ecc0c319c57cb466
    observed: 2026-11-13T06:19:35Z
    duration_seconds: 0
    actor: claude-code (hc t38r)
  - id: 2026-11-13-tc-cli-integrations-no-runtime-dependency-f79cdd
    claim: "Ran cargo test --locked --test integrations -- --exact chough_never_starts_hc_herdr_or_rune_and_its_skill_never_names_them for TC-cli-integrations-no-runtime-dependency: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test integrations -- --exact chough_never_starts_hc_herdr_or_rune_and_its_skill_never_names_them
    revision: c0e3c340ae4fce048f37442831d242d42eeeef28
    environment: local-macos-arm64
    content: 1012c8e4277ff01558fc9805d340ea45db95300ec2124f149e6c4af0ae6b866a
    fingerprint: sha256:994a2da66035c1731debe782f93f2aacc823dc9cbb3ea3d02269aae63c0e5ae2
    case: TC-cli-integrations-no-runtime-dependency
    case_digest: f7f634f29e7ef6671beb8a7fec69289f22983cf4ba5d71e54bf2be64e7d9daf5
    observed: 2026-11-13T06:19:38Z
    duration_seconds: 0
    actor: claude-code (hc t38r)
  - id: 2026-11-13-tc-cli-integrations-readme-0e6fc0
    claim: "Ran cargo test --locked --test integrations -- --exact the_readme_names_every_template_and_says_chough_has_no_runtime_dependency for TC-cli-integrations-readme: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test integrations -- --exact the_readme_names_every_template_and_says_chough_has_no_runtime_dependency
    revision: c0e3c340ae4fce048f37442831d242d42eeeef28
    environment: local-macos-arm64
    content: 1012c8e4277ff01558fc9805d340ea45db95300ec2124f149e6c4af0ae6b866a
    fingerprint: sha256:58f514756322148ba5a4c97b7a5fc3354eecc713f36c4153616118a64f69fc47
    case: TC-cli-integrations-readme
    case_digest: 8789bd2fb23f68bf4f254a914934273ebf75ccf35a2b5a3b0bb3cd0c336c0440
    observed: 2026-11-13T06:19:41Z
    duration_seconds: 0
    actor: claude-code (hc t38r)
  - id: 2026-11-13-tc-cli-integrations-skill-file-exists-after-init-bda2c8
    claim: "Ran cargo test --locked --test integrations -- --exact each_templates_skill_file_exists_after_its_own_chough_init for TC-cli-integrations-skill-file-exists-after-init: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test integrations -- --exact each_templates_skill_file_exists_after_its_own_chough_init
    revision: c0e3c340ae4fce048f37442831d242d42eeeef28
    environment: local-macos-arm64
    content: 1012c8e4277ff01558fc9805d340ea45db95300ec2124f149e6c4af0ae6b866a
    fingerprint: sha256:8d8d4ff1ad0a9bb9e160071315434bb26d5e4e5a24a1375fc87f4b64ce2800c7
    case: TC-cli-integrations-skill-file-exists-after-init
    case_digest: 23f650f2098c747e712edc8684d3bda735c787475ceb29c5d44401c9fe1ad543
    observed: 2026-11-13T06:19:47Z
    duration_seconds: 3
    actor: claude-code (hc t38r)
  - id: 2026-11-13-the-full-test-suite-passed-741-of-741-across-49-targets-cargo
    claim: "The full test suite passed 741 of 741 across 49 targets (cargo test --locked --no-fail-fast, under the heavy-tests lock) at dbe09d42, #253's head after t384's two must-fixes; clippy -D warnings on the integrations test and rustfmt --check were clean"
    result: passed
    revision: dbe09d422be76fe12904652e04ba10d8e0d3fdeb
    environment: local-macos-arm64
    content: 1012c8e4277ff01558fc9805d340ea45db95300ec2124f149e6c4af0ae6b866a
    observed: 2026-11-13T06:47:47Z
    actor: claude-code (hc t38r)
  - id: 2026-11-13-tc-cli-integrations-templates-a34ade
    claim: "Ran cargo test --locked --test integrations -- --exact each_template_has_hcs_front_matter_resumes_with_chough_next_and_holds_no_local_path for TC-cli-integrations-templates: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test integrations -- --exact each_template_has_hcs_front_matter_resumes_with_chough_next_and_holds_no_local_path
    revision: 5219fe8daacaa2baeb870782955b1f06090c072f
    environment: local-macos-arm64
    content: f2e4d81afcb0f9ebc9fbccbb66beaa8e4ff94e07972e671050f8b067113aec53
    fingerprint: sha256:9f8f2fe7f03422907860103e34aab50e85b12b655e2af1f820f0d1192174c703
    case: TC-cli-integrations-templates
    case_digest: 3666c27de6e873eab3f46399adac32565efbb8d0d2b3a7c6cb12645fd38ec4ff
    observed: 2026-11-13T14:49:59Z
    duration_seconds: 5
    actor: claude-code (hc t38r)
  - id: 2026-11-13-tc-cli-integrations-manifest-1fbb4b
    claim: "Ran cargo test --locked --test integrations -- --exact the_manifest_versions_every_template_with_its_digest for TC-cli-integrations-manifest: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test integrations -- --exact the_manifest_versions_every_template_with_its_digest
    revision: 5219fe8daacaa2baeb870782955b1f06090c072f
    environment: local-macos-arm64
    content: f2e4d81afcb0f9ebc9fbccbb66beaa8e4ff94e07972e671050f8b067113aec53
    fingerprint: sha256:3a032147d36644f812486b419b20710a3749e377a8ba9599b43592d2ce2db937
    case: TC-cli-integrations-manifest
    case_digest: f2dc5b12c78e1adc195ab83332e1a2f823e4fd34da08456cd7f51f72af9bdf29
    observed: 2026-11-13T14:50:01Z
    duration_seconds: 0
    actor: claude-code (hc t38r)
  - id: 2026-11-13-tc-cli-integrations-digest-detects-change-0d3a56
    claim: "Ran cargo test --locked --test integrations -- --exact a_changed_template_fails_until_a_new_version_lists_its_digest for TC-cli-integrations-digest-detects-change: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test integrations -- --exact a_changed_template_fails_until_a_new_version_lists_its_digest
    revision: 5219fe8daacaa2baeb870782955b1f06090c072f
    environment: local-macos-arm64
    content: f2e4d81afcb0f9ebc9fbccbb66beaa8e4ff94e07972e671050f8b067113aec53
    fingerprint: sha256:b8a762f950c1b35eeee4db2f55e96cb2dbf1f025f08674b1d14488c2e2efd159
    case: TC-cli-integrations-digest-detects-change
    case_digest: aadc547832dd06247ad6e92c9b62442095049f56faf523a6ecc0c319c57cb466
    observed: 2026-11-13T14:50:03Z
    duration_seconds: 0
    actor: claude-code (hc t38r)
  - id: 2026-11-13-tc-cli-integrations-no-runtime-dependency-c74660
    claim: "Ran cargo test --locked --test integrations -- --exact chough_never_starts_hc_herdr_or_rune_and_its_skill_never_names_them for TC-cli-integrations-no-runtime-dependency: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test integrations -- --exact chough_never_starts_hc_herdr_or_rune_and_its_skill_never_names_them
    revision: 5219fe8daacaa2baeb870782955b1f06090c072f
    environment: local-macos-arm64
    content: f2e4d81afcb0f9ebc9fbccbb66beaa8e4ff94e07972e671050f8b067113aec53
    fingerprint: sha256:7c5c8b8e6b9cf93d63a5d4ae9d2d5996fd515d3f6e4a16f6ebf7d0d0a844d39a
    case: TC-cli-integrations-no-runtime-dependency
    case_digest: f7f634f29e7ef6671beb8a7fec69289f22983cf4ba5d71e54bf2be64e7d9daf5
    observed: 2026-11-13T14:50:05Z
    duration_seconds: 0
    actor: claude-code (hc t38r)
  - id: 2026-11-13-tc-cli-integrations-readme-c629fa
    claim: "Ran cargo test --locked --test integrations -- --exact the_readme_names_every_template_and_says_chough_has_no_runtime_dependency for TC-cli-integrations-readme: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test integrations -- --exact the_readme_names_every_template_and_says_chough_has_no_runtime_dependency
    revision: 5219fe8daacaa2baeb870782955b1f06090c072f
    environment: local-macos-arm64
    content: f2e4d81afcb0f9ebc9fbccbb66beaa8e4ff94e07972e671050f8b067113aec53
    fingerprint: sha256:8bd06d0ec36e5e50d408e3850f8b9fb73f97865ae95a6a3b8d15c7b6be4a147d
    case: TC-cli-integrations-readme
    case_digest: 8789bd2fb23f68bf4f254a914934273ebf75ccf35a2b5a3b0bb3cd0c336c0440
    observed: 2026-11-13T14:50:07Z
    duration_seconds: 0
    actor: claude-code (hc t38r)
  - id: 2026-11-13-tc-cli-integrations-skill-file-exists-after-init-fd8cfb
    claim: "Ran cargo test --locked --test integrations -- --exact each_templates_skill_file_exists_after_its_own_chough_init for TC-cli-integrations-skill-file-exists-after-init: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test integrations -- --exact each_templates_skill_file_exists_after_its_own_chough_init
    revision: 5219fe8daacaa2baeb870782955b1f06090c072f
    environment: local-macos-arm64
    content: f2e4d81afcb0f9ebc9fbccbb66beaa8e4ff94e07972e671050f8b067113aec53
    fingerprint: sha256:45d91e55a199c2a89a8f09c86ab203129022f0de9e4a888958d05c8ff098ec2c
    case: TC-cli-integrations-skill-file-exists-after-init
    case_digest: 23f650f2098c747e712edc8684d3bda735c787475ceb29c5d44401c9fe1ad543
    observed: 2026-11-13T14:50:11Z
    duration_seconds: 1
    actor: claude-code (hc t38r)
  - id: 2026-11-13-the-full-test-suite-passed-743-of-743-across-49-targets-cargo
    claim: "The full test suite passed 743 of 743 across 49 targets (cargo test --locked --no-fail-fast, under the heavy-tests lock) at 5219fe8d, #253's head with main (#242) merged and the narrowed REQ-cli-065 materialized as cli version 90; the six integrations cases passed at the same content"
    result: passed
    revision: 5219fe8daacaa2baeb870782955b1f06090c072f
    environment: local-macos-arm64
    content: f2e4d81afcb0f9ebc9fbccbb66beaa8e4ff94e07972e671050f8b067113aec53
    observed: 2026-11-13T14:55:58Z
    actor: claude-code (hc t38r)
```
