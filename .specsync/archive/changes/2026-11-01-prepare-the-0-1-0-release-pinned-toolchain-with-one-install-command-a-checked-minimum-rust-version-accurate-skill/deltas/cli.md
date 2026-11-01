## MODIFIED

### REQUIREMENT REQ-cli-008

Stage routing bounded by native gates. The helper SHALL route the stages intent, design, plan, build, test, accept and done, the optional release stage, and the side stages bugs, validate and librarian. It SHALL record each advance, iterate, back, reset, pause, resume, skip and side-stage enter or return as an append-only transition with its reason. Back and reset SHALL list the later stages whose outputs need refresh and SHALL point to native SpecSync status for any reapproval or recovery. Every move SHALL meet the prerequisite of its destination, whichever command makes it: build SHALL require the recorded native change to be approved, implementing or verifying with a valid definition approval; test and accept SHALL require the build prerequisite or the done prerequisite; done SHALL require the change to be archived with authenticated terminal evidence. Unavailable, unknown or undecodable native state, an approval reported invalid, unauthenticated archive evidence and an invalid SDD policy SHALL refuse those moves; a valid disabled policy or no policy SHALL leave them ungated. The helper SHALL NOT record or infer human approval or review. Trace: FLOW-2, FLOW-5, FLOW-10, LIFE-1.

Acceptance Criteria
- Every transition kind is exercised in an isolated fixture; history survives back and reset; skip without a reason is refused.
- Entering build is refused for draft, unavailable and approval-invalid states and allowed for a valid approved state.
- Reaching test or accept is refused for draft, unavailable and approval-invalid states and allowed for a valid approved state or an archive with authenticated terminal evidence.
- Leaving accept, reset to done and release to done are refused unless the change is archived with authenticated terminal evidence.
- A malformed or ill-typed SDD policy refuses gated moves; a valid disabled policy leaves them ungated.
- Entering a side stage records its caller, and return restores that caller.

### REQUIREMENT REQ-cli-016

Host skill installation. The helper SHALL install the complete skill bundle for each requested supported host directory, defaulting to `.agents/skills`, SHALL report each installed file as current, edited, known-outdated or missing, and SHALL update only missing files and files byte-identical to a known earlier bundle. Trace: FLOW-1, LIFE-4.

Acceptance Criteria
- Installing for the claude host creates `.claude/skills/chough-workflow/` with the complete bundle, and repeating the install preserves edits.
- A bundle file byte-identical to one from a known earlier build is updated, while an edited file is reported and left unchanged.

### REQUIREMENT REQ-cli-019

Installable package. The crate package SHALL contain the helper source, the canonical skill bundle, Cargo files, README and LICENSE, and SHALL NOT contain Chough's own development records; initializing a new project SHALL install the helper's skills and an empty checkpoint only, and skill status SHALL recognize copies installed from earlier builds. The canonical skill bundle SHALL contain no em dash or en dash. The package SHALL declare its minimum supported Rust version, and that version SHALL be built and tested in CI. The README SHALL give one command that installs the HI, SpecSync and Fledge versions CI uses, and the versions named in the README, the agent instructions and CI SHALL agree. Trace: LIFE-9, LIFE-4, LIFE-6.

Acceptance Criteria
- The package list contains no `.chough/`, `.specsync/`, examples, tests or host copies.
- A new project initialized from the installed binary has no bugs, evidence or artifacts.
- No bundle file contains U+2013 or U+2014, and unedited copies of the earlier dashed files are reported known-outdated and updated.
- Cargo.toml declares a rust-version, and a CI job builds and tests with that Rust version.
- The README, AGENTS.md and the CI workflow name the same HI, SpecSync and Fledge versions; Cargo.toml, the README and the CI job for the minimum version name the same Rust version; a test fails naming the file when one is missing or disagrees.
