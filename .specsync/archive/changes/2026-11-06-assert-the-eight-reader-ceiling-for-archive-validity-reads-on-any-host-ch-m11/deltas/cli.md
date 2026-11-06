## MODIFIED

### REQUIREMENT REQ-cli-033

Post-archive summary. `chough library archive` SHALL preview, writing nothing, the observation sets of archived changes outside the newest 3 that are committed identically at HEAD. These are sets listed only by such archives, and sets that no archive lists, attributed by cycle time to the first archived change finalized at or after the observation when that change is outside the newest 3 (KNOW-20). It SHALL keep every retention protection but age, with reasons, except that among the routine runs of one case and environment only the newest run and the newest failure stay protected as newest (KNOW-17). `--apply` SHALL need retention authority, SHALL refuse while a native change is open, SHALL ledger each removal first with the archive that lists or owns the set and its knowledge entry, and SHALL restore every file and refuse if the validity of an archive that lists a removed file changes. It SHALL read the native validity of those archives in parallel, with at most 8 at once. Each archive's knowledge entry SHALL summarize the sets attributed to it. Removed observations SHALL read as pruned. Trace: KNOW-7, KNOW-8, KNOW-9, KNOW-10, KNOW-11, KNOW-17, KNOW-20, LIFE-8.

Acceptance Criteria
- The preview writes nothing and keeps hot-archive, referenced, last-failure and non-routine sets protected with reasons, and the newest run of each case and environment protected as newest.
- Apply refuses without authority or with an open change; otherwise it removes only HEAD-identical files, ledger first, and they read as pruned.
- A validity change restores every removed file byte-identically and refuses.
- Of four routine runs of one case and environment listed only by a cold archive, the newest run and the newest failure stay and the other two are trimmed.
- A record no archive lists, observed in a cold archive's cycle, is trimmed and ledgered with that archive and its knowledge entry, and that entry names it; a record in a hot archive's cycle, or observed after the newest archive, stays.
- The native validity reads of the affected archives overlap.
- With twelve affected archives, no more than 8 validity reads run at once, and the reader count is at most 8 whatever parallelism the host reports.
