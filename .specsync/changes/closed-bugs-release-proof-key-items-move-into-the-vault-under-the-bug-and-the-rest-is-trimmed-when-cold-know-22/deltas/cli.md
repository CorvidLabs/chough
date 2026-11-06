## ADDED

### REQUIREMENT REQ-cli-048

Closed bugs release proof. In the post-archive summary and trim of REQ-cli-033, REQ-cli-041 and REQ-cli-046, a bug that is closed (verified, duplicate or wont-fix) SHALL NOT protect the observations it links: its observations, note observations and regression baseline. Its key items SHALL move into `knowledge/attachments/<folder>/`, where the folder is formed from the bug ID as REQ-cli-046 forms it from an archive ID, instead of being removed (KNOW-22). The key items are:
- the earliest linked failed observation, by observed time and then ID, with its artifacts;
- for a verified bug, the first linked passed observation observed after that failure, with its artifacts;
- the bug's fixture when it is a file or folder under `.chough/`.

A key item SHALL move only when it would otherwise be trimmed: its set or loose group is owned by a cold archive and has no other protection. Every other observation the bug links SHALL be summarized, ledgered and trimmed by the same cold rule. Moves SHALL follow REQ-cli-046: they never overwrite, keep permissions, are ledgered with their attachment paths, and are undone by a restore. A linked record that is missing or unreadable is not a key item, and an item two closed bugs name SHALL go with the bug whose ID sorts first. The bug record SHALL keep its status and IDs. An open bug SHALL keep protecting everything it links. `chough evidence prune`, which moves nothing into the vault, SHALL still treat every bug's links as protection.

`chough library index` SHALL write `bugs/<id>.md` for each closed bug whose attachments folder holds files. The note SHALL have front matter giving `id`, `title`, `type: bug`, `status` and `tags`, and SHALL link the bug record and each of its attachments. Trace: KNOW-22, KNOW-18, KNOW-19, KNOW-13.

Acceptance Criteria
- For a verified, a duplicate and a wont-fix bug linking a cold failed run with a log, a later passed run and another passed run, the trim at done:
  - moves the failed run and its log into `knowledge/attachments/<bug>/`;
  - moves the later passed run there only for the verified bug;
  - trims and ledgers the other run;
  - writes `knowledge/bugs/<bug>.md` linking what moved;
  - leaves the bug record unchanged.
- An open bug keeps every observation it links.
- A closed bug's fixture under `.chough/` moves into the vault under the bug.
- `evidence prune` still names a verified bug's link as a protection.
