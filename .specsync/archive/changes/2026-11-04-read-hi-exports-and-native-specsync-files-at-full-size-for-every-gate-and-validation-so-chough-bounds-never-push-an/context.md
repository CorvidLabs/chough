---
change: read-hi-exports-and-native-specsync-files-at-full-size-for-every-gate-and-validation-so-chough-bounds-never-push-an
artifact: context
---

# Context

Issue https://github.com/CorvidLabs/chough/issues/100, filed by Leif (0xLeif), the
Chough maintainer and the jackdaw product owner. In CorvidLabs/jackdaw PR #41 an
agent following the Chough workflow rewrote the human's HI intent to fit a Chough
size bound: `hi/board.md` intent went from about 1,300 words to 64 and `hi/cost.md`
from about 1,250 to 70. `evidence::validate_criteria` parsed `hi export -- <FAMILY>`
from `records::native`, whose stdout is the 16,384-byte display section, so any family
export over 16 KiB failed with "native HI family export is invalid or exceeds 16384
bytes". The same class of bound applied to native SpecSync files read through
`read_record` (65,536 bytes): archived `finalization.json` (`latest_archive` and
`retention_protection` in src/evidence.rs), archived `change.md` (src/library.rs,
silently dropped when large), `.specsync/sdd.json` (`sdd_policy` in src/stage.rs) and
the timing report's native records (`native_record` in src/timing.rs). REQ-cli-034
(2026-11-03) exempted spec files under `specs/` but not HI or these SpecSync files.

The owner's rule, verbatim (2026-11-04, relayed by the parent agent): "Chough should
only reduce and do all that for its own files in .chough and not the specs, not the
human intent." Captured as LIFE-15 in his words.

Authority: that instruction and issue #100 (its "Proposed fix" items 1 to 4) cover
this bounded fix; the agent account `claude` acts under it. Limits: no merge, tag,
publish, release or human acceptance grade; Chough's own record bounds unchanged.

Out of scope: `chough library snapshot` keeps its 4 MiB copy limit (REQ-cli-034 names
it as a known exception; it decides no gate). Earlier spec splits (history #59,
reporting) cited the 65,536-byte record bound for `specs/cli/requirements.md`; this
change does not undo them.
