# Contract: `trcli backup remote`

**Spec**: [Integrations and Google Drive](../spec.md) — User Story 5; FR-048 to FR-052

**Conventions**: [grammar, global options, shared verbs](../../000-foundation/contracts/cli-conventions.md) · [output and exit codes](../../000-foundation/contracts/output-and-exit-codes.md)

Extends `trcli backup` of `specs/002-research-lifecycle`
([cli-backup.md](../../002-research-lifecycle/contracts/cli-backup.md)), which makes and
restores backups as local files. A remote backup is the same backup, kept at a connection.

## Synopsis

```text
trcli backup remote create [--connection <name>] [--with-files] [--dry-run]
trcli backup remote list [--connection <name>] [--workspace-name <text>]
trcli backup remote verify <backup>
trcli backup remote restore <backup> [--into <dir>] [--replace]
trcli backup remote rm <backup>
trcli backup remote keep <n>
```

## Commands

| Command | Behavior | Fails with (exit code) |
|---------|----------|------------------------|
| `backup remote create` | Makes a backup, verifies it, sends it to a connection that allows `backup`, and verifies it again after arrival. Then removes the oldest backups beyond the number to keep, naming them | 7 when sending or the second verification fails — no earlier backup is removed; 7 when space is not enough, said before sending; 5 on the first send without confirmation |
| `backup remote list` | Remote backups with date, size, the workspace each is of, the version of TRCLI that made it, and whether it includes files | — |
| `backup remote verify` | Checks a remote backup without restoring it | 6 `check_failed` |
| `backup remote restore` | Fetches a backup, verifies it, and recreates the workspace in an empty location | 4 `workspace_exists` unless `--replace`; 4 `workspace_too_new`; 6 on a damaged backup, changing nothing; 5 when a passphrase is needed and it cannot ask |
| `backup remote rm` | Deletes one remote backup | 5 `confirmation_required` |
| `backup remote keep` | Sets how many remote backups to keep for this workspace | 2 when not a whole number ≥ 1 |

## Values

| Value | Rule |
|-------|------|
| `<backup>` | the date and time shown by `list`, or `latest` |
| `--with-files` | also includes the files records refer to; off by default |
| passphrase protection | when on for the connection (`trcli integration protect`), backups are unreadable at the service and restoring asks for the passphrase; without it a backup cannot be recovered, and the tool says so plainly |
| independence from sync | a remote backup can be restored when the synced remote copy no longer exists; a restored workspace that is later synced is treated as old and receives what is newer, after showing what will change |
| settings | `backup.remote_keep` (default `5`), `backup.connection` |

## Example

```console
$ trcli backup remote create
Backup made and verified locally (48 MB)
Sending to Google Drive (drive)  ████████████████████ 100%
Verified after arrival ✔  2026-10-08 16:20
Keeping 5: removed 2026-09-03 09:11

$ trcli backup remote restore latest --into ~/recovered
Fetched and verified 2026-10-08 16:20 (48 MB)
Restored "Doctorate" into /home/ana/recovered · 10,412 records · identical to the backup
```
