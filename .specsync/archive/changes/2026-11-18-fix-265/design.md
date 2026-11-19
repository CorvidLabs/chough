---
change: fix-265
artifact: design
---

# Design

- `records::conflict_marker(text)` returns the first conflict line of a record's body, with its 1-based file line and its text. The body is what follows the front matter's closing `---`.
  - A line starting `<<<<<<<` or `>>>>>>>` is a marker.
  - A line of `=======` is one only while a `<<<<<<<` line is open.
- `records::write_new_bounded` and `write_replace_bounded`, through which every record write passes, refuse a record, a Markdown file under `.chough/` or a journal post, whose body holds one (a generated knowledge note or a skill file may quote a marker), as `refused:conflict-markers`, before anything is written.
- `chough migrate --apply` checks every planned text file the same way before it writes any.
- `chough doctor` adds `conflict_markers` findings for committed records only, the files `bound_of` classifies as records. Each finding gives the file, the line and the marker. There is no fix, and `--fix` reports it `skipped`.
