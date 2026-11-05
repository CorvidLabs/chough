---
change: make-the-knowledge-directory-an-obsidian-style-vault-yaml-front-matter-on-every-note-iso-dates-standard-links-and
artifact: plan
---

# Plan

## Approach

A new `src/vault.rs` renders deterministic YAML front matter, ISO 8601 UTC dates, the
requirement-trace map and the module, intent and rules notes. `src/library.rs` reads
each archive's affected specs and calls into it while building the directory, so every
note is built and bounded in memory before anything is written.

## Out of scope

The wider reduction and attachments (KNOW-16 to KNOW-19), [[wikilinks]], and copying
any lesson or criterion text.

## Steps

1. Capture KNOW-14 to KNOW-19 in hi.
2. Add the failing tests in tests/vault.rs.
3. Implement src/vault.rs and wire it into the build.
4. Update the skill reference, docs and CHANGELOG, then regenerate this repository's
   vault.

## Risks

Front matter must stay valid YAML for every title: titles and commit IDs are written as
double-quoted JSON strings.

## Constraints consulted

KNOW-3 (no second ledger), KNOW-4 (deterministic), KNOW-12 (pointers only), the
65,536-byte note bound, and the brief's composability guidance for files over 1,000
lines.
