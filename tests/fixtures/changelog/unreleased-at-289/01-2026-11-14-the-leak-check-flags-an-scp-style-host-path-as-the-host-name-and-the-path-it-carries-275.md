- The leak check now flags an scp-style remote path, `host:/path`, as the host name and
  the path it carries: a home or system root after the colon refuses as a path leak, and
  the host part counts as the local host name when it matches, in writes and in
  `chough doctor` alike (REQ-records-006, #275).
