# Lesson bundle: 219-life-24-integrations-ships-versioned-example-hc-templates-for-claude-code-codex-kimi-grok-and-a-local-agent-a

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: #219 LIFE-24: integrations/ ships versioned example hc templates for Claude Code, Codex, Kimi, Grok and a local agent, a manifest with each template's version and digest, and a README; Chough never runs hc, Herdr or Rune
- **Kind**: Feature
- **Specs**: cli
- **Paths**: integrations/README.md, integrations/templates.toml, integrations/claude-code/hc-template.md, integrations/codex/hc-template.md, integrations/kimi/hc-template.md, integrations/grok/hc-template.md, integrations/local/hc-template.md, tests/integrations.rs, specs/cli/cli.spec.md, docs/integrations.md, CHANGELOG.md
- **Acceptance**: integrations/ holds a README and one hc template per host (claude-code, codex, kimi, grok, local) whose front matter is exactly name, description and command, whose body runs chough next and loads the skill file its own chough init --host creates, with no absolute or home path; integrations/templates.toml lists each with its init host, status (local unqualified until #220) and an ordered versions list with their SHA-256, and tests/integrations.rs fails while a template differs from its newest listed digest, its versions skip, or a digest repeats (an in-place edit of a listed entry is left to review); src/ never runs hc, Herdr or Rune and the skill never names Herdr or Rune (red then green in tests/integrations.rs)

## Evidence

- Verification commit: `8306f4c3d7314cab68fbed6ef6d0836a1d8bde07`
- Base commit: `63200149608d01550722a2033cadc83103c69c9a`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

Issue #219 is LIFE-24 (Leif, 2026-11-10 MVP interview): "Herdr/hc and Rune stay out of
Chough's core: Chough ships an optional integrations/ folder with versioned example hc
templates and a multi-agent coordination guide ...". This change ships the templates,
the manifest and the README. The coordination guide is the next PR.

On 2026-11-13 Leif answered ch10-flake's (claude-code, hc t38r) design question Q4 at
03:30Z, relayed by t370: "Claude, Codex, Kimi, Grok + local (Recommended)". The local
template is marked unqualified until #220. Rejected: adding Antigravity, and shipping
only Claude Code, Codex and Kimi now.

The layout was agreed with ch10-harness (t38q) for #218:
- `integrations/<host>/hc-template.md`, with the harness's `adapter.sh` beside it;
- the scenario and judge in `tools/host-matrix/`.

The harness reads each template as it is, so the front matter stays exactly hc's
`name`, `description` and `command`. Versions go in a manifest because whether hc
accepts extra front-matter keys could not be probed without writing to the shared hc
template folder.

The templates the fleet used locally before this change had two problems: they
pointed at home-folder paths (the Kimi one at an absolute binary path), and the Codex
one lacked hc's required `--no-daemon`. `src/actor.rs` reads the `HC_TERMINAL_ID`
variable only to name the actor on records; it never starts hc.

## From the change's design.md

# Design

The five template bodies share one text. Only the `--host` value differs, plus one
extra line for Claude Code, which does not discover `.agents/skills` on its own:

| Host | name | command | init host | status |
|---|---|---|---|---|
| claude-code | chough-claude | claude | claude | supported |
| codex | chough-codex | codex --no-daemon | openai | supported |
| kimi | chough-kimi | kimi | openai | supported |
| grok | chough-grok | grok | openai | supported |
| local | chough-local | chough-local-agent (replace) | openai | unqualified |

Codex, Kimi, Grok and the local agent use `--host openai`, the shared
`.agents/skills` copy that docs/integrations.md records as observed. A project set up
for any other host already has it, and `--host grok` would install only `.grok/skills`
(found in t384's review of #253). The local template's command is a placeholder until #220 picks
and qualifies a setup.

`integrations/templates.toml` (`schema = "chough-integrations/1"`) has one
`[[template]]` per host with `host`, `file`, `name`, `init_host`, `status` and an
ordered `versions` list of `{ version, sha256 }`. The newest entry is the file's digest,
the versions run 1, 2, 3 and no digest repeats. An in-place edit of a listed entry is
left to review. A test also runs a real `chough init` per template and checks the
skill file the template names. The test is `tests/integrations.rs`
(module cli). It reads the files, uses the `toml` and `sha2` crates, and takes
the accepted `--host` values from the built binary's `chough init --help`.

## From the change's testing.md

# Testing

`tests/integrations.rs` checks the templates' front matter, bodies and paths, the
manifest's entries and digests, that `src/` and `skills/` neither start nor name
the tools, and the README. It is red at the test commit and green at the fix.
Chough cases bind each test through `cargo-test`.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-cli-065 | `tests/integrations.rs`: five tests, one per acceptance criterion plus the README |

## Where these lessons go

- `specs/cli/cli.3md`
