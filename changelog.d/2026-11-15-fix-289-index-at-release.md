- The knowledge index and feed (`knowledge/index.md`, `knowledge/journal/feed.xml`) are
  rebuilt and checked once, at release, not at each move into done or in each pull
  request, so open pull requests no longer conflict on them (#289, REQ-cli-043,
  REQ-cli-047). The move into done writes its journal post and reports the index
  `deferred`; no verify, CI or pull request lane runs `chough library index --check`.
  `fledge run release-check` rebuilds the index on the release preparation branch, fails
  until the rebuild is committed, then runs the strict check. Between releases the index
  on main lags its sources.
