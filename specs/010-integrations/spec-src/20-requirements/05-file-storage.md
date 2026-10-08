#### File storage

- **FR-038**: Users MUST be able to send the files a record refers to to a connection that
  offers file storage, and to fetch them on another machine to the place the record
  expects.
- **FR-039**: Every sent and fetched file MUST be checked against its fingerprint; a file
  that does not match MUST NOT be put in place, and the mismatch MUST be reported.
- **FR-040**: The system MUST show, for each referenced file, whether it is local only,
  remote only, both and identical, or both and different.
- **FR-041**: Users MUST be able to send or fetch the files of one record, of all records of
  a kind, of a project, or matching a tag, and receive a summary.
- **FR-042**: Transfers MUST show progress, be interruptible, continue rather than restart
  when repeated, and transfer nothing when the two sides are already identical.
- **FR-043**: When a file that was sent has changed locally, sending it again MUST keep the
  earlier remote content retrievable for any dataset version, result, or manuscript version
  that recorded it.
- **FR-044**: Before sending, the system MUST check that enough space is available where the
  service reports it, and MUST say so before transferring anything when there is not.
- **FR-045**: A command that needs a file that is remote only MUST say that it must be
  fetched first and offer to fetch it.
- **FR-046**: Users MUST be able to free local space by removing files that have an
  identical remote copy, after confirmation; no other file may be removed.
- **FR-047**: Sending the files of a dataset flagged as personal or sensitive, or of a
  record marked private, MUST warn, require explicit confirmation, and record the
  acknowledgement.
