### User Story 4 - Keep the files records refer to in the Drive (Priority: P4)

Records in a workspace refer to files — a dataset, a figure, a manuscript, a local copy of
a paper. A researcher sends chosen files to their Drive, so that on another machine the
record's file can be fetched when it is needed instead of being copied by hand. The tool
checks every transfer against the file's fingerprint.

**Why this priority**: Syncing records without their files leaves the second machine with
references to files that are not there. This completes sync, and matters most for data too
large to keep everywhere.

**Independent Test**: Send a dataset's files to the Drive from machine A, sync, fetch them
on machine B, and confirm the dataset verifies against its recorded fingerprint there.

**Acceptance Scenarios**:

1. **Given** a record with a local file and a connection that offers file storage, **When**
   the researcher sends the file, **Then** it is stored remotely and the record shows that
   a remote copy exists.
2. **Given** a record whose file exists remotely and not locally, **When** the researcher
   fetches it, **Then** the file is placed where the record expects it and checked against
   its fingerprint.
3. **Given** a fetched file that does not match its fingerprint, **When** the check fails,
   **Then** the file is not put in place, and the tool reports the mismatch.
4. **Given** records with files, **When** the researcher asks which files are where,
   **Then** each is shown as local only, remote only, both and identical, or both and
   different.
5. **Given** several records, **When** the researcher sends or fetches the files of all
   records of a kind, of a project, or matching a tag, **Then** all are transferred and a
   summary says what was done.
6. **Given** a large file, **When** it is transferred, **Then** progress is shown, the
   transfer can be interrupted, and repeating it continues rather than starts again.
7. **Given** a file already identical on the other side, **When** it is sent or fetched
   again, **Then** nothing is transferred.
8. **Given** a file that changed locally after being sent, **When** the researcher asks,
   **Then** it is shown as different, and sending it keeps the earlier remote version
   retrievable for the dataset version that used it.
9. **Given** a dataset flagged as holding personal or sensitive data, **When** the
   researcher sends its files, **Then** the tool warns, requires explicit confirmation, and
   records the acknowledgement.
10. **Given** a file to send larger than the space left in the Drive, **When** the
    researcher sends it, **Then** the tool says so before transferring anything.
11. **Given** a command that needs a file that is remote only (verifying a dataset, starting
    a run), **When** it runs, **Then** the tool says the file must be fetched first and
    offers to fetch it.
12. **Given** a record whose remote file was deleted in the Drive by hand, **When** the
    researcher fetches it, **Then** the tool reports it as missing remotely and changes
    nothing locally.
13. **Given** files, **When** the researcher frees local space for files that have an
    identical remote copy, **Then** only those are removed locally, after confirmation.

---
