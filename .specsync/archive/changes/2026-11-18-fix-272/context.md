---
change: fix-272
artifact: context
---

# Context

GitHub #272: twelve `chough journal add` commands run at once all write their post, but the regenerated `knowledge/journal/feed.xml` can miss some, and `chough library index --check` then fails. REQ-cli-040 already names `journal add` among the commands that hold the project's writer lock while they read what they change and save it. On main at 8abb5bb `journal add` takes no lock, so each command rebuilds the feed from a directory listing another command is still adding to. This change makes the code do what the requirement says. It changes no requirement text and adds no command or flag. `chough library index` is not in REQ-cli-040's list and is not changed here.
