### Edge Cases

- The researcher signs in with a different Google account than the one the workspace was
  published with: the tool notices that the remote copy is not there, says which account
  was used before, and changes nothing.
- The researcher deletes or renames TRCLI's folder in their Drive by hand: the next transfer
  reports that the remote copy is missing or moved, offers to publish again or to relink,
  and never recreates it silently.
- The researcher edits a file inside TRCLI's folder by hand: the tool detects that the
  remote content is not what it wrote and refuses to use it.
- The Drive is full: the transfer stops before or as soon as this is known, nothing local is
  affected, and the tool says how much space is needed.
- Access is withdrawn at Google while a transfer is running: the transfer stops, the remote
  copy stays valid, and the tool asks to connect again.
- The machine's clock is wrong: the order of changes does not depend on clocks alone, and
  the tool reports a clock that is clearly wrong.
- The same workspace is published twice from two machines that never synced: the tool
  refuses to merge two unrelated histories and explains the options.
- A machine is lost or stolen: from another machine the researcher disconnects that
  machine's access, and is told that the workspace content already on it cannot be recalled.
- Two workspaces are published to the same connection: each has its own remote copy, and
  listing shows both.
- A workspace is restored from an old backup and then synced: the tool treats the restored
  state as old and receives what is newer, after showing what will change.
- A file's name is not allowed by the service, or two files differ only by letter case: the
  tool stores them under safe names of its own and keeps the real names itself.
- A referenced file is a very large directory with many small files: it is transferred as a
  whole with one progress indication, and fetched as a whole.
- Sync is asked for while an experiment run is in progress: the run's state is sent as it
  is; another machine sees it as running elsewhere and cannot resume it.
- The remote copy was written by an older version of TRCLI: the tool upgrades it only when
  asked, after a backup, and other machines are told to upgrade before syncing.
- The network is slow or drops repeatedly: transfers resume where they stopped and never
  leave a half-written remote copy in use.
- A command is run by another program, without a person present, and sign-in is needed: it
  fails immediately with a message, and never waits for a browser.
- The passphrase is changed: content already at the service is protected again with the new
  one, or the tool says clearly that old content still needs the old passphrase.
- An integration is removed from a later version of the tool while connections to it exist:
  the connections are shown as unsupported, local data is untouched, and the tool says how
  to retrieve what is at the service.
