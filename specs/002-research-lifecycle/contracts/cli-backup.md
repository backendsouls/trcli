# Contract: `trcli backup, restore, export, upgrade`

**Spec**: [Research Lifecycle Extensions](../spec.md) — User Story 3; FR-013 to FR-019

**Conventions**: [grammar, global options, shared verbs](../../000-foundation/contracts/cli-conventions.md) · [output and exit codes](../../000-foundation/contracts/output-and-exit-codes.md)

## Synopsis

```text
trcli backup create --to <file> [--with-files]
trcli backup verify <file>
trcli backup restore <file> [--into <dir>] [--replace]
trcli workspace export --to <dir>
trcli workspace upgrade [--check]
```

## Commands

| Command | Behavior | Fails with (exit code) |
|---------|----------|------------------------|
| `backup create` | Writes every record, link, and audit entry to one file and verifies it | 1 when there is not enough space; nothing partial is left |
| `backup verify` | Checks a backup file is complete and undamaged | 6 `check_failed` |
| `backup restore` | Recreates the workspace in an empty location | 4 `workspace_exists` unless `--replace`; 6 on a damaged file; 4 `workspace_too_new` for a newer backup |
| `workspace export` | Writes all records in an open, documented form readable without the tool | 5 when the directory is not empty |
| `workspace upgrade` | Upgrades the workspace format after taking a backup; `--check` only reports | 1 and the workspace is left as it was when the upgrade fails |

## Values

| Value | Rule |
|-------|------|
| `--with-files` | also includes files the workspace refers to; off by default |
| `--replace` | required to restore over an existing workspace; asks for confirmation |

## Example

```console
$ trcli workspace upgrade --check
This workspace uses format 3; this version of trcli uses format 5.
An upgrade is needed. Run: trcli workspace upgrade   (a backup is taken first)
```
