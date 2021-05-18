# Simple CLI app to clean your projects from old git branches

[![Actions Status](https://github.com/kattaris/delete_old_branches_cli/workflows/build/badge.svg)](https://github.com/kattaris/delete_old_branches_cli/actions)
[![Coverage Status](https://codecov.io/github/kattaris/delete_old_branches_cli/coverage.svg?branch=master)](https://codecov.io/gh/kattaris/delete_old_branches_cli)
[![Releases](https://img.shields.io/github/v/release/kattaris/delete_old_branches_cli.svg?include_prereleases&style=flat-square)](https://github.com/kattaris/delete_old_branches_cli/releases)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

### Example usage:

```
1. clone repo
2. cargo run
    or
   cargo build
and use binary
3. In path request provide a path to a repo
4.      'all' or 'a' - Shows all branches
        'local' or 'l' - Shows local branches
        'remote' or 'r' - Shows remote branches
        'check branch_name' or 'c branch_name' - Shows branch by specified name
        'delete branch name' or 'd branch name' - Deletes branch by specified name
        'help' or 'h' or '?' - Shows this help list 
        'quit' or 'q' - Exits from app
```

## Work in progress
1. Implement a command to view branches older than xx(in days, weeks, months or years) 