# External artifacts stay machine-local (issue 14)

## Modified

### REQUIREMENT REQ-cli-006

The helper SHALL bound assembled context and reject unsupported records or paths escaping the project. An absolute artifact path given to `evidence add` or `cases record` names an external artifact outside the project: the file SHALL be fingerprinted (SHA-256) in place within a 64 MiB bound and SHALL NOT be copied into the project, and the committed record SHALL keep only a neutral `external:<16 hex>` ID and the fingerprint, never the path or any part of it, which can name a person or a machine; the path SHALL be kept only in the git-ignored machine-local `.chough/local/external-artifacts.json`. A missing, symlinked, directory or oversized external file SHALL be refused with its path named. Freshness SHALL re-read an external artifact in place where this machine bound it; where the mapping or the file is absent, the observation SHALL read `external-not-checkable-here`, neither current nor historical, because an external artifact verifies only on the machine that recorded it (#14). Trace: LIFE-3, LIFE-4.

Acceptance: traversal, invalid module/record IDs and symlink escape attempts fail; large context is visibly bounded. Local artifacts remain reviewable plain files.

Acceptance Criteria
- An absolute artifact path is fingerprinted in place, never copied; the committed record holds only `external:<id>` and the fingerprint, with no path, file name, user name or home directory; missing, symlinked, directory or oversized external files are refused by name; project-relative canonical artifact behavior is unchanged.
- On the recording machine a changed external file reads stale; without the machine-local mapping or the file the observation reads `external-not-checkable-here`; `evidence prune --apply` never removes an external file.
- The reader SHALL accept optional diagnostic inventory v1 with strict inventory fields version:u32, complete:bool, entries and strict entry fields domain:u8, key, path, digest. Inventory SHALL contain at most128 entries and at most32768 compact serialized JSON bytes. v1 entries SHALL have unique safe nonempty relative normal paths, valid SHA256 digests and domain/key matching the existing canonical fingerprint_key classification. Unknown fields/types, invalid semantic entries, exceeded limits and unsupported versions SHALL refuse explicitly without mutating observations. Absent/null inventory SHALL preserve existing legacy reading behavior and existing whole-record bounds.
