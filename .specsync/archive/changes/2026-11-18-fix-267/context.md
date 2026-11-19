---
change: fix-267
artifact: context
---

# Context

GitHub #267: a GitHub token glued to a hyphen, `x-ghp_<token>`, is accepted in a checkpoint question, a session key and a raw log name and is committed byte for byte, and `chough doctor --leaks` does not report it. The same token after a space, colon or dot is refused. Slugs, IDs and CI run names produce exactly this hyphen-glued shape. REQ-records-006 already refuses a credential token form in every value Chough writes and has the doctor scan for the same patterns. On main at 8abb5bb the matcher skips a token prefix when the byte before it is a letter, digit, underscore or hyphen, so a hyphen hides the token. This change makes a hyphen a boundary, in the prefixed forms and in the AWS key form alike. A letter, digit or underscore before the prefix still continues a word, as before. It changes no requirement text. #268 (the --file write check) is a separate, parked issue and is not touched.
