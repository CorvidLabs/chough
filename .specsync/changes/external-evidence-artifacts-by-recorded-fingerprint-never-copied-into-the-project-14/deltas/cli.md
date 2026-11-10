# External evidence artifacts by fingerprint (issue 14)

## Modified

### REQUIREMENT REQ-cli-006

The helper SHALL bound assembled context and reject unsupported records or paths escaping the project. An absolute artifact path on an observation record is supported input that stays outside the project: the file SHALL be fingerprinted (SHA-256) in place within a 64 MiB bound and SHALL NOT be copied into the project; a missing, symlinked, directory or oversized external file SHALL be refused with its path named, and the freshness re-read SHALL re-read it in place (#14). Trace: LIFE-3, LIFE-4.

Acceptance: traversal, invalid module/record IDs and symlink escape attempts fail; large context is visibly bounded. Local artifacts remain reviewable plain files.

Acceptance Criteria
- An absolute artifact path is fingerprinted in place, never copied, and missing, symlinked or oversized external files are refused by name; project-relative canonical artifact behavior is unchanged.
- The reader SHALL accept optional diagnostic inventory v1 with strict inventory fields version:u32, complete:bool, entries and strict entry fields domain:u8, key, path, digest. Inventory SHALL contain at most128 entries and at most32768 compact serialized JSON bytes. v1 entries SHALL have unique safe nonempty relative normal paths, valid SHA256 digests and domain/key matching the existing canonical fingerprint_key classification. Unknown fields/types, invalid semantic entries, exceeded limits and unsupported versions SHALL refuse explicitly without mutating observations. Absent/null inventory SHALL preserve existing legacy reading behavior and existing whole-record bounds.
