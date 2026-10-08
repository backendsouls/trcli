# Contracts: Integrations and Google Drive

The interfaces this specification exposes to users and scripts. Each file is the contract
for one group of commands; changing one after release is a breaking change.

Shared rules — grammar, global options, verbs every record has, output forms, and exit
codes — are in the [CLI conventions](../../000-foundation/contracts/cli-conventions.md) and are not repeated in each file.

| Commands | Contract | Covers |
|----------|----------|--------|
| `trcli integration` | [cli-integration.md](./cli-integration.md) | User Stories 1, 2, and 6; FR-001 to FR-023, FR-053 to FR-059 |
| `trcli sync` | [cli-sync.md](./cli-sync.md) | User Story 3; FR-024 to FR-037 |
| `trcli file` | [cli-file.md](./cli-file.md) | User Story 4; FR-038 to FR-047 |
| `trcli backup remote` | [cli-backup-remote.md](./cli-backup-remote.md) | User Story 5; FR-048 to FR-052 |

## Rules shared by these four

- **Nothing is sent without a connection**, and nothing is sent by a command that does not
  say it sends. With no connection, `trcli` behaves exactly as it does without this
  specification.
- **`--dry-run`** on every sending or receiving command lists what would be transferred and
  transfers nothing.
- **`--connection <name>`** chooses the connection when several allow the capability.
- **Exit code 7** means the service could not do what was asked (unreachable, out of space,
  access withdrawn); the message says which, and local data is never affected.
- **Without a person present**, a command that would need sign-in, a passphrase, or a
  confirmation fails at once with exit code 5.
- **Credentials and passphrases** never appear in output, logs, or files these commands
  produce, and are never accepted as command-line arguments.

Technical findings about signing in to Google, for planning, are in
[../notes/google-sign-in.md](../notes/google-sign-in.md).
