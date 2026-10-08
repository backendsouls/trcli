# Contract: `trcli sync`

**Spec**: [Integrations and Google Drive](../spec.md) — User Story 3; FR-024 to FR-037

**Conventions**: [grammar, global options, shared verbs](../../000-foundation/contracts/cli-conventions.md) · [output and exit codes](../../000-foundation/contracts/output-and-exit-codes.md)

**Handle prefix**: `cnf` (conflict)

Sync here is between one researcher's own machines. `specs/002-research-lifecycle` builds
collaboration between several people on these same commands.

## Synopsis

```text
trcli sync publish [--connection <name>] [--as <remote-name>] [--machine <name>]
trcli sync remotes [--connection <name>]
trcli sync obtain <remote-name> [<dir>] [--connection <name>] [--machine <name>]
trcli sync [--dry-run]
trcli sync push [--dry-run]
trcli sync pull [--dry-run]
trcli sync status
trcli sync conflicts
trcli sync resolve <conflict-ref> (--mine | --theirs)
trcli sync resolve --all (--mine | --theirs)
trcli sync auto <on|off|remind>
trcli sync machines
trcli sync machine rename <name> | remove <name>
trcli sync unlink [--delete-remote]
trcli sync upgrade-remote
```

## Commands

| Command | Behavior | Fails with (exit code) |
|---------|----------|------------------------|
| `sync publish` | Creates a remote copy of this workspace at a connection that allows `sync` and links the workspace to it | 2 when already linked, or the remote name exists; 5 on the first send without confirmation; 7 when the service is unreachable or out of space |
| `sync remotes` | Published workspaces at a connection, with last change and the machines linked | — |
| `sync obtain` | Creates a complete local workspace from a remote copy, linked to it | 4 `workspace_exists` when `<dir>` holds one; 4 `workspace_too_new` when the remote copy was written by a newer version; 3 when the remote name is not found; 5 when the remote copy is passphrase-protected and it cannot ask |
| `sync` | Receives, then sends | as `pull` and `push` |
| `sync push` | Sends local changes and reports how many | 6 when the remote copy has changes not yet received (receive first; nothing is lost); 7 when unreachable; 0 with a message when there is nothing to send |
| `sync pull` | Receives changes and reports what changed, by kind of record. Combines changes to different records, and to different details of one record, without asking | 6 `check_failed` when conflicts were found — everything else is applied and the conflicts wait; 4 `workspace_too_new`; 1 when the remote content is not what TRCLI wrote, changing nothing |
| `sync status` | Changes waiting to be sent, whether there is something to receive, last sync, open conflicts, and files that are remote only | 6 when conflicts are open; 0 otherwise. Works without a network, saying that the remote side is unknown |
| `sync conflicts` | Each conflict: the record and detail, both values, their machines, and their times | — |
| `sync resolve` | Chooses one side; nothing was overwritten before | 3 when the conflict is not found |
| `sync auto` | `on`: sync after commands that change the workspace; `remind`: say when changes are not yet sent; `off`. Automatic sync never resolves a conflict and never makes the command fail | — |
| `sync machines` | Machines linked to the remote copy, with their names and last sync | — |
| `sync machine remove` | Removes a machine's link; says that content already on that machine cannot be recalled | 5 `confirmation_required` |
| `sync unlink` | Unlinks this workspace; the local workspace stays complete. `--delete-remote` also deletes the remote copy | 5 `confirmation_required`, asked twice for `--delete-remote` |
| `sync upgrade-remote` | Upgrades the remote copy to this version's format after a backup; other machines must upgrade before they sync | 5 `confirmation_required` |

## Guarantees

| Guarantee | Meaning |
|-----------|---------|
| Nothing is lost silently | A change is either applied on every machine or shown as a conflict for the researcher to decide |
| Add-only records stay whole | Audit entries, notebook entries, frozen pre-registrations, and saved reports from every machine are all kept, unaltered; `trcli audit verify` still passes |
| Interruptions are safe | After a dropped network or a closed laptop, the local workspace and the remote copy are both valid and the command can simply be run again |
| The working storage stays local | Only what TRCLI writes for exchange is placed at the service, never the workspace's working storage itself |
| Offline first | Every command that does not sync works with no network, for as long as needed |
| Clocks are not trusted alone | The order of changes does not depend on machines' clocks alone; a clearly wrong clock is reported |
| Unrelated workspaces are not merged | Two workspaces that do not share a history are never combined |

## Values

| Value | Rule |
|-------|------|
| `--as`, `<remote-name>` | 1–100 characters; defaults to the workspace's name |
| `--machine` | this machine's name in conflicts and logs; 1–40 characters; defaults to the computer's name |
| settings | `sync.auto` (`off`, `remind`, `on`; default `remind`), `sync.connection` |
| a deletion against a change | is a conflict, like two changes to the same detail |
| a run in progress on another machine | is shown as running elsewhere and cannot be resumed here |

## Example

```console
$ trcli sync publish
This will send your workspace's records to Google Drive (drive, ana.silva@example.org). Continue? [y/N] y
Published "Doctorate" · 14,208 changes sent · this machine is "laptop"

# later, on the workstation
$ trcli sync obtain Doctorate ~/research --machine workstation
Obtained "Doctorate" into /home/ana/research · 10,412 records · 37 files are remote only (trcli file status)

$ trcli sync
Received 12 changes from "laptop": 6 references, 4 annotations, 2 tasks
Sent 3 changes
1 conflict needs your choice:
  cnf-4h2s  ms-5t1q "Our Paper" · deadline
            laptop       2027-01-15   changed 2026-10-07 18:02
            workstation  2027-02-01   changed 2026-10-08 09:40
$ echo $?
6

$ trcli sync resolve cnf-4h2s --theirs
ms-5t1q deadline is now 2027-01-15 (from laptop)
```
