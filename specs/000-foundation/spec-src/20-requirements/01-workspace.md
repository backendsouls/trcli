#### Workspace

- **FR-001**: Users MUST be able to create a workspace in a directory with a name and an
  optional description, and to view and change those details and the name under which
  their own actions are recorded.
- **FR-002**: Creating a workspace where one exists MUST be refused and MUST leave the
  existing workspace untouched.
- **FR-003**: The system MUST find the workspace to use, in this order: the one named for
  the command; the one named for the session; the nearest one at or above the current
  directory; the user's default workspace. It MUST say which workspace it used when that
  was the default.
- **FR-004**: When no workspace is found, every command that needs one MUST fail without
  creating or changing anything and MUST explain how to create or point to one.
- **FR-005**: Each workspace MUST keep its records separate from every other workspace, and
  MUST keep working when its directory is moved or renamed.
- **FR-006**: Viewing a workspace MUST show its name, description, location, format version,
  and the number of records of each kind.
- **FR-007**: The system MUST record the format version of every workspace. It MUST refuse
  to change a workspace whose format is newer than it understands. For an older format it
  MUST say an upgrade is needed, MUST NOT upgrade without being asked, MUST keep a copy of
  the workspace as it was before upgrading, and MUST leave the workspace unchanged if the
  upgrade fails.
- **FR-008**: When a workspace's stored data is damaged or invalid, the system MUST report
  what is wrong and where, and MUST NOT overwrite it.
- **FR-009**: Every change to a workspace MUST be made completely or not at all, including
  when the command is interrupted or the machine stops; two commands run at the same moment
  MUST NOT leave the workspace inconsistent.
