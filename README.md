<p align="center">
  <img src=".github/assets/hero.png" width="600" alt="Fold">
</p>

<h1 align="center">Fold</h1>
<p align="center"><em>Fold your agents, not your terminals.</em></p>

Fold lets you run Claude Code, Codex, OpenCode, and other coding agents in parallel — each in its own isolated git worktree, each in a persistent terminal session. Manage them from a native GTK desktop app or the CLI, similar to [cmux](https://github.com/manaflow-ai/cmux), [Superset](https://github.com/superset-sh/superset), and [Conductor](https://www.conductor.build/).

## Features

- **Works with any coding agent** — run Claude Code, Codex, OpenCode, or any other CLI tool inside a session. Sessions are just terminals, so anything that runs in a shell runs in Fold.
- **Workspace per feature using git worktrees** — every workspace is an isolated git worktree, so parallel agents never share a checkout or step on each other's branches.
- **Fast workspace cloning** — branch off an existing workspace into a new worktree and branch in one command.
- **Multi-repository management** — register any number of GitHub repositories and manage their workspaces and sessions from one place.
- **Ghostty-backed persistent terminals** — sessions are backed by `libghostty-vt` and survive disconnects; detach and reattach from the GUI or CLI without killing the process.

## Installation

Fold runs on Linux. Install the latest prebuilt release with:

```console
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/penberg/fold/releases/latest/download/fold-installer.sh | sh
```

Then start the GUI with:

```console
fold
```

Or drive Fold from the command line with `foldctl`:

```console
foldctl --help
```

For more information, check out the [documentation section](#documentation).

## Motivation

Coding agents are most effective when you can run many of them in parallel — one per feature, one per bug fix. But in practice, this quickly turns into chaos. Each agent needs its own copy of the source code so they don't step on each other's changes, and each one runs in its own terminal. Before long, you're drowning in terminal tabs, losing track of which agent is working on what, and manually juggling git branches and directories.

Fold exists to tame that. It gives you a single place to manage all your repositories, workspaces, and agent sessions — so you can scale up the number of parallel agents without the overhead of keeping it all organized yourself.

## How it works

Fold organizes work into **repositories**, **workspaces**, and **sessions**. A repository is the git repository that holds your project's source code. A workspace is an isolated copy of that source code, backed by a git worktree, where work happens independently without interfering with other workspaces. A session is a persistent terminal environment running inside a workspace — it could be a coding agent, a shell, or any long-running command.

For example, a typical **workspace-per-feature** workflow looks like this: create a workspace for every feature or bug fix, let a coding agent work on it in a session, submit a pull request, address review comments and iterate — all within the same workspace. When the work is merged to mainline, remove the workspace.

## Building from source

The GTK desktop app depends on GTK 4 development headers and a Zig toolchain for the vendored `libghostty-vt` build. Fold currently requires `zig 0.15.2`.

On Ubuntu/Debian, install the same system packages used by the release build:

```sh
sudo apt install gcc g++ libcairo2-dev libglib2.0-dev libgtk-4-dev libpango1.0-dev pkg-config
```

Then install Zig 0.15.2 and make sure `zig` is on your `PATH`.

Install the program with:

```sh
cargo install --path .
```

## Documentation

See [MANUAL.md](MANUAL.md) for the full reference.

## License

Fold is available under the [MIT license](LICENSE.md).
