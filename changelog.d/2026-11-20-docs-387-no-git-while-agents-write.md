- The workflow skill's checkpoint reference and `docs/cli.md` say not to run `git pull`,
  `git merge`, `git rebase`, `git checkout`, `git stash` or another Git command that
  rewrites the worktree while another agent works there, since it can replace a record
  another agent's Chough command has just saved; a lone agent may run one between its
  own commands (#387).
