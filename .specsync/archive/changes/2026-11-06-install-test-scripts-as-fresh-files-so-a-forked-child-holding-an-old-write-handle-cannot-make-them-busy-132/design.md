---
change: install-test-scripts-as-fresh-files-so-a-forked-child-holding-an-old-write-handle-cannot-make-them-busy-132
artifact: design
---

# Design

## Approach

A shared `install_executable(path, content)` in tests/common pipes the content into `sh -c 'cat > "$1.tmp.$$" && chmod 755 "$1.tmp.$$" && mv "$1.tmp.$$" "$1"'`. The rename gives the path a new inode. Any write handle a fork leaked points at the old inode, and the new one is only ever written inside that `sh` child. `stand_in` uses it. `Fixture::executable` reads the file it is given, which is a read-only handle and cannot block exec, and reinstalls it the same way.

## Risks

A test that relied on an executable keeping its inode would notice; none does.

## Constraints consulted

CorvidLabs/spec-sync#888. The rule against changing assertions or timeouts.
