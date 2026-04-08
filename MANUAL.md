# Fold Reference Manual

## Repositories

A repository is the top-level codebase record in `fold`.

Each repository maps to one remote git repository hosted on a forge.

The canonical repository identifier is:

```text
host/owner/name
```

Example: `github.com/penberg/fold`

`fold repo` commands:
- register a repository with `fold`
- sync a repository's local bare clone from the remote
- remove a repository from `fold`
- list repositories known to `fold`

Repository commands do not create workspaces or sessions.

## Workspaces

A workspace is a local checkout for a registered repository.

Each workspace belongs to exactly one repository and is backed by a git worktree on disk.

`fold workspace` commands:
- create a new local worktree for a repository
- clone a workspace into a new local worktree
- list workspaces for a repository
- inspect workspace metadata
- remove a workspace

## Sessions

A session is a command environment attached to one workspace.

Sessions are the operational unit that `fold` runs, attaches to, and eventually shows as terminal tabs in the TUI.

Each session belongs to exactly one workspace.

`fold session` commands:
- create a new session for a workspace
- list sessions
- inspect session metadata
- attach to an existing session
- stop a running session

## Prune

`fold prune` removes local state that is no longer useful.

`fold prune` commands:
- prune stopped, failed, or exited sessions

## On-Disk Format

`fold` stores persistent state in the XDG data directory:

```text
~/.local/share/fold/
```

Layout:

```text
~/.local/share/fold/
  index.db
  repos/
    <host>/
      <owner>/
        <name>/
          meta.toml
          repo.db
          workspaces/
            <workspace-name>/
          sessions/
            <session-id>/
              meta.toml
              log
```

### `index.db`

Global repository index. Used by `fold repo list`, `fold repo add`, and `fold repo remove`.

Stores one row per repository:
- `host`
- `owner`
- `name`
- `alias`
- `created_at`

Canonical identity: the `(host, owner, name)` tuple. Aliases are shortcuts, not primary identifiers.

### `repos/<host>/<owner>/<name>/`

Per-repository directory. Example:

```text
~/.local/share/fold/repos/github.com/penberg/fold/
```

Contains repository-specific state and worktree directories.

### `meta.toml`

Repository metadata. Example:

```toml
host = "github.com"
owner = "penberg"
name = "fold"
canonical = "github.com/penberg/fold"
alias = "fold"
```

### `repo.db`

Repository-local database. Stores:
- workspaces
- sessions
- artifacts
- event history

### `workspaces/<workspace-name>/`

Git worktree checkout directories. Example:

```text
~/.local/share/fold/repos/github.com/penberg/fold/workspaces/main/
```

The workspace name is the identifier within that repository.

### `sessions/<session-id>/`

Per-session runtime state. Example:

```text
~/.local/share/fold/repos/github.com/penberg/fold/sessions/01JSESSIONEXAMPLE/
```

This directory is intended to hold session-local files such as:
- metadata
- logs
- PTY or attachment state

Sessions are stored under the repository shard because they are attached to workspaces in that repository.

### Notes

- `index.db` is global. `repo.db` is per-repository.
- On-disk paths derive from the canonical identifier, not the alias.
- Workspace paths derive from the workspace name.
- Session paths derive from a session identifier.

## Command Line Reference

### `fold session`

Manage workspace sessions.

Future TUI terminal tabs should map to sessions, not directly to workspaces.

### `fold session create`

Create a session for a workspace.

#### Usage

```text
fold session create <workspace> -- <command> [args...]
```

#### Arguments

- `<workspace>`: Workspace reference.

#### Behavior

- Resolves the target workspace.
- Creates a session record in `repo.db`.
- Starts the requested command in the workspace directory.
- Persists enough metadata for later inspection or attachment.

#### Examples

```text
fold session create fold/main -- bash
fold session create fold/github-actions -- cargo test
```

### `fold session list`

List sessions.

#### Usage

```text
fold session list
fold session list <workspace>
fold session list --json
```

#### Arguments

- `<workspace>`: Optional workspace reference.

#### Options

- `--json`: Emit machine-readable output.

#### Behavior

- Lists sessions known to local `fold` state.
- When `<workspace>` is provided, filters to that workspace.

### `fold session info`

Show metadata for one session.

#### Usage

```text
fold session info <session>
```

#### Arguments

- `<session>`: Session identifier.

#### Behavior

- Prints session metadata such as workspace, command, status, and timestamps.

### `fold session attach`

Attach to an existing session.

#### Usage

```text
fold session attach <session>
```

#### Arguments

- `<session>`: Session identifier.

#### Behavior

- Connects to a running session.
- Intended for interactive terminal use.
- Press `Ctrl-]` to detach without terminating the session.
- `Ctrl-D` is passed through to the attached process and may cause shells to exit.

### `fold session stop`

Stop a running session.

#### Usage

```text
fold session stop <session>
```

#### Arguments

- `<session>`: Session identifier.

#### Behavior

- Requests session termination.
- Marks the session as stopped in local state.

### `fold prune`

Prune local state.

### `fold prune sessions`

Prune stopped, failed, or exited sessions.

#### Usage

```text
fold prune sessions
```

#### Behavior

- Scans all repositories known to local `fold` state.
- Removes sessions whose status is `stopped`, `failed`, or `exited`.
- Leaves `starting` and `running` sessions untouched.

#### Expected Output

```text
Pruned 2 sessions
```

### `fold workspace`

Manage repository workspaces. Alias: `fold ws`.

### `fold workspace create`

Create a workspace for a repository.

#### Usage

```text
fold workspace create <repository> [name]
fold ws create <repository> [name]
```

#### Arguments

- `<repository>`: Repository alias or canonical `host/owner/name`.
- `[name]`: Optional workspace name.

#### Behavior

- Resolves the repository by alias first, then by canonical identifier.
- Creates a workspace record in `repo.db`.
- Creates a git worktree under `repos/<host>/<owner>/<name>/workspaces/<workspace-name>/`.
- Starts a default session for the new workspace using the user's login shell.
- Rejects duplicate workspace names within the same repository.

#### Examples

```text
fold workspace create fold
fold workspace create fold feature-x
fold ws create github.com/penberg/fold review-docs
```

#### Expected Output

```text
Created workspace main for fold
Created session 01JSESSIONEXAMPLE
```

### `fold workspace clone`

Clone a workspace into a new workspace.

#### Usage

```text
fold workspace clone <workspace> <name>
fold ws clone <workspace> <name>
```

#### Arguments

- `<workspace>`: Source workspace reference.
- `<name>`: New workspace name.

#### Behavior

- Resolves the source workspace.
- Creates a new git worktree under `repos/<host>/<owner>/<name>/workspaces/<workspace-name>/`.
- Creates a new branch named `<name>` from the source workspace's current committed `HEAD`.
- Starts a default session for the cloned workspace using the user's login shell.
- Rejects duplicate workspace names within the same repository.

#### Examples

```text
fold workspace clone fold:main feature-x
fold ws clone fold/bugfix bugfix-copy
```

#### Expected Output

```text
Cloned workspace fold:main to feature-x for fold
Created session 01JSESSIONEXAMPLE
```

### `fold workspace list`

List workspaces for a repository.

#### Usage

```text
fold workspace list <repository>
fold ws list <repository>
fold workspace list <repository> --json
```

#### Arguments

- `<repository>`: Repository alias or canonical `host/owner/name`.

#### Options

- `--json`: Emit machine-readable output.

#### Behavior

- Reads workspace records from `repo.db`.
- Prints one workspace per row.

#### Examples

```text
fold workspace list fold
fold ws list github.com/penberg/fold --json
```

### `fold workspace info`

Show metadata for one workspace.

#### Usage

```text
fold workspace info <workspace>
fold ws info <workspace>
```

#### Arguments

- `<workspace>`: Workspace reference.

#### Behavior

- Prints workspace metadata: repository, name, path, creation time.

### `fold workspace remove`

Remove a workspace.

#### Usage

```text
fold workspace remove <workspace>
fold ws remove <workspace>
```

#### Arguments

- `<workspace>`: Workspace reference.

#### Behavior

- Removes the workspace record from the repository-local database.
- Removes the corresponding git worktree.
- Removes the workspace directory on disk.

### `fold repo add`

Register a repository with `fold`.

#### Usage

```text
fold repo add <host/owner/name|remote-url> [--alias <name>]
```

#### Arguments

- `<host/owner/name|remote-url>`: Canonical repository identifier or full git remote URL.

#### Options

- `--alias <name>`: Optional local shorthand for the repository. If omitted, `fold` uses the repository name as the default alias.

#### Behavior

- Validates `host/owner/name` format or parses a full git remote URL.
- Creates a repository record in `index.db`.
- Stores the exact remote URL when one is provided explicitly.
- Defaults the alias to the repository name if `--alias` is omitted.
- Rejects duplicates.

#### Examples

```text
fold repo add github.com/penberg/fold
fold repo add github.com/penberg/other --alias other
fold repo add git@github.com:penberg/private-repo.git --alias private
```

#### Expected Output

```text
Added repo fold
```

### `fold repo list`

List repositories registered with `fold`.

#### Usage

```text
fold repo list
fold repo list --json
```

#### Options

- `--json`: Emit machine-readable output.

#### Behavior

- Reads repository records from `index.db`.
- Prints one repository per row.

#### Examples

```text
fold repo list
fold repo list --json
```

#### Expected Output

```text
ALIAS            REPOSITORY
fold            github.com/penberg/fold
```

### `fold repo sync`

Sync a registered repository from its remote.

#### Usage

```text
fold repo sync <repository>
```

#### Arguments

- `<repository>`: Repository alias or canonical `host/owner/name`.

#### Behavior

- Resolves the repository by alias first, then by canonical identifier.
- Creates or updates the local bare clone at `repos/<host>/<owner>/<name>/source.git`.
- Fetches all remotes and prunes deleted refs when the bare clone already exists.

#### Examples

```text
fold repo sync fold
fold repo sync github.com/penberg/fold
```

#### Expected Output

```text
Synced repo fold
```

### `fold repo remove`

Remove a repository from `fold`.

#### Usage

```text
fold repo remove <repository>
```

#### Arguments

- `<repository>`: Repository alias or canonical `host/owner/name`.

#### Behavior

- Resolves the repository by alias first, then by canonical identifier.
- Removes the row from `index.db`.
- Removes the repository directory under `repos/<host>/<owner>/<name>/`.

#### Examples

```text
fold repo remove fold
fold repo remove github.com/penberg/fold
```

#### Expected Output

```text
Removed repo fold
```

## Exit Codes

- `0`: Command succeeded.
- `1`: Runtime or IO failure.
- `2`: Invalid arguments or unknown command usage.
- `3`: Repository not found.
- `4`: Repository already exists or alias conflict.
