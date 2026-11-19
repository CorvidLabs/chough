- The leak check no longer refuses an ordinary word that equals a short all-letter host
  name: such a name is a leak only inside a path or a URL or as `<user>@<host>`, while a
  host name of 8 or more characters, or holding a digit, a hyphen or a dot, is still
  refused anywhere, in writes and in `chough doctor` alike (REQ-records-006).
