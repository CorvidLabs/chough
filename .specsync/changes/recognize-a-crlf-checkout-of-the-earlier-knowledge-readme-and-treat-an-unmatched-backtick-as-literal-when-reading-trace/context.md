---
change: recognize-a-crlf-checkout-of-the-earlier-knowledge-readme-and-treat-an-unmatched-backtick-as-literal-when-reading-trace
artifact: context
---

# Context

Hosted CI on #107 at 6b2dfb5 failed on Windows only: `an_untouched_earlier_readme_is_upgraded_and_an_edited_one_kept` panicked, because the README fixture is checked out with CRLF line endings, so its bytes no longer matched the 8fbbbe7 template digest. A real Windows checkout of an untouched README would also stay un-upgraded. The digest is now taken over the LF-normalized bytes. corvid-agent's non-blocking note at 6b2dfb5 is handled too: one unmatched backtick before a `Trace:` list put the rest of the line "in code", but CommonMark treats an unmatched backtick as a literal, so only paired backticks open a code span now.
