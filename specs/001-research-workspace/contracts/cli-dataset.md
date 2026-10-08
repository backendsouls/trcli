# Contract: `trcli dataset`

**Spec**: [Research Workspace](../spec.md) — User Story 7; FR-040 to FR-042

**Conventions**: [grammar, global options, shared verbs](../../000-foundation/contracts/cli-conventions.md) · [output and exit codes](../../000-foundation/contracts/output-and-exit-codes.md)

**Handle prefix**: `dat`, `dv`

## Synopsis

```text
trcli dataset add --name <text> --location <path-or-url> [--description <text>] [--origin <text>] [--license <text>]
trcli dataset verify <ref>
trcli dataset version add <ref> [--note <text>]
trcli dataset version list <ref>
```

The shared verbs `list`, `show`, `edit`, `rm`, `tag`, and `note` apply and are not repeated.

## Commands

| Command | Behavior | Fails with (exit code) |
|---------|----------|------------------------|
| `dataset add` | Registers a dataset by reference and records version 1 with its fingerprint | 2 when a local location does not exist |
| `dataset verify` | Reports `unchanged`, `changed`, or `unreachable` | 6 `check_failed` when changed or unreachable |
| `dataset version add` | Records a new version from the current content | 2 when the content has not changed |
| `dataset version list` | Versions with fingerprint, size, and date | — |
| `dataset rm` | Shared verb | 5 `blocked_by_dependents` when a run used a version |

## Values

| Value | Rule |
|-------|------|
| `--location` | a local path (absolute, or relative to the workspace) or an `http(s)` address; an address is recorded but not fingerprinted |
| `--license` | up to 100 characters |

## Example

```console
$ trcli dataset verify dat-3n8x
dat-3n8x "Train set": changed since version 1 (recorded 2026-10-02)
  Record the new content with: trcli dataset version add dat-3n8x
$ echo $?
6
```
