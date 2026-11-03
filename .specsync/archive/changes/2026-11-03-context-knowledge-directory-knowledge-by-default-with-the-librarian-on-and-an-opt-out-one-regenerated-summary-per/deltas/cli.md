## ADDED

### REQUIREMENT REQ-cli-033

Post-archive summary. `chough library archive` SHALL preview, writing nothing, the observation sets listed only by archived changes outside the newest 3 and committed identically at HEAD, keeping every retention protection but age, with reasons. `--apply` SHALL need retention authority, SHALL refuse while a native change is open, SHALL ledger each removal first, and SHALL restore every file and refuse if an affected archive's validity changes. Removed observations SHALL read as pruned. Trace: KNOW-7, KNOW-8, KNOW-9, KNOW-10, KNOW-11, LIFE-8.

Acceptance Criteria
- The preview writes nothing and keeps hot-archive, referenced, newest, last-failure and non-routine sets protected with reasons.
- Apply refuses without authority or with an open change; otherwise it removes only HEAD-identical files, ledger first, and they read as pruned.
- A validity change restores every removed file byte-identically and refuses.

## MODIFIED

### REQUIREMENT REQ-cli-015

Project librarian. The librarian SHALL be on by default in `knowledge/` (or `library-dir`) and off only by a recorded preference. It SHALL copy requested originals into snapshots identified by REQ-cli-023 with source path, Git revision and SHA-256, SHALL never move, rewrite or replace a canonical source, and SHALL regenerate a deterministic index of canonical sources, native changes, open bugs, observations, briefs, snapshots, an earlier `library/` folder as is, a lessons index of pointers, and one entry per archived change with its criteria, commits, listed files with digests and locations, outcomes, bugs and lesson bundle. No gate SHALL read it. Snapshot IDs SHALL be `SNAP-<slug>-<24-lowercase-hex>` with a random 96-bit suffix; legacy dated IDs SHALL stay readable and listed without migration. It SHALL be usable as a side stage from any stage. Trace: FLOW-12, KNOW-1, KNOW-2, KNOW-3, KNOW-4, KNOW-5, KNOW-6, KNOW-12.

Acceptance Criteria
- Snapshotting leaves the source byte-identical and records its digest; an escaping or symlinked path fails.
- The index lists every open bug, including snapshotted ones.
- Checkpoints and resume work with the librarian off; with no preference it is on in `knowledge/`.
- Repeated identical snapshots get distinct SNAP IDs, keeping digests and timestamps in the manifest.
- Legacy dated snapshots stay readable and indexed beside new ones, and same-ID different-content creation refuses without overwrite, fallback or retry.
- Regenerating gives identical bytes; archive entries list records and artifacts with digests and locations; the lessons index has no lesson text.
