---
change: change-leaks
artifact: context
---

# Context

Issue #412 and the sites of #367 that L2 (#418, merged at 643420a9) left to it.

- `chough change new`'s description and `--rationale`, and `chough change answer`'s answer, reach committed files under `.specsync/changes/<id>/` (state.json, change.md) with no leak check (src/change.rs, the New and Answer arms). The IDs (kind, spec, path, id, question) are already refused by #413.
- `chough doctor --leaks`, CI's leak step, scans only files Git tracks under `.chough/` and the knowledge folder (src/doctor.rs, `tracked`).
- The echo sites left to #412 (tests/output_safety.rs's FOR_412): `change new`'s description, `change answer`'s answer, and `change ship --close-gate` in the relay refusal (src/change.rs, "closing gate"). `evidence.rs:798` says a bare "invalid evidence ID" for an explicit ID, while evidence.rs:922 names what it holds.

Measured on main with a throwaway build that adds SpecSync's folders to doctor's scan (scratchpad r412, one-lane):
- The 22 real repositories: scanning `.specsync/changes/` adds 2 findings in 2 repositories, both in artifacts and neither holding a user or host name:
  - Pix: `research.md`, a tilde path to a tool's folder (home-path);
  - site: `testing.md`, a temporary screenshot path (system-path).

  Those two repositories' leak scans would then exit 1. Scanning `.specsync/archive/` as well adds 279 findings in 7 repositories, among them 116 in Chough's own archive, which CI would then fail on for good. No other finding changed in any repository.
- The (A) replay: of every description and answer already in the 22 repositories, the leak check would have refused 4 values (`change.md` and `state.json`, all in archives: chough, jackdaw, spec-sync). No open change holds one.

Based on main 7bf110dc: #419 changed REQ-cli-067 only.
