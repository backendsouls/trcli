#### Workspace sync

- **FR-024**: Users MUST be able to publish a workspace to a connection that offers sync,
  creating a remote copy linked to the workspace, and to obtain a published workspace on
  another machine as a complete local workspace linked to the same remote copy.
- **FR-025**: Users MUST be able to send local changes, receive remote changes, and do both
  in one step; after machines have exchanged all changes their workspaces MUST be
  identical.
- **FR-026**: The system MUST show sync status: changes waiting to be sent, whether there is
  something to receive, the time of the last sync, and open conflicts.
- **FR-027**: Changes made on different machines to different records, or to different
  details of the same record, MUST be combined without asking.
- **FR-028**: Changes made on different machines to the same detail of the same record, and
  a deletion on one machine with a change on another, MUST be presented as conflicts with
  both versions, their machines, and their times, and MUST NOT be resolved without the
  user's choice. No change may be lost silently.
- **FR-029**: Records that are only added to — audit entries, notebook entries, frozen
  pre-registrations, saved reports — MUST keep every entry from every machine unaltered, and
  the audit trail MUST remain a single verifiable history.
- **FR-030**: A workspace MUST remain fully usable without a network connection; sync MUST
  be able to happen later, however long the gap.
- **FR-031**: An interrupted transfer MUST leave both the local workspace and the remote
  copy valid and MUST be safely repeatable. The working storage of a local workspace MUST
  never itself be placed where a service modifies it.
- **FR-032**: When two machines send at the same moment, one MUST succeed and the other
  MUST be told to receive first; neither machine's changes may be lost.
- **FR-033**: Users MUST be able to be reminded of changes not yet sent, and to turn on
  automatic sync after commands that change the workspace; automatic sync MUST never
  resolve a conflict by itself and MUST never make a command fail.
- **FR-034**: Users MUST be able to unlink a workspace from its remote copy, leaving the
  local workspace complete, and separately to delete the remote copy after confirmation.
- **FR-035**: The system MUST refuse to sync with a remote copy written by a newer version
  than it understands, MUST upgrade a remote copy only when asked and after a backup, and
  MUST refuse to merge two workspaces that do not share a history.
- **FR-036**: The order in which changes are combined MUST NOT depend on machines' clocks
  alone, and the system MUST report a clock that is clearly wrong.
- **FR-037**: Each machine linked to a remote copy MUST be identifiable by a name the user
  can set, shown on conflicts and in the transfer log; users MUST be able to list linked
  machines and remove one's access.
