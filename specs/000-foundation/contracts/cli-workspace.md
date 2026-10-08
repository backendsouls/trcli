# Contract: `trcli init, workspace, config, completions`

**Spec**: [Foundation and Architecture](../spec.md) — User Stories 1 and 5; FR-001 to FR-009, FR-039 to FR-045

**Conventions**: [grammar, global options, shared verbs](./cli-conventions.md) · [output and exit codes](./output-and-exit-codes.md)

## Synopsis

```text
trcli init [<dir>] --name <name> [--description <text>]
trcli workspace show
trcli workspace edit [--name <name>] [--description <text>] [--researcher <name>]
trcli workspace upgrade [--check]
trcli workspace check
trcli config list | get <key> | set <key> <value> [--user] | unset <key> [--user] | path
trcli completions <bash|zsh|fish|powershell>
```

## Commands

| Command | Behavior | Fails with (exit code) |
|---------|----------|------------------------|
| `init` | Creates `.trcli/` and an empty workspace in `<dir>` (default: current directory) | 4 `workspace_exists` when one is already there; 2 when `--name` is missing or empty |
| `workspace show` | Name, description, location, format version, number of records of each kind | 4 `no_workspace` |
| `workspace edit` | Changes only the details given | 2 on invalid values |
| `workspace upgrade` | Brings an older workspace to the current format: keeps a copy in `.trcli/backups/` first, then upgrades; `--check` only reports whether one is needed | 1 with the workspace restored from the copy when the upgrade fails; 0 with a message when none is needed |
| `workspace check` | Verifies the stored data can be opened and is consistent, and that the audit trail is intact | 4 `workspace_damaged` or 6 `check_failed`, saying what and where |
| `config list` | Every effective setting and the source it came from | — |
| `config get` / `set` / `unset` | Reads or writes one setting; `--user` targets the user file | 2 on unknown key or invalid value |
| `config path` | The settings files in use | — |
| `completions` | Prints a completion script for the shell | 2 on unknown shell |

## Values

| Value | Rule |
|-------|------|
| `--name` | 1–200 characters, required for `init` |
| `--description` | up to 20,000 characters |
| `--researcher` | 1–200 characters; recorded as the actor in the audit trail |
| `<key>` | a key listed in [configuration.md](./configuration.md) |

## Example

```console
$ trcli init --name "Doctorate"
Created workspace "Doctorate" in /home/ana/research/.trcli

$ trcli init --name "Again"
error: a workspace already exists in /home/ana/research/.trcli
Nothing was changed.
$ echo $?
4
```
