# Lesson bundle: union-merge-only-the-generated-knowledge-notes-and-cover-the-vault-code-of-the-index-fix

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Union-merge only the generated knowledge notes, and cover the vault code of the index fix
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: .gitattributes, src/vault.rs, tests/vault.rs
- **Acceptance**: Only the knowledge notes library index generates and library index --check compares (INDEX.md, rules.md, timeline.md and the timeline, archives, modules and intent notes) merge with Git union; attachments, snapshots, briefs and README.md do not. src/vault.rs, changed by the index-bound fix, is covered. The new test fails first and fledge lanes run verify passes on Linux with the audit green.

## Evidence

- Verification commit: `2d81d3d290e9faa91178adb148ce706a988eb6a6`
- Base commit: `4ab0b8ad33a937a7945f7ac5b898d7b9145ca5f7`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

corvid-agent's review of #112 at 71a3efd3 found two blockers:
1. `.gitattributes` gave `knowledge/**` the union driver. That reaches attachments, snapshots, briefs and the README, which `library index --check` never compares. Two branches editing one attachment JSON would merge cleanly into invalid JSON while the check still reported the vault current.
2. The index-bound fix changed `src/vault.rs` (`RECENT`, `timeline_notes`, the module-note cap), which was outside its declared paths, so the audit refuses that path.

Both fall under Leif's decisions for #112: "Yes, record and merge", and "Widen #112 with the fix" (09:21 MST).

## From the change's testing.md

# Testing

tests/vault.rs `only_generated_knowledge_notes_merge_by_union` runs `git check-attr merge` on this repository:
- `knowledge/INDEX.md`, `rules.md`, `timeline.md`, `timeline/*.md`, `archives/*.md`, `modules/*.md` and `intent/*.md` get `union`;
- attachments, snapshots, briefs and `README.md` do not.

It fails first, because `knowledge/**` gave `union` to everything. `fledge lanes run verify` passes on Linux with the audit green.

## Where these lessons go

- `specs/cli/context.md`
