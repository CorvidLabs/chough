# Lesson bundle: fix-260

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Fix 260
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: src/records.rs, tests/bugs.rs, .chough/cases/records.no-partial-new-record.cases.toml, changelog.d/2026-11-18-fix-260.md
- **Acceptance**: When the write of a new bug record fails part way, as under a file size limit, chough bug add exits non-zero, no file is left in .chough/bugs/, and chough bug list, chough status and chough next still exit 0.

## Evidence

- Verification commit: `83e7672a52d30adc673ff0ccdf5fda9485569eb0`
- Base commit: `8abb5bb89b5244e72327e1ea6abfc7e66634ebe1`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

GitHub #260: when the disk is full, `chough bug add` fails but leaves a 0-byte `.chough/bugs/<name>.md`, and from then on `bug list`, `status` and `next` all fail on that file until someone removes it by hand. Past a file size limit it leaves a cut-off record whose front matter is complete, which the readers accept as a valid bug although the command reported failure. `chough journal add` leaves the same empty post. `checkpoint` and `inbox add` leave nothing in the same conditions.

REQ-records-001 already says every record Chough writes is one Markdown file that starts with front matter, written by create-new for a new record. A file left empty or cut off by a failed write is not that. Bugs and journal posts are both created through one writer, `write_new_bytes`, so this change fixes that one place: when the write fails after the file was created, the writer removes the file it created and reports the failure. It changes no requirement text.

Out of scope: `chough doctor` does not report an uncommitted empty or cut-off record that something else left. A new doctor finding needs a sentence in REQ-history-012 first. Writing through a temporary file and a link, as `inbox add` does, is a larger change than this defect needs.

## From the change's testing.md

# Testing

`cargo test --locked --test bugs a_bug_add_whose_write_fails_part_way_leaves_no_partial_record` (Unix only) runs `chough bug add` with a record of about 12,000 bytes under a file size limit of 4,096 bytes, with the limit's signal ignored so the write returns an error as it does on a full disk. It checks that the command exits non-zero, that `.chough/bugs/` holds no file, and that `bug list`, `status` and `next` exit 0. It fails on main, where a 4,096-byte cut-off record is left. A full disk itself was not reproduced; the test uses the size limit because it gives the same failed write on macOS and Linux without a dedicated file system. Journal posts go through the same writer and have no test of their own here.

## Where these lessons go

- `specs/cli/cli.3md`
