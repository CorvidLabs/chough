- The gate summary counts evidence recorded with no case, shows work not committed yet
  as `git diff <base>`, and counts a session started before the first commit from that
  commit (#389). `doctor`'s unfinished-archive fix names the session's journal post
  among the files to commit (#389). Without SpecSync, a move past plan or accept says
  no human decision is recorded, and `stage advance --to` across them needs
  `--reason`, which lands in the Timeline (#390).
