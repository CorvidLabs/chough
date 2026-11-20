- A new record is written whole or not at all: a full disk or a file-size limit no
  longer leaves a 0-byte or cut-off bug, post or entry that stops or misleads every
  reader, and an empty record file is named as empty (#260). `chough init` works on an
  exFAT volume again: `.chough/local/.gitignore` is no longer created by hard link
  (#424).
