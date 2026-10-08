### User Story 17 - Collaborate on a shared workspace (Priority: P17)

Several researchers work on the same workspace, each on their own machine. They exchange
changes, see who changed what, resolve conflicting edits explicitly, and the workspace owner
decides who may read, edit, or administer it.

**Why this priority**: Collaboration changes a founding assumption of the base workspace —
one researcher, one machine — and touches every record type. It is the most valuable later
addition and the most costly, so it comes last and builds on everything else.

**Independent Test**: Two members change different records and exchange changes, ending
with identical workspaces; then both change the same field, and the conflict is shown and
resolved by choice, with nothing lost silently.

**Acceptance Scenarios**:

1. **Given** a workspace, **When** its owner invites a member with a role (reader, editor,
   administrator), **Then** the member can obtain a copy of the workspace.
2. **Given** two members with changes to different records, **When** they exchange changes,
   **Then** both workspaces end up identical and each change is attributed to its author.
3. **Given** two members who changed the same field of the same record, **When** they
   exchange changes, **Then** the conflict is shown with both values and their authors, and
   nothing is overwritten until one is chosen.
4. **Given** a member with the reader role, **When** they try to change a record, **Then**
   the change is refused.
5. **Given** a member whose access is removed, **When** they try to exchange changes,
   **Then** the exchange is refused.
6. **Given** a shared workspace, **When** any member views the audit trail, **Then** entries
   from all members appear in one ordered history.
7. **Given** a member working without a connection, **When** they reconnect and exchange
   changes, **Then** their offline work is merged under the same rules.
8. **Given** append-only records (notebook entries, audit entries, frozen
   pre-registrations), **When** members exchange changes, **Then** entries from all members
   are kept and none is altered.

---
