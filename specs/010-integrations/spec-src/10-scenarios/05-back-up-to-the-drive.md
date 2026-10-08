### User Story 5 - Back up to the Drive (Priority: P5)

A researcher keeps backups of the workspace somewhere other than their own disk: they
create a backup directly in their Drive, see the backups kept there, keep only the most
recent ones, and restore from one on any machine.

**Why this priority**: An off-machine backup is what protects years of work from a lost
laptop. It is simpler than sync and is useful even to someone who works on one machine, but
it builds on the backup already specified elsewhere.

**Independent Test**: Create a backup in the Drive, list remote backups, restore the
workspace from it on another machine into an empty location, and confirm it is identical.

**Acceptance Scenarios**:

1. **Given** a connection that offers a backup destination, **When** the researcher creates
   a backup there, **Then** the backup is made, verified, sent, and verified again after
   arrival.
2. **Given** remote backups, **When** the researcher lists them, **Then** each is shown with
   its date, size, the workspace it is of, and the version of the tool that made it.
3. **Given** a remote backup, **When** the researcher restores from it into an empty
   location, **Then** the workspace obtained is identical to the one backed up.
4. **Given** a number of backups to keep, **When** a new backup succeeds, **Then** the
   oldest beyond that number are removed remotely, and the tool says which.
5. **Given** a backup that fails to send or to verify, **When** it fails, **Then** no
   earlier backup is removed and the tool says the backup was not made.
6. **Given** a remote backup, **When** the researcher asks to verify it, **Then** it is
   checked without restoring.
7. **Given** a researcher who chose to protect backups with a passphrase, **When** a backup
   is made, **Then** it cannot be read in the Drive without the passphrase, and restoring
   asks for it.
8. **Given** a lost passphrase, **When** the researcher tries to restore, **Then** the tool
   says plainly that the backup cannot be recovered without it.
9. **Given** a backup that includes the files records refer to, **When** the researcher
   asks for it, **Then** those files are included, and without asking they are not.
10. **Given** a workspace that is also synced, **When** a backup is made, **Then** it is
    independent of sync and can be restored even if the remote copy is gone.

---
