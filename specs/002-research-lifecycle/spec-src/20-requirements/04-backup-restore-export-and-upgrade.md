#### Backup, restore, export, and upgrade

- **FR-013**: Users MUST be able to produce a complete backup of a workspace as a single
  file, and the system MUST verify the backup before reporting success.
- **FR-014**: Users MUST be able to restore a backup to an empty location and obtain a
  workspace identical to the original; restoring over an existing workspace MUST require
  explicit choice.
- **FR-015**: The system MUST detect a damaged or incomplete backup and MUST change nothing
  when restoring from one.
- **FR-016**: Users MUST be able to export an entire workspace in an open, documented form
  that can be read without the tool.
- **FR-017**: The system MUST record the format version of every workspace, MUST detect
  when an upgrade is needed, MUST NOT upgrade without being asked, and MUST take a backup
  before upgrading.
- **FR-018**: A failed upgrade MUST leave the workspace as it was before the upgrade.
- **FR-019**: The system MUST refuse to modify a workspace whose format is newer than the
  tool understands.
