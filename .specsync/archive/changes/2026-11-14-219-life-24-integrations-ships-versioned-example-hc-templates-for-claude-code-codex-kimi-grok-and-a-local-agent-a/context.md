---
change: 219-life-24-integrations-ships-versioned-example-hc-templates-for-claude-code-codex-kimi-grok-and-a-local-agent-a
artifact: context
---

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
