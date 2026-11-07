---
change: chough-gate-list-one-screen-gate-summaries-with-digests-and-one-reply-for-listed-gates-only-184-flow-19-flow-20
artifact: design
---

# Design

- **`chough gate list [--json]`** works over every open session (`.chough/sessions/*.json`, or the legacy checkpoint) and each of its `awaiting_human` gates, in session-key order then gate order. They are numbered 1..N.
- **Summary text** has at most 24 lines and 4,096 bytes, cut with a "… (cut)" line if longer. Its lines are:
  - `Gate: <gate>`
  - `Change: <id> — <title> (<state>)`
  - `What changed: <files> files, +<ins> −<del> since <base8>`, then up to five `  <path> +a −d` lines (from `git diff --numstat <base>..HEAD`);
  - `Tests: <p> passed, <f> failed, <n> not run`, plus `Failing: <ids>`. The cases are those in `.chough/cases/*.cases.toml` whose `requirements` include a `REQUIREMENT` heading in the change's `deltas/*.md`, plus any case the session's observations name. Each one's latest observation comes from `.chough/evidence/`, by `observed_at_unix_seconds`.
  - `Risks:` and the first 8 lines of `plan.md`'s Risks section, or `none recorded`;
  - `Diff: <url or command>`.

  The digest is the SHA-256 of the text.
- **List file.** `.chough/local/gate-lists/<list>.json` holds `{schema_version: 1, head, gates: [{number, session, change, gate, digest, summary}]}`. `<list>` is the first 12 hex digits of its SHA-256. It is written only once `.chough/local/` is ignored, as the direct-write baseline is.
- **`chough gate record <list> --gates 1,3 …`** checks everything first. The list must exist. Each number must be on it. Each gate must still be open in its session. Each summary rebuilt now must have the listed digest. No gate's change may be verifying. The relay values are checked by `checkpoint::check_relay`. Then it closes each gate as `checkpoint --close-gate` does, through the same save, with `relayed_by` carrying `summary_digest` and `list`.
- **Relay fields.** `RelayedBy` gains `summary_digest` and `list`, each `Option`, skipped when `None`. Validation accepts both or neither: `summary_digest` is 64 lowercase hex digits and `list` is 12.
- **Ship with a list.** `chough change ship <id> --list <l> --gate <n> --decided-by …` resolves the gate's text from the list and checks its digest before anything runs. The follow-up close then passes the digest and list through the hidden `checkpoint --close-gate … --summary-digest <d> --list <l>`, so the record is the same as `gate record`'s.
