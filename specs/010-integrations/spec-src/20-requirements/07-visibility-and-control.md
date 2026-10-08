#### Visibility and control

- **FR-053**: Every command that sends or receives MUST offer a preview that lists what
  would be transferred and transfers nothing.
- **FR-054**: The system MUST keep a transfer log recording, for every transfer, the time,
  connection, machine, direction, kind of content, amount, and outcome; users MUST be able
  to filter and export it.
- **FR-055**: Users MUST be able to exclude kinds of record, records with given tags, and
  individual records from leaving the machine; sync, file storage, and remote backup MUST
  honour the exclusions and say what was left out, and the system MUST say which links will
  be incomplete on other machines.
- **FR-056**: Users MUST be able to protect everything sent through a connection with a
  passphrase, so that content and the names of records and files are unreadable at the
  service; a machine obtaining the workspace MUST ask for the passphrase and keep it in its
  protected store; the system MUST state plainly that content cannot be recovered without
  the passphrase.
- **FR-057**: Users MUST be able to see what is stored at a service for a connection, by
  kind and size, and to delete all of it after confirmation.
- **FR-058**: When a service asks the system to slow down or fails temporarily, the system
  MUST wait and retry a limited number of times, say what it is doing, and stop with a clear
  message when it cannot continue.
- **FR-059**: Telemetry MUST remain on the machine; no integration may cause it to be sent.
