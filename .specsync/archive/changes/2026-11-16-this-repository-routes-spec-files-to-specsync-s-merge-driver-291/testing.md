---
change: this-repository-routes-spec-files-to-specsync-s-merge-driver-291
artifact: testing
---

# Testing

- The package test fails without the `.gitattributes` lines and passes with them.
- Manual check in a throwaway clone: two branches each bumped `specs/cli/cli.spec.md` to the same next version with their own Change Log row; with the driver installed, `git merge` finished with no conflict markers, rows renumbered and `version:` advanced to the larger number.
