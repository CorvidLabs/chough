# Lesson bundle: recognize-a-crlf-checkout-of-the-earlier-knowledge-readme-and-treat-an-unmatched-backtick-as-literal-when-reading-trace

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Recognize a CRLF checkout of the earlier knowledge README and treat an unmatched backtick as literal when reading Trace lists (Windows CI and review of 107)
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: src/vault.rs, src/library.rs, tests/vault.rs
- **Acceptance**: An untouched earlier knowledge README checked out with CRLF line endings is upgraded like the LF one, so the README test passes on Windows; an edited README is still kept; a requirement line with one unmatched backtick before its Trace list still credits that list. The new test fails first and fledge lanes run verify passes on Linux; hosted Windows passes.

## Evidence

- Verification commit: `4425f19d5ae6eeff3df0dca8f5fea2ab4b068f98`
- Base commit: `6b2dfb557fe939b6b2d98e3fd50006d1dae4442e`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

Hosted CI on #107 at 6b2dfb5 failed on Windows only: `an_untouched_earlier_readme_is_upgraded_and_an_edited_one_kept` panicked, because the README fixture is checked out with CRLF line endings, so its bytes no longer matched the 8fbbbe7 template digest. A real Windows checkout of an untouched README would also stay un-upgraded. The digest is now taken over the LF-normalized bytes. corvid-agent's non-blocking note at 6b2dfb5 is handled too: one unmatched backtick before a `Trace:` list put the rest of the line "in code", but CommonMark treats an unmatched backtick as a literal, so only paired backticks open a code span now.

## From the change's testing.md

# Testing

tests/vault.rs `crlf_earlier_readmes_upgrade_and_unmatched_backticks_keep_the_trace_list` writes the 8fbbbe7 README with CRLF line endings and a requirement line with one unmatched backtick before its Trace list. It fails first. The earlier README test, with its edited control, must pass on hosted Windows, and `fledge lanes run verify` runs on Linux.

## Where these lessons go

- `specs/cli/context.md`
