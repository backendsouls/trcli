# Contract: `trcli file`

**Spec**: [Integrations and Google Drive](../spec.md) — User Story 4; FR-038 to FR-047

**Conventions**: [grammar, global options, shared verbs](../../000-foundation/contracts/cli-conventions.md) · [output and exit codes](../../000-foundation/contracts/output-and-exit-codes.md)

The files meant here are those records refer to: a dataset's content, a figure or table, a
manuscript's files, a reference's local copy, a software location that is a local path.

## Synopsis

```text
trcli file status [<record-ref>...] [--kind <record-kind>] [--tag <tag>] [--only <state>]
trcli file push  (<record-ref>... | --kind <record-kind> | --tag <tag> | --all) [--dry-run]
trcli file pull  (<record-ref>... | --kind <record-kind> | --tag <tag> | --all) [--dry-run]
trcli file free  (<record-ref>... | --kind <record-kind> | --tag <tag> | --all) [--dry-run]
trcli file versions <record-ref>
```

`--project <ref>` and `--all-projects` (global options) narrow or widen every form.

## Commands

| Command | Behavior | Fails with (exit code) |
|---------|----------|------------------------|
| `file status` | For each referenced file: `local only`, `remote only`, `in step` (both and identical), or `differs`; with sizes | 6 `check_failed` when any file `differs`; 0 otherwise. Without a network, says the remote side is as last known |
| `file push` | Sends files to a connection that allows `files`; each is checked against its fingerprint after arrival. Identical files are not sent again. Shows progress; can be interrupted and continued | 7 when unreachable or when the space left is not enough (said before anything is sent); 5 for a dataset flagged personal or sensitive, or a record marked private, without confirmation — the acknowledgement is recorded; 5 on the first send of files without confirmation; 2 when a local file is missing |
| `file pull` | Fetches files to where their records expect them and checks each against its fingerprint | 6 `check_failed` for each file that does not match — it is not put in place; 7 when a remote file is missing, changing nothing locally; 5 when a different local file would be replaced and `--yes` is not given |
| `file free` | Removes local files that have an identical remote copy; nothing else is ever removed | 5 `confirmation_required` |
| `file versions` | The remote contents kept for a record's file and which dataset version, result, or manuscript version each belongs to | — |
| (a command that needs a remote-only file) | `dataset verify`, `run start`, `figure verify`, `manuscript version add`, and others say the file must be fetched first and offer to fetch it | 5 when it cannot ask |

## Values

| Value | Rule |
|-------|------|
| `<state>` | `local-only`, `remote-only`, `in-step`, `differs` |
| a changed file sent again | the earlier remote content stays retrievable for whatever recorded it |
| a directory | sent and fetched as a whole, with one progress indication; names the service does not allow are stored under safe names and restored on fetch |
| excluded records | their files are never sent; `file status` shows them as `kept here` |
| settings | `files.connection`, `files.confirm_sensitive` (always `true`; cannot be turned off) |

## Example

```console
$ trcli file status --kind dataset
RECORD     FILE                 SIZE      STATE
dat-3n8x   data/train           1.2 GB    in step
dat-7p2q   data/interviews      340 MB    local only     sensitive
dat-9r4s   data/benchmark       8.4 GB    remote only

$ trcli file pull dat-9r4s
Fetching data/benchmark  8.4 GB  ████████████████████ 100%  11m02s
Checked against fingerprint ✔  dat-9r4s "Benchmark" version 2

$ trcli file push dat-7p2q
warning: dat-7p2q "Interviews" holds sensitive data. It will be stored in your Google Drive
         (drive, ana.silva@example.org). Passphrase protection is off for this connection.
Send it and record your acknowledgement? [y/N]
```
