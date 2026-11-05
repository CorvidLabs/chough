---
hi: 1
families: [KNOW]
---

# Knowledge

## Intent

I want one context knowledge directory in each project that compounds what earlier
work learned: every archived change, the proof behind it and the lessons it left, in
a form the next agent reads at start and resume. The hot working folder should hold
only what current work needs, with older raw proof summarized and still recoverable
from Git, never silently lost. Source: Leif in chat, 2026-11-03, "I want the librarian
and archiving and compounding of lessons and archives and all.. artifacts.. we need a
context knowledge directory", and his 8:09–8:27 AM MT answers to the follow-up
questions.

## Criteria

- **KNOW-1**  My project has one context knowledge directory, `knowledge/` at the project root, and an agent is pointed to its index at start and resume, right after the checkpoint. Source: Leif in chat, 2026-11-03 8:09–8:27 AM MT, "Use knowledge/ at the project root".
- **KNOW-2**  The knowledge directory has an index and one entry per archived native change that records what was wanted, the requirements and commits it verified, each observation's outcome and environment, every listed record and artifact with its digest and where its bytes are now, the bugs linked to it and a pointer to its lesson bundle. Source: Leif in chat, 2026-11-03 8:09–8:27 AM MT; the entry fields are an agent-normalized capture of "archiving and compounding of lessons and archives and all.. artifacts".
- **KNOW-3**  The knowledge directory is never a second ledger: it points to HI, SpecSync archives, module lesson sources and Chough records instead of restating their current state, and no gate reads it. Agent-normalized safeguard from the project rule that HI, SpecSync and Chough keep owning their records.
- **KNOW-4**  The index and entries are regenerated from their sources, and regenerating with the same sources gives identical bytes. Agent-normalized safeguard so the directory can be rebuilt rather than hand-maintained.
- **KNOW-5**  The librarian and knowledge directory are on by default in every project, and I can opt out with a recorded preference; with it off, the checkpoint and cold resume still work. Source: Leif in chat, 2026-11-03 8:09–8:27 AM MT, "Librarian on everywhere with an opt-out".
- **KNOW-6**  Existing `library/` snapshots and briefs stay readable and byte-identical and are listed in the knowledge index as they are, with no migration. Source: Leif in chat, 2026-11-03 8:09–8:27 AM MT, "Use knowledge/ at the project root", with the existing library content indexed as is.
- **KNOW-7**  After a native change is archived and falls outside the hot window, its raw observation records and artifacts that nothing current needs leave the working tree and only its knowledge summary stays, with each file's digest and the commit that holds its bytes, so they remain recoverable from Git; a file is removed only when its bytes are committed identically at the current commit. Source: Leif in chat, 2026-11-03 8:09–8:27 AM MT, "Keep only a summary; raw files stay recoverable from git".
- **KNOW-8**  The newest three archived changes stay hot with all of their raw proof, and proof that anything current needs stays too: the checkpoint's references, bugs of any status, open native changes, validation runs, the newest runs and newest failure of each case and environment, manually or agent-assisted recorded proof and files named directly. Source: Leif in chat, 2026-11-03 8:09–8:27 AM MT, "Keep the newest 3 archived changes hot"; the other protections are the existing retention protections kept unchanged.
- **KNOW-9**  Every removal is written first to the append-only retention ledger with the change, the knowledge entry, the digests and the authority, and a removed observation is shown as pruned, never as current or passed. Agent-normalized safeguard extending LIFE-8 to the summary step.
- **KNOW-10**  Summarizing old proof never weakens a native archive: when a removal would change any affected archive's terminal evidence validity, the removed files are restored and the step is refused. Agent-normalized safeguard; the main technical risk Leif was shown with the draft.
- **KNOW-11**  The summary step is a read-only preview by default, and removing files is a separate step that needs my retention consent or standing preference and is refused while any native change is open, so it never runs between verification and archive. Agent-normalized safeguard from the existing retention consent rule.
- **KNOW-12**  The knowledge index keeps a lessons index that points to each module's native lesson source and each archived change's lesson bundle, without copying lesson text, so each archive adds to what the next agent is pointed to. Source: Leif in chat, 2026-11-03 8:09–8:27 AM MT; agent-normalized capture of "compounding of lessons".
- **KNOW-13**  With the librarian on, which is the default, the old proof that the summary step may remove is trimmed automatically when my change reaches done after its archive, and I can opt out with a recorded preference; the librarian default counts as my standing retention preference. Source: Leif in a Claude Code session, 2026-11-04 about 7:20 AM MST, choosing "Auto-trim at done" for issue #88 from the offered options, whose wording was "With the librarian on (the default), stage advance into done, after the archive, trims cold changes (ledger first, KNOW-10 checks kept), and chough pref set retention manual opts out. The librarian default counts as the standing preference." A preview with a one-step apply, and an opt-in standing preference, were offered and not chosen. KNOW-11 is unchanged: this is the standing preference it names.
