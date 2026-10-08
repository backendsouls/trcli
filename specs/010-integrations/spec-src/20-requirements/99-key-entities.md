### Key Entities *(include if feature involves data)*

- **Integration**: An outside service TRCLI can work with, described by the capabilities it
  offers and the access it needs. Google Drive is the first.
- **Capability**: Something an integration can do for a workspace: workspace sync, file
  storage, backup destination.
- **Connection**: A named, authorized link between this machine's user and one account at an
  integration, with the access granted and the capabilities it may be used for.
- **Credentials**: What proves to the service that a connection was approved. Kept in the
  machine's protected store; never part of a workspace.
- **Remote Copy**: The published form of a workspace at a connection, which linked machines
  send to and receive from.
- **Machine**: One computer linked to a remote copy, with a name the researcher gives it.
- **Change**: One thing that happened to one record, exchanged between machines.
- **Conflict**: Two machines' incompatible changes to the same thing, awaiting a choice.
- **Remote File**: The stored content of a file a record refers to, with its fingerprint and
  the versions of it still needed.
- **Remote Backup**: A backup of a workspace kept at a connection.
- **Transfer**: One sending or receiving, recorded in the transfer log.
- **Exclusion**: A rule keeping kinds of record, tagged records, or particular records on
  the machine.
- **Passphrase**: A secret chosen by the researcher that makes what is sent unreadable at
  the service.
