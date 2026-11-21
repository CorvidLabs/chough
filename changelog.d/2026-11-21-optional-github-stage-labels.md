- Optional GitHub stage labels. `integrations/github/` ships a workflow and a
  script a project copies in to label a pull request with the seven main Chough
  stages, in rainbow order. `chough init` does not install them, and the `chough`
  binary does not call GitHub. This repository runs the same two files.
