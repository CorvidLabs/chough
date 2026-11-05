---
change: recognize-a-crlf-checkout-of-the-earlier-knowledge-readme-and-treat-an-unmatched-backtick-as-literal-when-reading-trace
artifact: testing
---

# Testing

tests/vault.rs `crlf_earlier_readmes_upgrade_and_unmatched_backticks_keep_the_trace_list` writes the 8fbbbe7 README with CRLF line endings and a requirement line with one unmatched backtick before its Trace list. It fails first. The earlier README test, with its edited control, must pass on hosted Windows, and `fledge lanes run verify` runs on Linux.
