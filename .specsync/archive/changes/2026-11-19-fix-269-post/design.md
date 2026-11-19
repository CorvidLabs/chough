---
change: fix-269-post
artifact: design
---

# Design

- `journal::derived_body` takes the story's front matter `objective` as its first part, where it took the Goal's first paragraph. The weights and cutting are unchanged.
- `sessions::archive` compares the record's Goal first paragraph with `one_line(objective)` before rendering the story. When they differ, it appends one paragraph, `Objective at closeout: <objective>`, to the end of the Goal section. The title, the Goal paragraph and every other byte stay as they were. A Goal that is the objective gets no line.
