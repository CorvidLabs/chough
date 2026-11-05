## ADDED

### REQUIREMENT REQ-cli-047

No conflicting files. When two branches each record Chough work, merging one into the other SHALL need no hand-resolved conflict in a file Chough writes (DRIVE-12). Case definitions MAY be split per change: `.chough/cases/<module>.<part>.cases.toml` SHALL hold cases of `<module>`, which its `module` field SHALL name, `<part>` SHALL use letters, digits, `-` and `_`, and case IDs SHALL stay unique across all files. `chough library index --check` SHALL write nothing and SHALL fail, naming each generated knowledge file that is missing or differs from a rebuild, with line endings ignored; the verify and CI lanes SHALL run it, so generated knowledge can merge with Git's union driver and be rebuilt. `chough bug merge <base> <ours> <theirs>`, a Git merge driver for `.chough/bugs/*.json`, SHALL merge two edits of one bug: a field only one side changed SHALL take that change, list fields SHALL keep base's entries and then each side's additions without duplicates, the update time SHALL be the later one, the result SHALL be a valid bug record written as bug commands write it, and a field both sides changed differently SHALL be left for a human with `ours` unchanged and a nonzero exit. Trace: DRIVE-12, DRIVE-3.

Acceptance Criteria
- Two branches that each add a case in their own `cli.<part>.cases.toml` merge without conflict, and both cases load; a case ID defined in two files is refused.
- `library index --check` passes on a rebuilt vault, fails naming a generated file that was edited, passes again after `library index`, and passes on a CRLF copy of a current file.
- Two edits of one bug, one adding a note and verifying it and the other adding a note and an observation, merge with both notes, the observation, the verified status and the later update time; two different status changes exit nonzero and leave ours unchanged.
