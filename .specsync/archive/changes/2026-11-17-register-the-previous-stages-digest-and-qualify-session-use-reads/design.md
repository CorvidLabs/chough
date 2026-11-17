---
change: register-the-previous-stages-digest-and-qualify-session-use-reads
artifact: design
---

# Design

No visual or interaction change.

`KNOWN_PREVIOUS` gains the SHA-256 of `references/stages.md` at main `0fd9fd22`. The regression reads that file from `tests/fixtures/stages-at-0fd9fd22.md`, outside the skill-release blob corpus. A pristine copy becomes current on `chough skill update`. An edited copy stays `preserved-edit`.

`docs/architecture.md` keeps the sentence that starting or switching never moves or rewrites another session. It replaces the claim that `session use` reads only the activated session with the three reads `resolve_open` does: an exact open ID returns without reading another session, a bare key or alias reads each open session, and a name that matches none also reads archived stories.
