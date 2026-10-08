# Contract: `trcli integration`

**Spec**: [Integrations and Google Drive](../spec.md) — User Stories 1, 2, and 6; FR-001 to FR-023, FR-053 to FR-059

**Conventions**: [grammar, global options, shared verbs](../../000-foundation/contracts/cli-conventions.md) · [output and exit codes](../../000-foundation/contracts/output-and-exit-codes.md)

**Handle prefix**: `con` (connection) — a connection is usually named instead: `drive`, `drive-lab`

## Synopsis

```text
trcli integration list
trcli integration info <integration>
trcli integration connect <integration> [--name <name>] [--only <capability>]... [--no-browser]
                          [--folder <name>] [--timeout <span>]
trcli integration status [<connection>]
trcli integration limit <connection> --only <capability>... | --all
trcli integration reconnect <connection> [--no-browser]
trcli integration disconnect <connection> [--purge]
trcli integration stored <connection>
trcli integration purge <connection>
trcli integration protect <connection> [--change]
trcli integration log [--connection <name>] [--direction <sent|received>] [--kind <kind>]
                      [--from <date>] [--to <date>] [--failed] [--to-file <file>]
trcli integration exclude add (--kind <record-kind> | --tag <tag> | <record-ref>)...
trcli integration exclude list | rm <n>
```

Every command that sends or receives — here and in the other contracts of this
specification — accepts `--connection <name>` (needed only when several fit) and
`--dry-run` (list what would be transferred, transfer nothing).

## Commands

| Command | Behavior | Fails with (exit code) |
|---------|----------|------------------------|
| `integration list` | Integrations available, what each offers, and the connections that exist | — |
| `integration info` | In plain words: what is sent to the service, what is read from it, what is stored on this machine, and the access it asks for | 3 on unknown integration, listing those available |
| `integration connect google-drive` | Opens Google's sign-in in the browser and waits for approval; with `--no-browser`, or when no browser can be opened, prints a web address and a short code to approve on another device. On success stores the connection and names the account | 7 when the approval is denied, abandoned, or not completed within `--timeout` (nothing is stored); 7 with cause `blocked_by_administrator` when an institution's policy refuses it; 7 `no_connection` without a network; 5 when it cannot ask a person; 2 when the name is in use |
| `integration connect` (no protected store) | Says so and asks whether to keep credentials in a file readable only by the user, or not to connect | 5 when it cannot ask |
| `integration status` | Account, access granted, capabilities allowed, last use, whether it works now, space used and available. Without a name: every connection | 6 `check_failed` when a connection does not work, saying whether access expired, was withdrawn, or the service is unreachable |
| `integration limit` | Restricts what a connection may be used for; other uses are then refused with exit code 5 | 2 on unknown capability |
| `integration reconnect` | Signs in again for an existing connection whose access expired or was withdrawn; nothing local is lost | as `connect` |
| `integration disconnect` | Removes credentials from this machine and asks the service to withdraw the access; says what remains at the service. `--purge` also deletes it | 5 `confirmation_required`; 0 with a warning when the service could not be told |
| `integration stored` | What is kept at the service for this connection, by kind and size | 7 when unreachable |
| `integration purge` | Deletes everything TRCLI stored at the service for this connection | 5 `confirmation_required`, asked twice |
| `integration protect` | Turns on passphrase protection for a connection: asks for the passphrase twice, keeps it in the protected store, and makes everything sent unreadable at the service, names included. States that content cannot be recovered without it | 5 when it cannot ask; 2 when the two entries differ or it is shorter than 12 characters |
| `integration log` | The transfer log: time, connection, machine, direction, kind, amount, outcome | 2 when `--to` is before `--from` |
| `integration exclude add` | Keeps kinds of record, tagged records, or particular records on this machine; says which links will be incomplete elsewhere | 3 when a record is not found |

## Values

| Value | Rule |
|-------|------|
| `<integration>` | `google-drive` (the only one in this specification) |
| `<capability>` | `sync`, `files`, `backup` |
| `--name` | 1–40 characters of `a-z 0-9 -`; defaults to `drive` for the first Google Drive connection |
| `--folder` | the folder TRCLI creates in the Drive; 1–100 characters; default `TRCLI` |
| `--timeout` | how long to wait for approval; default `5m`, at most `15m` |
| credentials and passphrases | never printed, logged, exported, backed up, or synced; never accepted as command-line arguments |
| settings | `integration.confirm_first_send` (default `true`), `integration.retry_limit` (default `5`) |
| first send of a kind of content | asks for confirmation once per connection and kind; `--yes` answers it |
| non-interactive use | any command that would need sign-in or a passphrase fails at once with exit code 5 |

## Example

```console
$ trcli integration info google-drive
Google Drive — sync, files, backup
  Access asked    only the files and folders TRCLI itself creates in your Drive
  Sent            your workspace's records, and the files you choose to send
  Read            only what TRCLI wrote there
  Kept here       a credential, in this machine's protected store for secrets
  Never           your Google password; your other Drive files; telemetry

$ trcli integration connect google-drive
Opening your browser to sign in with Google…
Waiting for approval (5 minutes) ✔
Connected "drive" as ana.silva@example.org · folder "TRCLI" · sync, files, backup

$ trcli integration connect google-drive --name drive-lab --no-browser
On any device, open  https://www.google.com/device  and enter the code  QHTR-LMNP
Waiting for approval (5 minutes) ✔
Connected "drive-lab" as ana.silva@example.org · folder "TRCLI" · sync, files, backup
```
