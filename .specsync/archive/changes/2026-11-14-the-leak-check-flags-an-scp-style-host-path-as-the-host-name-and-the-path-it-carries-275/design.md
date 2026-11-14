---
change: the-leak-check-flags-an-scp-style-host-path-as-the-host-name-and-the-path-it-carries-275
artifact: design
---

# Design

`names_a_host` in `src/leaks.rs` gains one context: a name followed by `:` and then `/` or `\` is a host in context, like a name inside a path or after `<user>@`. The path after the colon was already checked as a path. No visual or interaction change.
