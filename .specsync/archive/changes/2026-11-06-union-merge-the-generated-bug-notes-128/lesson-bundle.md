# Lesson bundle: union-merge-the-generated-bug-notes-128

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Union-merge the generated bug notes (128)
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: .gitattributes, tests/vault.rs, CHANGELOG.md
- **Acceptance**: git check-attr reports merge=union for knowledge/bugs/<id>.md while attachments, snapshots, briefs and the README stay non-union; the test fails first and fledge lanes run verify passes on Linux.

## Evidence

- Verification commit: `a1dba47c0a06374a72d65f510219131680512701`
- Base commit: `4c68cca7275b3ab8f49af94cc925bf547842757a`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

Issue #128. KNOW-22 (#121) added generated bug notes at `knowledge/bugs/<id>.md`, but `.gitattributes` has no `merge=union` line for them. Every other generated knowledge note union-merges and is rebuilt by `library index --check` (REQ-cli-047, DRIVE-12). So two branches that each change a bug note can still conflict.

This was found while scoping KNOW-23 (#127). Leif chose "Yes, small separate fix" on 2026-11-05, via the orc.

## From the change's testing.md

# Testing

- tests/vault.rs `only_generated_knowledge_notes_merge_by_union` checks that a bug note path is `union`. It fails first, with `unspecified`. Attachments, snapshots, briefs and the README stay non-union.
- `fledge lanes run verify` passes on Linux.

## Where these lessons go

- `specs/cli/context.md`
