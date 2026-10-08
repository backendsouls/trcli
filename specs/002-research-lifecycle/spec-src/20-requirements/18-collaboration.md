#### Collaboration

- **FR-064**: A workspace owner MUST be able to add and remove members and give each a role:
  reader, editor, or administrator.
- **FR-065**: Members MUST be able to exchange changes so that their workspaces converge to
  the same content, including after working without a connection.
- **FR-066**: Every change MUST be attributed to the member who made it.
- **FR-067**: Conflicting changes MUST be shown with both values and their authors, and MUST
  NOT be resolved without a member's explicit choice; no change may be lost silently.
- **FR-068**: The system MUST enforce roles: readers MUST NOT change records, and removed
  members MUST NOT be able to exchange changes.
- **FR-069**: Append-only and frozen records MUST keep those properties across members, and
  the audit trail MUST present one ordered history for the whole workspace.
