#### Remote backup

- **FR-048**: Users MUST be able to create a backup of a workspace directly at a connection
  that offers a backup destination; the backup MUST be verified before sending and again
  after arrival.
- **FR-049**: Users MUST be able to list, verify, restore from, and delete remote backups;
  restoring into an empty location MUST yield a workspace identical to the one backed up.
- **FR-050**: Users MUST be able to set how many remote backups to keep; older ones MUST be
  removed only after a new backup has succeeded and been verified.
- **FR-051**: Remote backups MUST be independent of workspace sync and restorable when the
  synced remote copy no longer exists.
- **FR-052**: A backup MUST include the files records refer to only when the user asks.
