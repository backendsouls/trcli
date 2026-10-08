# Contract: `trcli member, sync`

**Spec**: [Research Lifecycle Extensions](../spec.md) — User Story 17; FR-064 to FR-069

**Conventions**: [grammar, global options, shared verbs](../../000-foundation/contracts/cli-conventions.md) · [output and exit codes](../../000-foundation/contracts/output-and-exit-codes.md)

**Handle prefix**: `mbr`

> `trcli sync push`, `pull`, `status`, and conflict handling are now defined by
> `specs/010-integrations` ([cli-sync.md](../../010-integrations/contracts/cli-sync.md)) for one
> researcher's machines. This contract adds only what collaboration needs on top: members
> and roles.

## Synopsis

```text
trcli member add <name> --role <reader|editor|administrator>
trcli member list | role <name> <role> | rm <name>
trcli sync push | pull | status
trcli conflict list
trcli conflict resolve <conflict-ref> --mine | --theirs
```

## Commands

| Command | Behavior | Fails with (exit code) |
|---------|----------|------------------------|
| `member add` / `rm` | The owner grants or removes access | 5 when not the owner or an administrator |
| `sync push` / `pull` | Exchanges changes with the shared copy; workspaces converge | 7 when the shared copy cannot be reached; 5 when access was removed |
| `sync status` | Changes waiting to be sent or received, and open conflicts | 6 when conflicts are open |
| `conflict resolve` | Chooses one of two conflicting values; nothing is overwritten before | — |
| (any change by a reader) | Refused | 5 |

## Values

| Value | Rule |
|-------|------|
| note | how members are identified and where the shared copy lives is decided at planning time |

## Example

```console
$ trcli sync pull
Received 12 changes from 2 members. 1 conflict needs your choice:
  cnf-4h2s  ms-5t1q "Our Paper" · deadline   mine: 2027-01-15   bruno: 2027-02-01
```
