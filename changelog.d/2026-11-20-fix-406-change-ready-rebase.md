- `chough change ready` counts only the change's own work after a rebase onto the
  default branch or a squash merge of it, so the default branch's files are no longer
  named as outside the declared paths; on the default branch itself every changed file
  still counts, and a base commit the repository no longer has is said plainly instead
  of Git's error (#406).
