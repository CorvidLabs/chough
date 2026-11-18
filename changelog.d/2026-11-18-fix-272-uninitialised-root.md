- `chough journal add` now holds the writer lock in a project with no `.chough/`
  directory too. It writes there with default preferences, and the first fix for #272
  took no lock on that path, so parallel adds could still leave the feed missing a
  post (review of #302, REQ-cli-040).
